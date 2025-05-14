use crate::back_of_house::inventory;

pub fn add_to_waitlist() {
    println!("Adding customer to the waitlist");
    // Check with back_of_house inventory for table availability
    if inventory::check_table_availability() {
        println!("Tables are available, wait time is short");
    } else {
        println!("All tables occupied, estimated wait time: 30 minutes");
    }
}

pub fn seat_at_table() {
    println!("Seating customer at a table");
    // Mark a table as occupied in the inventory
    inventory::mark_table_occupied(5); // Table number 5
}