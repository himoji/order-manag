
mod order;
mod order_status;
mod menu;

mod prelude {
    pub use crate::menu::atm;
    pub use crate::order::Order;
    pub use crate::order_status::OrderStatus;
}
use prelude::*;

fn main() {
    const VEC_CAPACITY: usize = 10;
    let order_array = Vec::with_capacity(VEC_CAPACITY);

    atm(order_array);
}
