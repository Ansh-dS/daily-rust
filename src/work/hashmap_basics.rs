use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // 1. Inserting key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 2. Accessing a value (returns an Option<&V>)
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(val) => println!("{}: {}", team_name, val),
        None => println!("Team not found"),
    }

    // 3. Iterating over key-value pairs
    for (team, score) in &scores {
        println!("{} has {} points", team, score);
    }

    // 4. Updating values:
    // Overwriting directly:
    scores.insert(String::from("Blue"), 25);

    // Only insert if key does not exist yet (.entry + .or_insert)
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(50); // won't overwrite 25

    println!("\nscores map: {:?}", scores);

    // 5. Practical example: word counting with entry API
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("word counts: {:?}", word_count);
}
