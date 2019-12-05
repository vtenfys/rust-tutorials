enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn log(&self) {
        match self {
            IpAddr::V4(x1, x2, x3, x4) => println!("{}.{}.{}.{}", x1, x2, x3, x4),
            IpAddr::V6(addr) => println!("{}", addr),
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving {}, {}", x, y),
            Message::Write(message) => println!("Writing message: {}", message),
            Message::ChangeColor(r, g, b) => println!("Changing colour to {},{},{}", r, g, b),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    home.log();
    loopback.log();

    let move_right = Message::Move { x: 10, y: 0 };
    let say_hello = Message::Write(String::from("Hello, world!"));
    let become_red = Message::ChangeColor(255, 0, 0);
    let quit = Message::Quit;
    move_right.call();
    say_hello.call();
    become_red.call();
    quit.call();
}
