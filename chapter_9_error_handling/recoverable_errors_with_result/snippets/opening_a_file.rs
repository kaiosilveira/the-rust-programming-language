use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
      Ok(file) => file, // file is now open, return a handle to it
      Err(error) => println!("Failed to open the file. Error: {:?}", error) // failed to open the file
    }
}
