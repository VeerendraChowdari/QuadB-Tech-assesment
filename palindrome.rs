fn is_palindrome(s: &str) -> bool {
    let clean_string: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    clean_string.eq_ignore_ascii_case(&clean_string.chars().rev().collect::<String>())
}

fn main() {
    // Test cases for the first question
    println!("{}", is_palindrome("radar"));  
    println!("{}", is_palindrome("level")); 
    println!("{}", is_palindrome("hello")); 
}
