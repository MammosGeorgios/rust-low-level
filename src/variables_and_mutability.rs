
// Lifetime 'static' -> Lives for the duration of the program
static STATIC_X: i32 = 1;

// We can make it mutable but it's considered dangerous.
// We need unsafe blocks to actually access it
static mut MUT_STATIC_X: i32 = 2;

// For strings, we usually have the following
static STATIC_STRING: &str = "We can also have static string which is pretty common";

#[allow(dead_code)]
pub fn variables_and_mutability() {
    // let name: type = default value;

    // let x = 2; // by default this will be i32 and immutable
    // x = 3; // this will throw a compiler error

    let mut x = 2; // With mut keyword, x is marked as mutable and I can edit it
    x += 1;
    println!("{}", x);

    // ------------------
    const PI: f32 = 3.14;
    println!("PI = {}", PI);

    println!("I can easily access STATIC_X: {}", STATIC_X);

    unsafe {
        let y = &raw mut MUT_STATIC_X;

        println!(
            "But for MUT_STATIC_X I need an unsafe block, and use &raw mut to get access to MUT_STATIC_X: {}",
            *y
        );
    }

    println!("{}", STATIC_STRING);
}