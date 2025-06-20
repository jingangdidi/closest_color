use std::path::Path;

use argh::FromArgs;

/// error: 定义的错误类型，用于错误传递
use crate::{
    error::MyError,
    utils::read_lines_to_vec,
};

/// R语言内置的颜色，使用`colors()`获取
const R_COLORS: &str = include_str!("../../R_colors.txt");

#[derive(FromArgs)]
/// find closest color
struct Paras {
    /// colors, can be a file(one color per line) or colon separated multiple colors, support rgb(0-255 or 0~1, e.g. 171,193,35:0.3,0.8,0.1) and hex(with or without the "#", shorthand of three letters format, e.g. #f034e6:f034e6:f3e)
    #[argh(option, short = 'c')]
    color: String,

    /// candidate colors, can be a file or colon separated colors, default 657 R colors
    #[argh(option, short = 'd')]
    candidate: Option<String>,

    /// number of closest colors, default: 1
    #[argh(option, short = 'n')]
    num: Option<usize>,

    /// distance/difference algorithm, 1(Ciede2000), 2(DeltaE), 3(EuclideanDistance), default: 1
    #[argh(option, short = 'a')]
    algorithm: Option<u8>,
}

/// 原始颜色值
#[derive(Debug)]
pub enum RawColor {
    RgbU8((String, [u8; 3])),   // (原始颜色字符串, 提取的颜色值)，值范围0-255，例如：171,193,35
    RgbF32((String, [f32; 3])), // (原始颜色字符串, 提取的颜色值)，值范围0-1，例如：0.3,0.8,0.1
    Hex(String),                // 原始颜色字符串，可以省略“#”前缀，也可以使用3字母简写，例如：#f034e6、f034e6、f3e
}

impl RawColor {
    /// 根据颜色值字符串创建RawColor
    fn new(raw: String) -> Result<RawColor, MyError> {
        if raw.contains(",") { // rgb颜色
            let tmp_rgb: Vec<&str> = raw.split(",").collect();
            if tmp_rgb.len() == 3 {
                if raw.contains(".") { // 值范围0-1
                    Ok(RawColor::RgbF32(
                        (
                            raw.clone(),
                            tmp_rgb.iter().map(|s| Ok(s.parse::<f32>().map_err(|e| MyError::ParaError{para: format!("parse {} to f32 error: {:?}", s, e)})?)).collect::<Result<Vec<f32>, MyError>>()?.try_into().unwrap(),
                        )
                    ))
                } else { // 值范围0-255
                    Ok(RawColor::RgbU8(
                        (
                            raw.clone(),
                            tmp_rgb.iter().map(|s| Ok(s.parse::<u8>().map_err(|e| MyError::ParaError{para: format!("parse {} to u8 error: {:?}", s, e)})?)).collect::<Result<Vec<u8>, MyError>>()?.try_into().unwrap(),
                        )
                    ))
                }
            } else {
                return Err(MyError::ParaError{para: format!("rgb color must contain 3 \",\": {}", raw)})
            }
        } else { // hex颜色
            if raw.starts_with("#") {
                if raw.len() == 4 || raw.len() == 7 {
                    Ok(RawColor::Hex(raw))
                } else {
                    return Err(MyError::ParaError{para: format!("hex color starts with \"#\" length must be 4 or 7, not {}", raw)})
                }
            } else {
                if raw.len() == 3 || raw.len() == 6 {
                    Ok(RawColor::Hex(raw))
                } else {
                    return Err(MyError::ParaError{para: format!("hex color not starts with \"#\" length must be 3 or 6, not {}", raw)})
                }
            }
        }
    }

    /// 获取原始颜色字符串
    pub fn get_raw_color(&self) -> String {
        match self {
            RawColor::RgbU8(s)  => s.0.clone(), // (原始颜色字符串, 提取的颜色值)，值范围0-255，例如：171,193,35
            RawColor::RgbF32(s) => s.0.clone(), // (原始颜色字符串, 提取的颜色值)，值范围0-1，例如：0.3,0.8,0.1
            RawColor::Hex(s)    => s.clone(),   // 原始颜色字符串，可以省略“#”前缀，也可以使用3字母简写，例如：#f034e6、f034e6、f3e
        }
    }
}

