pub use std::ops::Add;
#[derive(Debug)]
pub enum OrderStatus {
    InProgress,
    Sent,
    Delivered,
    Disputed(String),
}

impl OrderStatus {
    pub fn get_order_status_in_string(&self) -> String {
        match self {
            OrderStatus::InProgress => String::from("In progress"),
            OrderStatus::Sent => String::from("Sent"),
            OrderStatus::Delivered => String::from("Delivered"),
            OrderStatus::Disputed(str) => {
                let s = String::from("Disputed, reason: ");
                s.add(str)
            }
        }
    }
}
