fn main() {
    println!("Hello, world!");

    let value = Some(6);
    if let Some(x) = value {
        println!("Value is {}", x);
    }
    else {
        println!("No value")
    }
}
