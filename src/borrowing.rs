#[allow(dead_code)]
pub fn borrowing() {
    let s1 = "hehe".to_string();
    println!("{}", calc_len(s1)); // Calc len took ownership of s1
    // println!("{s1}");// Cannot do this anymore

    // By using &, we pass the reference only, keeping the ownership inside this method
    //calc_len_with_borrowing borrows the reference instead
    let s2 = "haha".to_string();
    println!("{}", calc_len_with_borrowing(&s2));
    println!("{}", s2);

    // We can also pass mutable by reference
    let mut s3 = "hoho".to_string();
    println!("{}", calc_len_and_mutate(&mut s3));
    println!("{}", calc_len_and_mutate(&mut s3));
    println!("{}", calc_len_and_mutate(&mut s3));
    println!("{}", calc_len_and_mutate(&mut s3));
    println!("{}", calc_len_and_mutate(&mut s3));
    println!("{}", calc_len_and_mutate(&mut s3));
    println!("{}", calc_len_and_mutate(&mut s3));
    println!("{}", calc_len_and_mutate(&mut s3));

    // We can have as many immutable borrows as we want
    let _s1_br1 = &s2;
    let _s1_br2 = &s2;
    let _s1_br3 = &s2;
    let _s1_br4 = &s2;
    let _s1_br5 = &s2;

    // Mutable borrows however are different when mixed with
    // The following code fails with [Cannot borrow 's3' as mutable more than once at a time
    // let s3_br1 = &mut s3;
    // let s3_br2 = &mut s3;
    // let s3_br3 = &mut s3;
    // let s3_br4 = &mut s3;
    // let s3_br5 = &mut s3;
    //
    // calc_len_and_mutate(s3_br1);
    // calc_len_and_mutate(s3_br2);
}

fn calc_len(s: String) -> usize {
    s.len()
}

fn calc_len_with_borrowing(s: &String) -> usize {
    s.len()
}

fn calc_len_and_mutate(s: &mut String) -> usize {
    s.push('A');
    s.len()
}
