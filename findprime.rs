fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    if n <= 3 {
        return true; // 2 and 3 are prime
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false; // Even numbers greater than 2 and numbers divisible by 3 are not prime
    }

    let mut i = 5;
    while i * i <= n {
        // Check divisibility by numbers of the form 6k ± 1 (where k is an integer)
        if n % i == 0 || n % (i + 2) == 0 {
            return false; // If n is divisible by any of these numbers, it's not prime
        }
        i += 6;
    }
    true // If no divisor is found, n is prime
}

fn main() {
    let num = 29; // Change this to test different numbers
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
