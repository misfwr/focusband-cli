fn main() {
    let option1 = std::env::args().nth(1).expect("please supply an argument");
    if option1.starts_with("hola") {
        println!("Hello, world!");
    }
}
