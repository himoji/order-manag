use crate::prelude::*;
#[derive(Debug)]
pub struct Order {
    pub name: String,
    pub shop: String,
    pub price: usize,
    pub status: OrderStatus,
}

impl Order {
    pub fn print_info(&self) {
        println!(
            "Order:\nName: {}Shop: {}Price: {}\nStatus: {:?}\n",
            self.name,
            self.shop,
            self.price,
            self.status.get_order_status_in_string()
        );
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    pub fn update_order(&mut self, key: &str, value: &str) {
        match key.trim() {
            "name" => {
                self.name = value.to_string();
            }
            "shop" => {
                self.shop = value.to_string();
            }
            "price" => {
                self.price = value.trim().parse().expect("Value is not an integer!");
            }

            &_ => (),
        }
    }
}
