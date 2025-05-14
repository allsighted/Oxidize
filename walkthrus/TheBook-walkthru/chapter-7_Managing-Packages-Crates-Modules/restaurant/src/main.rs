// use restaurant::front_of_house::hosting;
// use restaurant::front_of_house::serving;
use restaurant::front_of_house::{
    hosting, 
    serving 
};
use restaurant::back_of_house::manager_status_report;

fn main() {
    // Simulate a customer visit
    println!("\n--- New Customer Arrives ---\n");
    
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
    
    manager_status_report();
    
    println!("\n--- Customer Visit Complete ---\n");
}