/// Linear search algorithm.
/// It is a brute force algorithm.
/// It goes through the array one by one until it finds the value.
/// It's time complexity is O(n).
pub fn _linear_search<T>(arr: &Vec<T>, predicate: fn(&T) -> bool) -> Option<T> where T: Copy, T: PartialEq {
    for i in arr.iter() {
        if predicate(i) {
            return Some(*i);
        }
    }
    return None;
}


/// Binary search algorithm. (CAUTION: ARRAY MUST BE SORTED)
/// It is a divide and conquer algorithm.
/// It splits the array into two halves and checks if the value is in the first half or the second half.
/// It goes like until array is fully scanned or value found in the array.
/// It's time complexity is O(log n) or O(n log n).
pub fn _binary_search<T>(arr: &Vec<T>, value: T) -> Option<usize> where T: Ord, T: PartialEq {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = (low + high) / 2;
        if arr[mid] == value {
            return Some(mid);
        } else if arr[mid] < value {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    return None;
}