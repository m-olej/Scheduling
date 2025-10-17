use std::fs;
use std::path::Path;

pub fn read_from_file(file_path: &Path) -> Vec<u8> {
    fs::read(&file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read file '{}': {}", file_path.display(), err);
        std::process::exit(1);
    })
}

pub fn write_to_file(file_path: &Path, data: &[u8]) {
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

    fs::write(&file_path, &data).unwrap_or_else(|err| {
        eprintln!("Failed to write file '{}': {}", file_path.display(), err);
        std::process::exit(1);
    });
}
