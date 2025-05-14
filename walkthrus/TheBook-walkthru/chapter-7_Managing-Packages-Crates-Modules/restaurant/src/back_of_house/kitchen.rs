use crate::back_of_house;

// Track orders being prepared
pub fn prepare_order(dish_name: &str, order_id: u32) {
    println!("Kitchen: Preparing {} for order #{}", dish_name, order_id);
    
    if dish_name.contains("Pasta") {
        cook_pasta(dish_name);
    } else if dish_name.contains("Steak") {
        cook_steak(dish_name);
    }
    
    // Notify manager about special orders
    if dish_name.contains("Special") {
        back_of_house::notify_manager(&format!("Special order #{}", order_id));
    }
}

// Check if an order is ready
pub fn is_order_ready(order_id: u32) -> bool {
    // In a real app, this would check a database or state
    println!("Kitchen: Checking status of order #{}", order_id);
    
    // For demonstration, we'll say even-numbered orders are ready
    order_id % 2 == 0
}

// Private function - internal kitchen process
fn cook_pasta(pasta_type: &str) {
    println!("Kitchen: Boiling water for {}", pasta_type);
    println!("Kitchen: Adding pasta to boiling water");
    println!("Kitchen: Preparing sauce");
    println!("Kitchen: Combining pasta and sauce");
}

// Private function - internal kitchen process
fn cook_steak(steak_type: &str) {
    println!("Kitchen: Heating the grill");
    println!("Kitchen: Seasoning the {}", steak_type);
    println!("Kitchen: Grilling to medium rare");
    println!("Kitchen: Letting it rest");
}