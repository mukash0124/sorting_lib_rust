pub fn insertion_sort<T, F>(arr: &[T], compare: &F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    let mut sorted = arr.to_vec();

    for i in 1..sorted.len() {
        let mut j = i;
        while j > 0 && !compare(&sorted[j - 1], &sorted[j]) {
            sorted.swap(j, j - 1);
            j -= 1;
        }
    }

    sorted
}