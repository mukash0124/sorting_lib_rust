pub fn merge_sort<T, F>(array: &mut [T], compare_func: &F)
where
    T: Copy + PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let length = array.len();
    if length <= 1 {
        return;
    }

    let middle = length / 2;
    let (left_half, right_half) = array.split_at_mut(middle);

    merge_sort(left_half, compare_func);
    merge_sort(right_half, compare_func);

    let mut merged_array = Vec::with_capacity(length);
    let (mut left_index, mut right_index) = (0, 0);

    while left_index < left_half.len() && right_index < right_half.len() {
        if compare_func(&left_half[left_index], &right_half[right_index]) {
            merged_array.push(left_half[left_index]);
            left_index += 1;
        } else {
            merged_array.push(right_half[right_index]);
            right_index += 1;
        }
    }

    merged_array.extend_from_slice(&left_half[left_index..]);
    merged_array.extend_from_slice(&right_half[right_index..]);

    array.copy_from_slice(&merged_array);
}