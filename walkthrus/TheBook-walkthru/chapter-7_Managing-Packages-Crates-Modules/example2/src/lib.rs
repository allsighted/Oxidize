mod back_of_house;
use crate::back_of_house::kitchen;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;

// can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
pub use crate::front_of_house::hosting;


mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house::kitchen;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }

    pub fn order() {
        // Absolute path
        crate::back_of_house::kitchen::cook_order();
        kitchen::cook_order();
    }
}
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }
// ! hosting::add_to_waitlist();
// ! ^^^^^^^ use of undeclared crate or module `hosting`

pub fn eat_at_restaurant() {
    // hosting::add_to_waitlist();
    customer::eat_at_restaurant();
    customer::order();
}

/// ---

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
