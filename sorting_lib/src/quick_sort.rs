pub fn quick_sort<T, F>(array: &mut [T], compare_func: &F)
where
    T: Copy + PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    if array.len() <= 1 {
        return;
    }

    let pivot_element = &array[array.len() / 2];

    let mut less_items = Vec::new();
    let mut equal_items = Vec::new();
    let mut greater_items = Vec::new();

    for item in array.iter() {
        if compare_func(item, pivot_element) {
            less_items.push(*item);
        } else if compare_func(pivot_element, item) {
            greater_items.push(*item);
        } else {
            equal_items.push(*item);
        }
    }

    quick_sort(&mut less_items, compare_func);
    quick_sort(&mut greater_items, compare_func);

    array[..less_items.len()].copy_from_slice(&less_items);
    array[less_items.len()..less_items.len() + equal_items.len()].copy_from_slice(&equal_items);
    array[less_items.len() + equal_items.len()..].copy_from_slice(&greater_items);
}
