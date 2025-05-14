use crate::back_of_house::kitchen;
use crate::back_of_house::inventory;

pub fn take_order() {
    println!("Taking order from customer");
    // Send the order to the kitchen
    kitchen::prepare_order("Special", 12); // Order ID 12
}

pub fn serve_order() {
    println!("Checking if order is ready");
    // Check with kitchen if the order is ready
    if kitchen::is_order_ready(12) {
        println!("Serving the prepared order to customer");
    } else {
        println!("Order not ready yet, checking again soon");
    }
}

pub fn take_payment() {
    println!("Processing payment for the meal");
    // Update inventory after payment
    inventory::update_daily_revenue(25.99);
}
