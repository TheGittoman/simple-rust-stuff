mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

fn deliver_order() {}

mod customer {
    use crate::back_of_house;
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("wheat");
        meal.seasonal_fruit = String::from("blueberries");
        println!("I'd like {} toast please", meal.toast);
        hosting::add_to_waitlist();
    }
}
