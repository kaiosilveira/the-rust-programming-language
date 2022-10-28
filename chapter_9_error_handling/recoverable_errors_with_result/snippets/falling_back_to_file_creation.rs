use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file_result = File::open("hello.txt");
  let greeting_file = match greeting_file_result {
    Ok(file) => file, // file exists, return it
    Err(error) => match error.errorKind() { // failed to open the file
      ErrorKind::NotFound => match File::create("hello.txt") { // file was not found, create it
        Ok(fc) => fc, // return the file that was just created
        Err(e) => panic!("Problem creating the file: {:?}", e) // failed to create the file, panic
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error); // some other error, panic
      }
    }
  }
}
