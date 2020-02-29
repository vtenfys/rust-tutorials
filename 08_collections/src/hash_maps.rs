use std::collections::HashMap;

pub fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    // creating a hashmap using insert()
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    println!("{:?}", scores);

    // blue and yellow can no longer be used
    // println!("{}, {}", blue, yellow);

    // using entry() to insert if a value doesn't exist
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);

    // creating a hashmap using zip() and collect()
    // zip() creates a vector of tuples and collect() converts this to a hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    // teams and initial scores can still be accessed
    println!("{:?}, {:?}", teams, initial_scores);

    // getting values
    println!(
        "The blue team scored {}",
        scores
            .get(&String::from("Blue")) // is this a memory leak?
            .expect("Blue team should exist")
    );

    // or using a for loop - order isn't guaranteed
    for (key, value) in &scores {
        println!("{} scored {}", key, value);
    }

    // creating a word frequency map
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
