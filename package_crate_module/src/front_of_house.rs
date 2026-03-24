pub mod hosting;

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}

fn super_call() {
    super::eat();
}

pub struct Food{
    name : String,
    age: u8,
}