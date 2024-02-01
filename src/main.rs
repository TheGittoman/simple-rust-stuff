// implementation of rust tutorial practise question

// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department
// in a company. For example, “Add Sally to Engineering”
// or “Add Amir to Sales.” Then let the user retrieve a
// list of all people in a department or all people in the
// company by department, sorted alphabetically.

use core::num;
use std::{
    collections::HashMap,
    io::{self, Write},
    result,
};

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    for _ in 0..3 {
        let mut result = String::new();
        print!("Add <name> to <department: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut result).expect("Error");

        result = match result.trim().parse() {
            Ok(string) => string,
            Err(_) => panic!(),
        };

        insert(&result, &mut employees);
    }
    io::stdout().flush().unwrap();

    print_employees(&employees);
    print_employees_by_department(&employees)
    // println!("{result}");
}

fn print_employees_by_department(employees: &HashMap<String, Vec<String>>) {
    let mut result: String = String::new();
    print!("Search for Department: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut result).expect("Error");

    result = match result.trim().parse() {
        Ok(string) => string,
        Err(_) => panic!(),
    };

    println!("in {} department there is:", result);
    let result = employees.get(&result).unwrap();
    for v in result {
        println!("                              {v}")
    }
}

fn print_employees(employees: &HashMap<String, Vec<String>>) {
    for (key, value) in employees {
        println!("in {} department there is:", key);
        for v in value {
            println!("                              {v}")
        }
    }
}

fn insert(input: &String, map: &mut HashMap<String, Vec<String>>) {
    if input.is_empty() {
        return;
    }
    let input: Vec<String> = input.split_whitespace().map(str::to_string).collect();
    if input.len() < 4 {
        return;
    }
    if input[0] == "Add".to_string() && input[2] == "To".to_string() {
        let mut values: Vec<String> = Vec::new();
        match map.get(&input[3]) {
            Some(value) => values = value.clone(),
            None => (),
        }
        values.push(input[1].clone());
        values.sort();
        map.insert(input[3].clone(), values);
    }
    //
}
