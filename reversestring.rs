fn reverse_string(s: &str) -> String {
    let mut reversed = String::new(); // Create an empty string to store the reversed characters
    for c in s.chars().rev() {
        reversed.push(c); // Append each character in reverse order to the reversed string
    }
    reversed // Return the reversed string
}

fn main() {
    let original = "hello";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
