pub mod kitchen;
pub mod inventory;

// We can also have utilities used across back_of_house modules
pub fn manager_status_report() {
    println!("Status report:");
    println!("All systems operational.");
    println!("3 new orders in the kitchen.");
    println!("2 tables available.");
    println!("Waitlist has 5 customers.");
    println!("Daily revenue: $1500.00");
    println!("Inventory: 20 steaks, 50 pastas, 30 desserts.");
    println!("Staff: 5 servers, 3 cooks, 1 manager.");
    println!("Customer feedback: 4.5/5 stars.");
    println!("End of report.");
}

pub fn notify_manager(message: &str) {
    println!("Manager Notification: {}", message);
}
