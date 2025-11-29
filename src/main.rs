use std::io;
use std::io::Write;

fn main() {
    console_example();
}

fn console_example() {
    let mut my_string = String::new();

    io::stdout()
        .write("\nWrite your message:\n".as_bytes())
        .expect("Oh Oh! stdout failure!!");

    io::stdin()
        .read_line(&mut my_string)
        .expect("You gave us a bad value");

    println!("You typed: {}", my_string)
}
