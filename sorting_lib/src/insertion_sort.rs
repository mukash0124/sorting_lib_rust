pub fn insertion_sort<T: Clone, F>(array: &mut [T], compare_func: &F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..array.len() {
        let mut current_index = i;
        while current_index > 0 && compare_func(&array[current_index], &array[current_index - 1]) {
            array.swap(current_index, current_index - 1);
            current_index -= 1;
        }
    }
}