mod front_of_house;

mod back_of_house {
    // All fields are public or private along with the enum itself
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Fields have to be marked pub as well to be used publicly
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
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
        super::deliver_order(); // Child scope can access anything in parent
    }

    fn cook_order() {}
}

fn deliver_order() { }

// Note how the front_of_house mod is still private.
// eat_at_restaurant can access its public children because it is
// defined at the same scope.
pub fn eat_at_restaurant() {
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // rel path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("id like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blackberries");
}
