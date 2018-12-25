use std::io;

pub fn solution() {
    println!("Hello, World!");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("{}", input);
}
