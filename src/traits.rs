#[allow(dead_code)]
pub fn traits() {
    let r = Rectangle::new(2, 3);
    println!("Area is {}", r.get_area());

    let s = RectangleTraited {
        height: 3,
        width: 2,
    };

    print_width_and_perimeter(&s)
}

struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]

struct Square {
    size_length: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(height: u32, width: u32) -> Self {
        // -> Rectangle would also work, but Self is so fancy
        Rectangle { height, width }
    }
    fn get_area(&self) -> u32 {
        self.height * self.width
    }
}

#[allow(dead_code)]
impl Square {
    fn new(size_length: u32) -> Self {
        Square { size_length }
    }

    fn get_area(&self) -> u32 {
        self.size_length * self.size_length
    }
}

//************************************
// Versions with traits

trait Shape {
    fn get_area(&self) -> u32;
    fn get_perimeter(&self) -> u32;
}

struct RectangleTraited {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
struct SquareTraited {
    size_length: u32,
}

impl Shape for RectangleTraited {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn get_perimeter(&self) -> u32 {
        self.width * 2 + self.height * 2
    }
}

impl Shape for SquareTraited {
    fn get_area(&self) -> u32 {
        self.size_length * self.size_length
    }

    fn get_perimeter(&self) -> u32 {
        self.size_length * 4
    }
}

fn print_width_and_perimeter(s: &dyn Shape) {
    println!(
        "Perimeter is {} and area is {}",
        s.get_perimeter(),
        s.get_area()
    )
}
