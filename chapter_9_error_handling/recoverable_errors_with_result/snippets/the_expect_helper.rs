fn main() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    // will return the file handle if the file exists. Otherwise, will call panic!
}
