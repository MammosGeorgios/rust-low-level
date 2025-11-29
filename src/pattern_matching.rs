#[allow(dead_code)]
pub fn pattern_matching() {
    match_message(Message::Welcome { x: 10, y: 20 });
    match_message(Message::Chat {
        msg: "Hello Pattern Matching",
    });
    match_message(Message::Color(8, 16, 24));
    match_message(Message::Quit);

    match_message_with_default(Message::Quit);
    match_message_with_default(Message::Welcome { x: 1, y: 1 });

    let x = Message::Welcome { x: 5, y: 3 };

    // We can do the following with a match statement
    let y: i32 = match x {
        Message::Welcome { x, y } => x + y,
        _ => -1,
    };

    println!("{}", y);

    // if let
    let array: [i32; 500] = [0; 500];

    if let Some(x) = array.get(501) {
        println!("Element found with value {}", x);
    } else {
        println!("HOLD ON PARTNER!!!!");
    }

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("top = {}", top);
    }

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

fn match_message_with_default(message: Message) {
    match message {
        Message::Quit => println!("It quits"),
        _ => println!("It doesn't quit"),
    }
}

#[allow(dead_code)]
enum Message2 {
    // TODO: Add enum variants
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
#[allow(dead_code)]
fn process_message(msg: Message2) -> String {
    // TODO: Match and return appropriate string
    let result: String = match msg {
        Message2::Quit => "Quit".to_string(),
        Message2::Move { x, y } => format!("Moving to ({x},{y})"),
        Message2::Write(m) => format!("Text: {m}"),
        Message2::ChangeColor(r, g, b) => format!("Changing color to ({r}, {g}, {b})"),
    };

    result
}
