fn main() {
    let mut n = 1;
    loop {
        println!("Hello! {}", n);
        n += 1;
        std::thread::sleep_ms(1000);
    }
}
