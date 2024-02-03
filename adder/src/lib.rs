#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, y: &Rectangle) -> bool {
        self.width > y.width && self.height > y.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_two_test() {
        let x = 8;
        assert_eq!(10, add_two(x));
    }

    #[test]
    fn larger_hold_smaller() {
        let larger = Rectangle {
            height: 30,
            width: 50,
        };
        let smaller = Rectangle {
            height: 20,
            width: 30,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_hold_larger() {
        let larger = Rectangle {
            height: 30,
            width: 50,
        };
        let smaller = Rectangle {
            height: 20,
            width: 30,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
