use core::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ReadFileError {
    IoError(io::Error),
    ParseError(FromUtf8Error),
}

impl fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadFileError::IoError(err) => write!(f, "I/O Error: {}", err),
            ReadFileError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

pub fn read_from_file(file_path: &Path) -> Result<String, ReadFileError> {
    let bytes = fs::read(file_path).map_err(ReadFileError::IoError)?;

    let content = String::from_utf8(bytes).map_err(ReadFileError::ParseError)?;

    Ok(content)
}

pub fn write_to_file(file_path: &Path, data: impl AsRef<[u8]>) {
    let parent_dir = file_path.parent().unwrap();

    if !Path::new(file_path).exists() {
        fs::create_dir_all(parent_dir).unwrap_or_else(|err| {
            eprintln!(
                "Failed to create directory '{}': {}",
                parent_dir.display(),
                err
            );
            std::process::exit(1);
        });
    }

    if let Err(err) = fs::write(file_path, data) {
        eprintln!("Failed to write file '{}': {}", file_path.display(), err);
        std::process::exit(1);
    }
}
