fn shortest_word(s: &str) -> Option<&str> {
    // Split the string into words
    let words: Vec<&str> = s.split_whitespace().collect();

    // Find the shortest word
    words.iter().min_by_key(|&word| word.len()).copied()
}

fn main() {
    let input = "apple banana cherry date";//strings
    match shortest_word(input) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("The string is empty"),
    }
}
