pub fn selection_sort<T, F>(arr: &[T], compare: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    let mut sorted = arr.to_vec();

    for i in 0..sorted.len() {
        let mut min_idx = i;
        for j in (i + 1)..sorted.len() {
            if compare(&sorted[j], &sorted[min_idx]) {
                min_idx = j;
            }
        }
        if i != min_idx {
            sorted.swap(i, min_idx);
        }
    }

    sorted
}