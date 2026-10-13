pub mod files_work {
    use std::{
        fmt::format, fs::File, io::{self},
    };

    pub fn all_files_in_directory(Path: String) {
        let paths = std::fs::read_dir(Path).unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }

    pub fn get_files_in_directory(directory_path: String, file_name: String) -> io::Result<File> {
        File::open(format!("{}{}", directory_path, file_name))
    }

    pub fn get_files_in_directory_with_name(directory_path: String) -> io::Result<File>{
        File::open(format!("{}",directory_path))
    }
}
