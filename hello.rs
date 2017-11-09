#[no_mangle]
pub extern fn hello() {
    println!("Hello World!");
}

fn main() {
    hello();
}
