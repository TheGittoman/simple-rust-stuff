fn main() {
    // calculate the size of a rectangle using structs
    let scale = 2;
    let rectangle = Rectangle {
        dim_x: dbg!(20 * scale), // dbg! lets us see what the code is doing
        dim_y: 36,
    };
    let rectangle2 = Rectangle {
        dim_x: 15, // dbg! lets us see what the code is doing
        dim_y: 30,
    };
    let square = Rectangle::square(40);
    if rectangle.check() {
        println!("Area of the rectangle: {}", rectangle.area());
        println!(
            "is rectangle bigger that rectangle2: {}",
            rectangle.is_bigger(&rectangle2)
        );
        println!(
            "can rectangle hold rectangle2: {}",
            rectangle.can_hold(&rectangle2)
        );
    } else {
        print!("Rectangle had invalid dimensions!");
    }

    dbg!(&rectangle); // dbg! shows the whole struct definition here
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            dim_x: size,
            dim_y: size,
        }
    }
    fn area(&self) -> u32 {
        self.dim_x * self.dim_y
    }
    fn is_bigger(&self, other: &Rectangle) -> bool {
        if self.area() > other.area() {
            true
        } else {
            false
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.dim_x > other.dim_x && self.dim_y > other.dim_y
    }
    fn check(&self) -> bool {
        self.dim_x > 0 && self.dim_y > 0
    }
}

// decorated with this to allow {:?} and dbg!() to print the whole struct
#[derive(Debug)]
struct Rectangle {
    dim_x: u32,
    dim_y: u32,
}
