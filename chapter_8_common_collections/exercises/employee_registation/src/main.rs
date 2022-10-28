// Using a hash map and vectors, create a text interface to allow a user to add employee
// names to a department in a company. For example, “Add Sally to Engineering” or
// “Add Amir to Sales.” Then let the user retrieve a list of all people in a
// department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
    print_banner();
    print_instructions();

    let mut employee_dept_map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to process the input. Please try again.");

        if user_input.trim() == "q!" {
            println!("Bye!");
            break;
        } else {
            let (employee_name, employee_department) =
                get_sentence_parts_from_user_input(&user_input);

            employee_dept_map
                .entry(employee_department)
                .or_insert(Vec::new())
                .push(employee_name);

            println!("Updated registry: {:?}", employee_dept_map);
        }
    }
}

fn get_sentence_parts_from_user_input(user_input: &str) -> (String, String) {
    let mut sentence_parts: Vec<String> = Vec::new();
    for word in user_input.split_whitespace() {
        sentence_parts.push(String::from(word));
    }

    (
        String::from(&sentence_parts[1]),
        String::from(&sentence_parts[3]),
    )
}

fn print_banner() {
    println!("+~~~~~~~~~~~~~~~~~~~~~~~+");
    println!("| Employee Registration |");
    println!("+~~~~~~~~~~~~~~~~~~~~~~~+");
}

fn print_instructions() {
    println!("To add a new employee, type the following text pattern: add [employee_name] to [department].");
    println!("Replacing [employee_name] by the actual employee name and [department] by the actual department");
}
