pub fn main() {
    let mut hello = String::from("안녕하세요");
    hello.push_str(" world");
    hello.push('!');
    println!("{}", hello);

    for c in hello.chars() {
        print!("{},", c);
    }
    println!();
    for b in hello.bytes() {
        print!("{},", b);
    }
    println!();

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // &String is coerced to &str
    let s3 = s1 + &s2;

    println!("s2: {}, s3: {}", s2, s3);
    // println!("{}", s1); // doesn't work

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // equivalent to creating an empty String and appending
    let tic_tac_toe = format!("{}-{}-{}", s1, s2, s3);
    println!("Let's play {}!", tic_tac_toe);
}
