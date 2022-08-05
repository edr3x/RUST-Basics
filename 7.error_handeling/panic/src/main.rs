use core::panic;

fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(69);
}

fn c(num: i32) {
    if num == 69 {
        panic!("Don't call {}", num);
    }
}
