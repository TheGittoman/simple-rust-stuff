use std::collections::HashMap;

// implementation of rust tutorial practise question

// Given a list of integers, use a vector and
// return the median (when sorted, the value in
// the middle position) and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

fn main() {
    let mut numbers = vec![1, 2, 3, 5, 4, 5, 1, 2, 3, 4, 5];
    let result = median(&mut numbers);
    println!("Median of the list is: {result}");
    // println!("{:?}", numbers);
    println!("Mode of the list is: {}", mode(numbers));
}

fn median(list: &mut Vec<i32>) -> i32 {
    // given a list of integers return median
    if list.is_empty() {
        return 0;
    }
    // println!("{:?}", list);
    list.sort();
    // println!("{:?}", list);
    match list.get(list.len() / 2) {
        Some(value) => return *value,
        None => return 0,
    }
}

fn mode(list: Vec<i32>) -> i32 {
    //given a list of integers return mode of the list
    if list.is_empty() {
        return 0;
    }

    // calculate hashmap for all the values to get frequency of the numbers
    let mut map: HashMap<i32, i32> = HashMap::new();
    for value in list {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    // get biggest value from the map values that include the frequency
    let mut max_key = 0;
    let mut biggest = 0;
    for (key, value) in &map {
        if value > &biggest {
            biggest = *value;
            max_key = *key;
        }
    }
    // println!("{:?}", map);
    max_key
    // if let Some(value) = map.get(&max_key) {
    //     return *value;
    // } else {
    // 0
    // }
}
