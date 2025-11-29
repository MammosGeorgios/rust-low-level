#[allow(dead_code)]
pub fn simple_control_flow() {
    let mut i = 32;
    if i < 0 {
        println!("I is negative")
    } else if i == 32 {
        println!("I is 32")
    } else {
        println!("I is positive but not 32")
    }

    while i != 0 {
        println!("{}", i);
        i -= 1;
    }

    // Rust has the following loop format as well , which is interesting
    i = 15;
    let new_i = loop {
        if i == 0 {
            break 69;
        }

        println!("Inside our loop {}", i);
        i -= 1;
    };

    println!("new_i= {}", new_i);

    // for loops
    for i in 0..5 {
        println!("loop {}", i);
    }
    
    for i in 0..=5 {
        println!("loop including last value {}", i);
    }

    let names = ["Alice", "Bob", "Martin"];

    for name in names.iter() {
        println!("Name is {}", name);
    }
}
