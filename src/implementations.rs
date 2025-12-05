#[allow(dead_code)]
pub fn implementations() {
    let r = Rectangle::new(2, 3);
    println!("Area is {}", r.get_area());
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(height: u32, width: u32) -> Self { // -> Rectangle would also work, but Self is so fancy
        Rectangle { height, width }
    }
    fn get_area(&self) -> u32 {
        self.height * self.width
    }
}
