// The main.rs file is the entry point for the binary crate
// We need to bring the garden module into scope
mod garden; // This declares that we're using a module defined in garden.rs or garden/mod.rs

fn main() {
    // Access a function from the garden module
    println!("I have {} plants in my garden.", garden::plant_count());
    
    // Access the Vegetable enum from the vegetables module (which is a submodule of garden)
    let carrot = garden::vegetables::Vegetable::Carrot;
    println!("I'm growing a {:?} in my garden.", carrot);
    
    // We can also use the vegetables module directly with the use keyword
    use garden::vegetables::Vegetable;
    let tomato = Vegetable::Tomato;
    println!("I'm also growing a {:?}.", tomato);
    
    // Demonstrate growing a vegetable
    garden::vegetables::grow_vegetable(Vegetable::Cucumber);
}