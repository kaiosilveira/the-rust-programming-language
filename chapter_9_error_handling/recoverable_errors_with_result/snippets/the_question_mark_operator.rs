use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // give the file handle to `username_file` if the operation succeeds, otherwise propagates the error
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // give the file contents string to username, otherwise, propagates the error
    Ok(username)
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username); // chaining a conditional execution
    Ok(username)
}

fn shortest_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt"); // using fs::read_to_string utility does all the above
}
