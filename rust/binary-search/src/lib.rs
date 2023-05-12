pub fn find<T: Ord + PartialEq>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    find_recursion(array.as_ref(), key, 0)
}

pub fn find_recursion<T: Ord + PartialEq>(
    array: &[T],
    key: T,
    current_index: usize,
) -> Option<usize> {
    let middle_index = array.len() / 2;

    match array.get(middle_index)?.cmp(&key) {
        std::cmp::Ordering::Greater => find_recursion(&array[..middle_index], key, current_index),
        std::cmp::Ordering::Equal => Some(current_index + middle_index),
        std::cmp::Ordering::Less => find_recursion(
            &array[middle_index + 1..],
            key,
            current_index + middle_index + 1,
        ),
    }
}
