fn main() {
    let i: i8 = 127;
    let u: i8 = 255;
    let x = 2.0; // f64
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // results in 0

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // chars
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup;
    tup.0;
    tup.1;
    tup.2;

    // arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_with_same_num = [3; 5]; // -> [3, 3, 3, 3, 3]
    arr[0];
    arr[4];
}
