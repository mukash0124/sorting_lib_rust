pub fn merge_sort<T, F>(arr: &[T], compare: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    merge(merge_sort(left, &compare), merge_sort(right, &compare), compare)
}

fn merge<T, F>(mut left: Vec<T>, mut right: Vec<T>, compare: F) -> Vec<T>
where
    F: Fn(&T, &T) -> bool,
{
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut left_idx, mut right_idx) = (0, 0);

    while left_idx < left.len() && right_idx < right.len() {
        if compare(&left[left_idx], &right[right_idx]) {
            merged.push(left[left_idx].clone());
            left_idx += 1;
        } else {
            merged.push(right[right_idx].clone());
            right_idx += 1;
        }
    }

    merged.extend_from_slice(&left[left_idx..]);
    merged.extend_from_slice(&right[right_idx..]);

    merged
}