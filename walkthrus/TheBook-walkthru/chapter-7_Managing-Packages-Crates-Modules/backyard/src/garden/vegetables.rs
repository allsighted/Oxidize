// This file defines the vegetables module, which is a submodule of garden

// Note that we need to mark items as pub to make them accessible outside this module
#[derive(Debug)]
pub enum Vegetable {
    Carrot,
    Cucumber,
    Lettuce,
    Tomato,
}

// Public function that can be called from outside
pub fn grow_vegetable(vegetable: Vegetable) {
    // Call a private function from within the module
    let time = growing_time(&vegetable);
    println!("A {:?} takes {} days to grow.", vegetable, time);
}

// Private function - only accessible within this module
fn growing_time(vegetable: &Vegetable) -> u32 {
    // Private implementation detail
    match vegetable {
        Vegetable::Carrot => 60,
        Vegetable::Cucumber => 55,
        Vegetable::Lettuce => 45,
        Vegetable::Tomato => 80,
    }
}

// You can even have private submodules
mod plant_science {
    pub fn is_nightshade(plant: &super::Vegetable) -> bool {
        matches!(plant, super::Vegetable::Tomato)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_growing_time() {
        // We can access private functions in tests
        assert_eq!(growing_time(&Vegetable::Lettuce), 45);
    }
    
    #[test]
    fn test_is_nightshade() {
        // We can also access private submodules
        assert!(plant_science::is_nightshade(&Vegetable::Tomato));
        assert!(!plant_science::is_nightshade(&Vegetable::Carrot));
    }
}