fn main() {
    let greeting_file = File::open("hello.txt").unwrap(); // will return the file handle
                                                          // if the file exists. Otherwise,
                                                          // will call panic!
}
