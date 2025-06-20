use palette::{
    Srgb,
    Lab,
    FromColor,
    color_difference::{ // 相似性算法：https://docs.rs/palette/0.7.6/palette/color_difference/index.html
        Ciede2000, // Calculate the CIEDE2000 ΔE* (Delta E) color difference between two colors
        DeltaE, // Calculate the ΔE color difference between two colors.
        EuclideanDistance, // Calculate the distance between two colors as if they were coordinates in Euclidean space.
    },
};

use crate::{
    error::MyError,
    parse_paras::RawColor,
};

/// 从待选颜色列表中，找出与指定颜色最接近的颜色
/// rgb颜色可以是u8或f32，Lab::from_color需要Srgb::<f32>，这里都统一用f32
pub fn find_closest_color(colors: Vec<RawColor>, candidate: Vec<(Option<String>, RawColor)>, num: usize, algorithm: u8) -> Result<(), MyError> {
    // 先把所有候选颜色转为Lab颜色
    let mut candidate_colors: Vec<(Option<String>, RawColor, Lab)> = vec![];
    for c in candidate {
        let rgb_color: Srgb::<f32> = match c.1 {
            RawColor::RgbU8(ref color_u8) => Srgb::<u8>::from(color_u8.1).into(), // Srgb::<u8>::new(color_u8[0], color_u8[1], color_u8[2]).into()
            RawColor::RgbF32(ref color_f32) => Srgb::<f32>::from(color_f32.1), // Srgb::<f32>::new(color_f32[0], color_f32[1], color_f32[2])
            RawColor::Hex(ref color_hex) => color_hex.parse::<Srgb::<u8>>().map_err(|e| MyError::ParseHexColorError{hex: color_hex.clone(), error: e})?.into(),
        };
        candidate_colors.push((c.0, c.1, Lab::from_color(rgb_color)));
    }

    // 遍历指定的每个颜色，与每个候选颜色进行比较
    for c in colors {
        let mut color_score: Vec<(Option<String>, String, f32)> = Vec::with_capacity(candidate_colors.len()); // (颜色名, 原始颜色值字符串, 相似度)，相似度score越小越好
        let color_lab = Lab::from_color(match c {
            RawColor::RgbU8(ref color_u8) => Srgb::<u8>::from(color_u8.1).into(), // Srgb::<u8>::new(color_u8[0], color_u8[1], color_u8[2]).into()
            RawColor::RgbF32(ref color_f32) => Srgb::<f32>::from(color_f32.1), // Srgb::<f32>::new(color_f32[0], color_f32[1], color_f32[2])
            RawColor::Hex(ref color_hex) => color_hex.parse::<Srgb::<u8>>().map_err(|e| MyError::ParseHexColorError{hex: color_hex.clone(), error: e})?.into(),
        });
        for cand in &candidate_colors {
            color_score.push((
                cand.0.clone(), // 颜色名
                cand.1.get_raw_color(), // 原始颜色值字符串
                match algorithm { // 相似度
                    1 => color_lab.difference(cand.2), // Ciede2000
                    2 => color_lab.delta_e(cand.2), // DeltaE
                    3 => color_lab.distance(cand.2), // EuclideanDistance
                    _ => unreachable!(),
                },
            ));
        }
        // 根据score由小到大排序，取前指定个最相近的颜色
        color_score.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        println!("{} {} closest colors: {}", c.get_raw_color(), num, color_score.iter().take(num).map(|s| {
            match &s.0 {
                Some(name) => format!("{}({}, {})", s.1, name, s.2),
                None => format!("{}({})", s.1, s.2),
            }
        }).collect::<Vec<_>>().join(", "));
    }
    Ok(())
}
