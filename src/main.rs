// generics allow us to replace specific types with a
// placeholder that represents multiple types without
// code dublication

fn main() {
    let number_list = vec![10, 20, 30, 40];
    let char_list = vec!['a', 'v', 'b', 'i', 'c'];

    let point = Point { x: 5, y: 10 };
    let pointf = Point { x: 5.5, y: 10.5 };

    let largest = find_largest(&number_list);
    let largest2 = find_largest_generic(&char_list);

    println!("The largest number is {} - {}", largest, largest2);
}

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// some T values doesn't work with compare operator >
// std::cmp::partialOrd is used here since it allows
// T to be only char or int
fn find_largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// T can only be a single type when defining a struct Point
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// using T & U allows us to use eq. int and float when defining a Point_
struct Point_<T, U> {
    x: T,
    y: U,
}
