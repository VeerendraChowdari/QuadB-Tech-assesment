fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // If the array has an even number of elements, return the average of the middle two elements
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        // If the array has an odd number of elements, return the middle element
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];  ///any numbers we can take
    let arr2 = [1, 2, 3, 4, 5, 6];
    
    println!("Median of arr1: {}", find_median(&arr1));
    println!("Median of arr2: {}", find_median(&arr2));
}
