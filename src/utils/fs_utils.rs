use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn create_directory<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn create_file_with_content<P: AsRef<Path>, C: AsRef<[u8]>>(
    path: P,
    content: C,
) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_ref())
}
