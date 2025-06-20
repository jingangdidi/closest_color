use closest_color::{
    parse_paras::parse_para,
    error::MyError,
    color::find_closest_color,
};

fn main() {
    if let Err(e) = run() {
        println!("{}", e); // 这里不要用`{:?}`，会打印结构体而不是打印指定的错误信息
    }
}

fn run() -> Result<(), MyError> {
    // 解析参数
    let paras = parse_para()?;

    // 寻找最接近的颜色
    find_closest_color(paras.color, paras.candidate, paras.num, paras.algorithm)
}
