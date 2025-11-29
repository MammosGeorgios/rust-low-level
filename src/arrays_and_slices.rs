#[allow(dead_code)]
pub fn arrays_and_slices() {
    // Array initializations
    let _a: [i32; 4] = [10, 20, 30, 40];

    // Initialize an array with a repeated value
    let _zeros = [0; 8]; // [0, 0, 0, 0, 0, 0, 0, 0]

    let mut my_array: [i32; 5] = [1; 5];
    println!("{}", my_array[2]);

    my_array[2] = 2;
    println!("{}", my_array[2]);

    // We can use get method, to get an Option
    let fifth_element = my_array.get(4);

    println!("Print option value {}", fifth_element.unwrap()); // unwrap is a shitty way to handle this!

    let sixth_element = my_array.get(5);

    if sixth_element.is_none() {
        println!("No sixth element exists!!!!")
    }

    // Slices!!
    let _x = &my_array[1..3]; //This is a non-mutable slice!

    let mut_y = &mut my_array[1..3];
    mut_y[0] = 99;

    println!("Change my_array value through mut_y slice: {:?}", my_array);

    println!("Length: {}", my_array.len())
}
