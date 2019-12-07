fn doesnt_work() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // v.push(6); // uncomment this line to break

    println!("The first element is: {}", first);
    println!("The last element is: {}", &v[5]);
}

fn does_work() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    println!("The first element is: {}", first);

    v.push(6);
    println!("The last element is: {}", &v[5]);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn main() {
    does_work();

    // other way to create a vector
    let mut v = Vec::new();
    // or typed (only necessary if not adding anything):
    // let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    println!("v is: {:?}", v);

    println!("looping over v...");
    for i in &v {
        println!("item: {}", i);
    }

    println!("multiplying each item by 2...");
    for i in &mut v {
        *i *= 2;
    }
    println!("v is: {:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("printing row items...");
    for i in &row {
        match i {
            SpreadsheetCell::Int(int) => println!("{}", int),
            SpreadsheetCell::Float(float) => println!("{}", float),
            SpreadsheetCell::Text(text) => println!("{}", text),
        }
    }
}
