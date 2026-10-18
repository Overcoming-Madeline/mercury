pub mod files_work {
    use std::{
        fs::File, io::{self}, path::Path,
    };  

    use std::fs::metadata;

    use std::fs;
    pub fn get_files_in_directory(directory_path: String, file_name: String) -> io::Result<File> 
    {
        File::open(format!("{}{}", directory_path, file_name))
    }

    pub fn get_all_audio_files_in_directory(directory_path: String) -> io::Result<Vec<File>> {
        let md = metadata(directory_path.clone()).unwrap();
        
        let paths = fs::read_dir(".");

        if md.is_dir() {
            let paths = fs::read_dir(directory_path.clone());
        } else if md.is_file() {
            let file_path = Path::new(&directory_path);
            let paths = fs::read_dir(file_path.parent().unwrap().to_str().unwrap().to_string());
        } else {
            panic!("no files sound_file_result")
        }
        let vec_size = &paths.iter().clone().count();

        let mut vec = Vec::with_capacity(*vec_size);

        

        for path in paths.unwrap() {
                let file = path.unwrap().path();
                if file.is_dir() {
                    continue;
                }
                if file.extension().unwrap_or_default() == ".mp3" {
                    let pushfile = File::open(file).unwrap();
                    vec.push(pushfile)
                }
        }


        io::Result::Ok(vec)
    }

    pub fn get_files_in_directory_with_name(directory_path: String) -> io::Result<File> {
        File::open(&directory_path)
    }
}
