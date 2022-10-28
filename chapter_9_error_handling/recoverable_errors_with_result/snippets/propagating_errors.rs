use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt"); // opens the file

    let mut username_file = match username_file_result {
        Ok(file) => file,        // file exists, give it to username_file
        Err(e) => return Err(e), // some error has occurred. Propagate the error to the calling code
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        // read the file's contents
        Ok(_) => Ok(username), // contents are now read, give it to `username`
        Err(e) => Err(e), // failed to read the file's contents, return the error to the calling code
    }
}
