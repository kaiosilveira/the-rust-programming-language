use std::collections::HashMap;

// Given a list of integers, use a vector and return the median (when sorted, the value
// in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn main() {
    let integers = vec![1, 2, 2, 3, 3, 3, 4, 5, 6, 7];

    let mode = find_mode(&integers);
    println!("mode: {}", mode);

    let median = find_median(&integers);
    println!("median: {}", median);
}

fn find_median(arr: &Vec<i32>) -> i32 {
    let mut unique_arr: Vec<&i32> = Vec::new();
    for i in arr {
        if !unique_arr.contains(&i) {
            unique_arr.push(i);
        }
    }

    *unique_arr[unique_arr.len() / 2]
}

fn find_mode(arr: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for number in arr {
        let count: &mut i32 = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut mode: i32 = 0;
    let mut current_value = 0;
    for (key, value) in map {
        if value > current_value {
            current_value = value;
            mode = *key;
        }
    }

    mode
}
