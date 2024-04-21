pub fn quicksort<T: Clone, F>(arr: &mut [T], compare: &F) -> Vec<T>
where
    F: Fn(&T, &T) -> bool,
{
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let pivot_idx = partition(arr, compare);

    let (left, right) = arr.split_at_mut(pivot_idx);
    let pivot = &mut right[0];

    let mut sorted_left = quicksort(left, compare);
    let mut sorted_right = quicksort(&mut right[1..], compare);

    sorted_left.push(pivot.clone());
    sorted_left.append(&mut sorted_right);

    sorted_left
}

fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    let pivot_idx = len / 2;
    let last_idx = len - 1;

    arr.swap(pivot_idx, last_idx);

    let mut i = 0;
    for j in 0..last_idx {
        if compare(&arr[j], &arr[last_idx]) {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, last_idx);
    i
}
