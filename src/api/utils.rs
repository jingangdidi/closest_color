use std::fs::read_to_string;

use crate::error::MyError;

/// 读取文件每行，存储到Vec
pub fn read_lines_to_vec(filename: &str) -> Result<Vec<String>, MyError> {
    let content = read_to_string(filename).map_err(|e| MyError::ReadFileError{file: filename.to_string(), error: e})?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}
