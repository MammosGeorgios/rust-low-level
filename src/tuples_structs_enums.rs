#[allow(dead_code)]
pub fn tuples_structs_enums() {
    // Tuples
    let _point: (i32, i32) = (0, 1);
    let point: (i32, u64, bool) = (1, 1, false);

    let (a, b, c) = point;
    println!("Print using deconstruction: {}, {}, {}", a, b, c);
    println!(
        "Print using tuple.0 .1 etc: {} {} {}",
        point.0, point.1, point.2
    );
    println!("print using just the tuple: {:?}", point);

    // Named tuple /Struct
    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);

    // Struct
    let phil = User {
        name: "Phil",
        age: 72,
        income: 130000,
    };

    println!(
        "{} is {} years old and makes {} a year",
        phil.name, phil.age, phil.income
    );

    // enum

    let _dir = Direction::UP;
    let _dir = Direction::DOWN;
    let _dir = Direction::LEFT;
    let _dir = Direction::RIGHT;

    match_message(Message::Welcome { x: 10, y: 20 });
    match_message(Message::Chat {
        msg: "Hello Pattern Matching",
    });
    match_message(Message::Color(8, 16, 24));
    match_message(Message::Quit)
}

struct User {
    name: &'static str,
    age: u8,
    income: i32,
}

// In C these would all be ints
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

// We can also do the following
enum Message {
    Welcome { x: i32, y: i32 },
    Chat { msg: &'static str },
    Color(u8, u8, u8),
    Quit,
}

// enums are powerful with pattern matching

fn match_message(message: Message) {
    match message {
        Message::Welcome { x, y } => println!("Welcome Message ({},{})", x, y),
        Message::Chat { msg } => println!("{}", msg),
        Message::Color(x, y, z) => println!("Color ({}, {}, {})", x, y, z),
        Message::Quit => println!("QUIT!!!!"),
    }
}
