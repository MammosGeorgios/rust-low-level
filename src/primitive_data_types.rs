#[allow(dead_code)]
pub fn primitive_data_types() {
    let _x: i32 = 0; // Usually in c int is also i32 as a default

    /*
    u8, u16, u32, u64, u128, usize (it will match the architecture i am currently in, so x64 will make usize into u64)

    We should prefer to be explicit

    i8, i16, i32, i64, i128, isize
     */

    let _c: char = 'a'; // a character is by default a 32but value in rust
    // char is very different from C
    let _emoji: char = '🚀'; // This is valid :)

    //////////////////////////////////

    let _unit: () = ();
    // This is basically the null type for rust

    let _slice: &[i32];

    // Boolean
    let _my_boolean: bool = true; // You cannot say my_boolean: bool = 1; (but you can say my_boolean: bool = 1 != 0;)

    // Casting
    let _y: u64 = 1 as u64;

    let _y: u64 = 1u64;

    ///////////////////////
    // Function pointers

    let f: fn(i32, i32) -> i32 = add;
    let result = f(2, 3);
    println!("Call function with pointer to add 2 and 3: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
