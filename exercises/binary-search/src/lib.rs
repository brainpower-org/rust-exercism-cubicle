use std::cmp::Ordering;

pub fn find<T, K>(array: T, key: K) -> Option<usize>
where
    T: AsRef<[K]>,
    K: Ord,
{
    find_with_index(array, key, 0)
}

fn find_with_index<T, K>(elements: T, key: K, start: usize) -> Option<usize>
where
    T: AsRef<[K]>,
    K: Ord,
{
    let array = elements.as_ref();
    match array.len() {
        0 => None,
        1 if array[0] == key => Some(0 + start),
        1 => None,
        c => {
            let middle = c / 2;
            match array[middle].cmp(&key) {
                Ordering::Equal => Some(middle + start),
                Ordering::Less => find_with_index(&array[middle..c], key, middle + start),
                Ordering::Greater => find_with_index(&array[0..middle], key, 0),
            }
        }
    }
}
