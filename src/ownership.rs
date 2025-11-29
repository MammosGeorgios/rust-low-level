#[allow(dead_code)]
pub fn ownership() {
    //  The Three Rules of Ownership
    // 
    //     Each value in Rust has a single owner.
    //     When the owner goes out of scope, the value is dropped (freed).
    //     A value can be moved to a new owner, but only one owner can exist at a time.

    let s1 = "hehe".to_string();

    let s2 = s1;

    // take_string(s1); // This is not valid, because ownership has moved to s2 [Value used after being moved]
    take_string_ownership(s2); // this is ok now

    // take_string(s2); // Since we moved s2 to the take_string already, i cannot do the same [Value used after being moved]

    // A dirty solution to the above problem is to have take_string_ownership return the string
    // The following code is fully valid:
    let s_a = "haha".to_string();
    let s_b = take_string_ownership_and_return_value(s_a);
    let s_c = take_string_ownership_and_return_value(s_b);
    println!("{s_c}");
    
}

fn take_string_ownership(s: String) {
    println!("{}", s)
}

fn take_string_ownership_and_return_value(s: String) -> String {
    println!("{}", s);
    s
}
