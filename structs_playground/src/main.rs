#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.height > target.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 640,
        height: 480,
    };

    println!("Rectangle: {:#?}", rectangle);
    println!("Area of rectangle: {}", rectangle.area());

    let rectangle2 = Rectangle {
        width: 40,
        height: 30,
    };

    println!(
        "Rectangle 1 can hold rectangle 2: {}",
        rectangle.can_hold(&rectangle2)
    );
}
