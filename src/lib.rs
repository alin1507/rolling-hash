pub mod rolling_hash;

use derive_more::Display;
use rolling_hash::rolling::{Block, CHUNK_SIZE};
use std::fs;
use std::io::{stdin, stdout, Write};

use crate::rolling_hash::rolling::Rolling;

//POSSIBLE ERRORS
#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "The file at '{path}' was not found")]
    FileNotFound { path: String },
    #[display(fmt = "The file at '{path}' is too small to be separate in two chunks of data")]
    FileTooSmall { path: String },
}

pub fn run() -> Result<Vec<Block>, AppError> {
    let mut original_path = String::new();
    let mut updated_path = String::new();

    //GET THE PATH FOR THE ORIGINAL FILE
    print!("Original file path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut original_path)
        .expect("Error when reading the line");
    original_path = correct_path(original_path);

    //GET THE PATH FOR THE UPDATED FILE
    print!("Updated file path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut updated_path)
        .expect("Error when reading the line");
    updated_path = correct_path(updated_path);

    //GET THE CONTENT FOR BOTH FILES
    let v1 = get_file_content(original_path)?;
    let v2 = get_file_content(updated_path)?;
    let v2_bytes = v2.as_bytes();

    //SEND CONTENT TO CREATE SIGNATURES AND DELTA
    let rolling = Rolling::new(v1, v2_bytes);

    //RETURN DELTA
    Ok(rolling.delta)
}

//REMOVE NEW LINE CHARACTERS
pub fn correct_path(mut path: String) -> String {
    if let Some('\n') = path.chars().next_back() {
        path.pop();
    }
    if let Some('\r') = path.chars().next_back() {
        path.pop();
    }
    path
}

//GET THE CONTENT FROM FILE WITH SPECIFIED PATH
pub fn get_file_content(path: String) -> Result<String, AppError> {
    //CHECK IF THE PATH IS CORRECT AND SAVE CONTENT
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => {
            return Err(AppError::FileNotFound { path });
        }
    };

    //CHECK IF THE CONTENT IS BIGGER THAN THE CHUNK SIZE TO BE ABLE TO CREATE AT LEAST 2 CHUNKS OF DATA
    if content.len() < (CHUNK_SIZE + 1) as usize {
        return Err(AppError::FileTooSmall { path });
    }

    //RETURN THE CONTENT OF THE FILE
    Ok(content)
}
