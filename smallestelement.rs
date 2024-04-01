fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; // Return None if k is greater than the length of the array
    }

    let mut sorted_arr = arr.to_vec(); // Create a mutable copy of the array
    sorted_arr.sort(); // Sort the array in ascending order

    Some(sorted_arr[k - 1]) // Return the kth smallest element (subtract 1 because indexing starts from 0)
}

fn main() {
    let arr = [4, 2, 7, 1, 9, 5];
    let k = 3; // Change this to test different values of k
    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
