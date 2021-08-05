pub fn find<T: Ord, R: AsRef<[T]>>(array: R, key: T) -> Option<usize> {
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let mid = array.len() / 2;
    if array[mid] == key {
        Some(mid)
    } else if array[mid] > key {
        find(&array[..mid], key)
    } else {
        find(&array[mid + 1..], key).map(|i| i + mid + 1)
    }
}
