pub fn iterators() {
    let mut example_vec = vec![1, 2, 3, 4, 5];
    let mut _iter = example_vec.iter();
    // Returns if any of the elements in the iterator match the predicate.
    let mut _check = _iter.any(|&x| x == 2);
    // Returns if all of the elements in the iterator match the predicate.
    _check = _iter.all(|&x| x == 2);
    // Returns the first element satisfying the predicate.
    let mut _first = _iter.find(|&&x| x == 2);
    // Returns the index of the first element satisfying the predicate.
    let mut _index = _iter.position(|&x| x == 2);
    // Returns the index of the last element satisfying the predicate.
    _index = _iter.rposition(|&x| x == 2);
    // Returns the next element in the iterator, returns None if the iterator is empty.
    let mut _next = _iter.next();
    // Returns reverse iterator. While order of iteration is not reversed,
    // the direction in which the iterator moves is.
    let mut _revers = _iter.clone().rev();
    // Returns the number of elements in the iterator.
    let mut _count = _iter.clone().count();
    // filters the iterator with the given predicate.
    let mut _filter = _iter.clone().filter(|&&x| x == 2);
    // Returns the maximum element in the iterator.
    let mut _max = _iter.max();



}