/// 存储解析后的命令行参数
///#[derive(Debug, Default)]
pub struct ParsedParas {
    pub color:     Vec<RawColor>,                   // 指定要比较的颜色，可以是写有颜色的文件（每行一个颜色），也可以是冒号间隔的多个颜色，支持rgb颜色（可以是0-1或0-255，例如“171,193,35:0.3,0.8,0.1”）和hex颜色（可以省略“#”前缀，也可以使用3字母简写，例如“#f034e6:f034e6:f3e”）
    pub candidate: Vec<(Option<String>, RawColor)>, // (待选颜色名, 待选颜色值)，可以是写有颜色的文件（每行一个颜色），也可以是冒号间隔的多个颜色，默认R的657个颜色
    pub num:       usize,                           // 获取最相近的几个颜色， 默认1
    pub algorithm: u8,                              // 相似性算法，1(Ciede2000)、2(DeltaE)、3(EuclideanDistance)，默认1
}

/// 解析参数
pub fn parse_para() -> Result<ParsedParas, MyError> {
    let para: Paras = argh::from_env();
    let out: ParsedParas = ParsedParas{
        color: { // 指定要比较的颜色，可以是写有颜色的文件（每行一个颜色），也可以是冒号间隔的多个颜色，支持rgb颜色（可以是0-1或0-255，例如“171,193,35:0.3,0.8,0.1”）和hex颜色（可以省略“#”前缀，也可以使用3字母简写，例如“#f034e6:f034e6:f3e”）
            let tmp_colors: Vec<String> = {
                let tmp_path = Path::new(&para.color);
                if tmp_path.exists() && tmp_path.is_file() {
                    read_lines_to_vec(&para.color)?
                } else {
                    para.color.split(":").map(|s| s.to_string()).collect()
                }
            };
            let mut colors: Vec<RawColor> = Vec::with_capacity(tmp_colors.len());
            for c in tmp_colors {
                colors.push(RawColor::new(c)?);
            }
            colors
        },
        candidate: match para.candidate { // (待选颜色名, 待选颜色值)，可以是写有颜色的文件（每行一个颜色），也可以是冒号间隔的多个颜色
            Some(c) => {
                let tmp_colors: Vec<String> = {
                    let tmp_path = Path::new(&c);
                    if tmp_path.exists() && tmp_path.is_file() {
                        read_lines_to_vec(&c)?
                    } else {
                        c.split(":").map(|s| s.to_string()).collect()
                    }
                };
                let mut colors: Vec<(Option<String>, RawColor)> = Vec::with_capacity(tmp_colors.len());
                for i in tmp_colors {
                    colors.push((None, RawColor::new(i)?));
                }
                colors
            },
            None => { // 使用R的657个颜色
                let mut colors: Vec<(Option<String>, RawColor)> = vec![];
                for line in R_COLORS.lines() {
                    let tmp_name_color: Vec<&str> = line.split("\t").collect();
                    if tmp_name_color.len() >= 2 { // 第1列颜色名，第2列颜色值，胡烈其他列
                        colors.push((Some(tmp_name_color[0].to_string()), RawColor::new(tmp_name_color[1].to_string())?));
                    } else { // 第1列视为颜色值
                        colors.push((None, RawColor::new(tmp_name_color[0].to_string())?));
                    }
                }
                colors
            },
        },
        num: match para.num { // 获取最相近的几个颜色， 默认1
            Some(n) => n,
            None => 1,
        },
        algorithm: match para.algorithm { // 相似性算法，1(Ciede2000)、2(DeltaE)、3(EuclideanDistance)，默认1
            Some(a) => {
                if a == 0 || a > 3 {
                    return Err(MyError::ParaError{para: format!("-a only support 1(Ciede2000), 2(DeltaE), 3(EuclideanDistance), not {}", a)})
                }
                a
            },
            None => 1,
        },
    };
    // 检查n是否越界
    if out.num == 0 {
        return Err(MyError::ParaError{para: "-n must > 0".to_string()})
    } else if out.num > out.candidate.len() {
        return Err(MyError::ParaError{para: format!("-n ({}) must <= candidate color number ({})", out.num, out.candidate.len())})
    }
    Ok(out)
}
