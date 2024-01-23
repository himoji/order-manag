use crate::prelude::*;
pub use std::io::stdin;

fn int_input(str_to_show: &str) -> usize {
    let mut user_input = String::new();
    println!("{}", str_to_show);
    loop {
        user_input.clear();
        if stdin().read_line(&mut user_input).is_ok() {
            let _user_input: usize = match user_input.trim().parse() {
                Ok(a) => return a,
                Err(e) => {
                    println!("Failed to parse! {e}: {user_input}");
                    continue;
                }
            };
        } else {
            println!("Failed to read the input!");
            continue;
        }
    }
}

fn string_input(str_to_show: &str) -> String {
    let mut user_input = String::new();
    println!("{str_to_show}");

    loop {
        user_input.clear();
        if stdin().read_line(&mut user_input).is_ok() {
            return user_input;
        } else {
            continue;
        }
    }
}

pub fn create_and_push_order(
    name: String,
    shop: String,
    price: usize,
    status: OrderStatus,
    order_array: &mut Vec<Order>,
) {
    let order: Order = Order {
        name,
        shop,
        price,
        status,
    };

    match order_array.last() {
        Some(_a) => {
            println!("Order array overflow!");
        }
        None => {
            order_array.push(order);
        }
    }
}

pub fn atm(mut order_array: Vec<Order>) {
    loop {
        let user_input = int_input("Choose (1,2,3,4,5):\n1) Create an order\n2) Check info\n3) Update status\n4) Update order info\n5) Delete an order");

        match user_input {
            1 => {
                let name = string_input("Order name: ");
                let shop = string_input("Order shop: ");
                let price = int_input("Order price: ");
                let status = OrderStatus::InProgress;

                create_and_push_order(name, shop, price, status, &mut order_array);

                order_array.last().unwrap().print_info();
                print!("At index: {:?}", order_array.last());
            }
            2..=5 => {
                let index = int_input("Order index: ");
                if let Some(_a) = order_array.get(index) {
                    match user_input {
                        2 => {
                            order_array[index].print_info();
                        }
                        3 => {
                            let user_input = int_input("Choose (1,2,3,4):\n1) In progress\n2) Sent\n3) Delivered\n4) Disputed");

                            match user_input {
                                1 => {
                                    order_array[index].update_status(OrderStatus::InProgress);
                                }
                                2 => {
                                    order_array[index].update_status(OrderStatus::Sent);
                                }
                                3 => {
                                    order_array[index].update_status(OrderStatus::Delivered);
                                }
                                4 => {
                                    let reason = string_input("Reason: ");

                                    order_array[index].update_status(OrderStatus::Disputed(reason));
                                }

                                _ => continue,
                            }
                        }

                        4 => {
                            let key = string_input("Key: ");
                            let value = string_input("Value: ");

                            order_array[index].update_order(&key, &value);
                        }

                        5 => {
                            order_array.remove(index);
                        }
                        _ => continue,
                    }
                } else {
                    println!("Sorry, the order cannot be found");
                }
            }
            _ => continue,
        }
    }
}
