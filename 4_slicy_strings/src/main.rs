fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

fn main() {
    let s1 = String::from("hello world");
    let s1_first = first_word(&s1);

    let s2 = String::from("rust");
    let s2_first = first_word(&s2);

    println!(
        "string 1: {}, string 2: {}, string 1 first: {}, string 2 first: {}",
        &s1, &s1_first, &s2, &s2_first
    );
}
