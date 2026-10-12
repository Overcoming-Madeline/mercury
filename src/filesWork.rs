pub mod filesWork {
    use std::{
        fs::File,
        io::{self},
    };

    pub fn AllFilesInDirectory(Path: String) {
        let paths = std::fs::read_dir(String::from(Path)).unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }

    pub fn GetFileInDirectory(DirectoryPath: String, FileName: String) -> io::Result<File> {
        return File::open(format!("{}{}", DirectoryPath, FileName));
    }
}
