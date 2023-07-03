use std::{fs::{read, read_dir, write, self}, path::Path};
use crate::debug;

// Reads a file from a given path.
#[allow(dead_code)]
pub fn try_read_file_to_bytes(path: String) -> Result<Vec<u8>, String> {

    debug!("Reading file {}", path);

    match read(path) {
        Ok(vec_content) => { Ok(vec_content) }
        Err(reason) => { Err(reason.to_string()) } 
    }
}

// Read every file from a directory.
#[allow(dead_code)]
pub fn try_read_files_from_dir_to_bytes(path: String) 
    -> Result<Vec<Vec<u8>>, String> {

    debug!("Reading files from directory {}", path);

    match read_dir(path) {
        Ok(dir_content) => {
            let mut output_files = vec![];
            dir_content.for_each(|file| {
                let file = file.unwrap();

                // if the file is a folder, the entry is not processed.
                if file.metadata().unwrap().is_file() { 

                    debug!("{:?}", file);

                    match try_read_file_to_bytes(
                        format!("{}", file.path().display())) {
                        Ok(content) => { output_files.push(content); }
                        Err(reason) => { panic!("{}", reason.to_string()); }
                    }
                }
            });
            Ok(output_files)
        } 
        Err(reason) => { Err(reason.to_string()) }
    }
}

// Write data into a file.
#[allow(dead_code)]
pub fn try_write_bytes_to_file(path: String, content: &[u8]) 
    -> Result<(), String> {
    if let Err(reason) = try_make_path(path.clone(), true) {
        return Err(reason);
    }
    
    match write(path, content) {
        Ok(()) => { Ok(()) }
        Err(reason) => { Err(reason.to_string()) }
    }
}

// Ensure every directory on a path exists, creates it otherwise.
// `last_item_is_a_file` determines if the last item is a file or not, thus
// creating only the directories before.
#[allow(dead_code)]
pub fn try_make_path(path: String, last_item_is_a_file: bool) 
    -> Result<(), String> {
    let mut arg_path = path.replace("/", "\\");
    let mut path = Path::new(&arg_path); 

    if last_item_is_a_file {
        // Break down the path into a set of path elements.
        let mut arg_path_cpy = arg_path.split("\\").collect::<Vec<&str>>();

        // Remove the last element and rebuilds the path.
        arg_path_cpy.pop();
        arg_path = arg_path_cpy.join("\\");

        // Build the new path without the last item, supposed to be a file.
        path = Path::new(&arg_path);
    }

    match fs::create_dir_all(path) {
        Ok(()) => { Ok(()) }
        Err(reason) => { Err(reason.to_string()) }
    }
}
