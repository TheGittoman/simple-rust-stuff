use std::{
    collections::HashMap,
    fmt::{self, format},
};
fn main() {
    let text = "This is a text about a bear whose name is bear";
    let map = count_words(text);

    for (str, value) in map {
        println!("word: {str} :used times: {value}");
    }
}

fn count_words(text: &str) -> HashMap<&str, i32> {
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // accessing the variable in count and adding one
        // let count ... assigns reference to the value in map
        // *count ... dereferences the value so we can access it directly
        *count += 1;
    }
    map
}

fn hashmap_func() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 50);
    scores.insert(String::from("red"), 70);
    scores.insert(String::from("red"), 100);
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(220);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn string_func() {
    let data = "initial contents";
    let mut s = String::from("String"); // equivalent to s3
    let mut s_empty = String::new();
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();
    let s4 = s.clone() + &s2;
    s.push_str("pushed data");
    s.push('1');
    let s = format!("{s2}-{s3}-{s4}");

    let s = String::from("hello");
    // let h = s[0]; // this doesn't work because rust doesn't support iterating over strings
    for c in "Hello world".chars() {
        print!("{c}");
    }
    println!();
    for c in "Hello world".as_bytes() {
        print!("{c}");
    }
}

fn vectors() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    for i in 1..5 {
        v.push(i);
    }
    for i in &mut v {
        *i += 50;
    }
    for v in &v {
        println!("{}", v)
    }
    let third: Option<&i32> = v.get(2);
    let first = &v[0];
    // v.push(6); // does not work since it is referencing to something used by "first"
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Integer: {value}"),
            SpreadsheetCell::Float(value) => println!("Float: {value}"),
            SpreadsheetCell::Text(value) => println!("Text: {value}"),
        }
    }
    println!("{}", &row[2]);
    println!("{:?}", &row.get(6));
    // println!("{}", &row.get(6));
    string_func();
}

impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpreadsheetCell::Int(value) => write!(f, "{value}"),
            SpreadsheetCell::Float(value) => write!(f, "{value}"),
            SpreadsheetCell::Text(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
