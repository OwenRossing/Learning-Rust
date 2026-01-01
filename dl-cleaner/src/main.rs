use std::path::{Path, PathBuf};
use std::io::{self, Read};

fn main() {
    // 1) Get the path argument from CLI
    // 2) Convert it into a PathBuf
    // 3) Call print_info(&path)
    // 4) Handle errors and exit with a non-zero code on failure
}

fn get_path_arg() -> Result<PathBuf, String> {
    // Read std::env::args()
    // Ensure there is exactly one argument after the program name
    // Return it as PathBuf
    todo!()
}

fn print_info(path: &Path) -> Result<(), String> {
    // Print:
    // - Exists?
    // - Is file or directory?
    // - If file: size in bytes
    // Use std::fs::metadata(path)
    todo!()
}

fn format_kind(path: &Path) -> Result<&'static str, String> {
    // Determine if it's a file or directory (or neither)
    todo!()
}

fn file_size_bytes(path: &Path) -> Result<u64, String> {
    // Return metadata.len()
    todo!()
}
