
fn _linear_search<T>(arr : &[T], value: T) -> Option<usize> where T: PartialEq  {
    for (index, item) in arr.iter().enumerate() {
        if item == &value {
            return Some(index);
        }
    }
    return None;
}