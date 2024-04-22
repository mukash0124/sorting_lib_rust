pub fn selection_sort<T, F>(array: &mut [T], compare_func: &F)
where
    T: Copy + PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    for i in 0..array.len() {
        let mut min_idx = i;
        for j in i + 1..array.len() {
            if compare_func(&array[j], &array[min_idx]) {
                min_idx = j;
            }
        }
        if min_idx != i {
            array.swap(i, min_idx);
        }
    }
}