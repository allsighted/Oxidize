// This file defines the garden module
// We need to declare the vegetables submodule
pub mod vegetables; // This tells Rust to look for the vegetables module in vegetables.rs or vegetables/mod.rs

// Functions in the garden module
pub fn plant_count() -> i32 {
    // Returns the number of plants in the garden
    8
}

// Another function in the garden module
pub fn garden_info() -> String {
    // We can also access items from the vegetables submodule
    let veg = vegetables::Vegetable::Lettuce;
    format!("This garden grows food like {:?}!", veg)
}

#[cfg(test)]
mod tests {
    // Note how we need to bring the parent's items into scope with super
    use super::*;
    
    #[test]
    fn test_garden_info() {
        assert!(garden_info().contains("Lettuce"));
    }
}