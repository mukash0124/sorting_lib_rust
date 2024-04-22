# Sorting Library

This is a custom sorting library implemented in Rust, providing several sorting algorithms for various data types. The library includes the following sorting algorithms:

- Merge Sort
- Quick Sort
- Selection Sort
- Insertion Sort

# Installation

To use this library in your Rust project, add it as a dependency in your Cargo.toml:

```
[dependencies]
custom_sorting = { git = "https://github.com/mukash0124/sorting_lib_rust" }
```

# Usage

Here is an example of how to use the sorting algorithms provided by this library:

```
use sorting_lib::insertion_sort::insertion_sort;
use sorting_lib::quick_sort::quick_sort;
use sorting_lib::merge_sort::merge_sort;
use sorting_lib::selection_sort::selection_sort;

fn main() {
    let mut arr = [8, 3, 5, 2, 9, 1, 7, 4, 6, 10];
    insertion_sort(&mut arr, &|a, b| a < b);
    println!("Sorted array: {:?}", arr);
    arr = [8, 3, 5, 2, 9, 1, 7, 4, 6, 10];
    quick_sort(&mut arr, &|a, b| a < b);
    println!("Sorted array: {:?}", arr);
    arr = [8, 3, 5, 2, 9, 1, 7, 4, 6, 10];
    merge_sort(&mut arr, &|a, b| a < b);
    println!("Sorted array: {:?}", arr);
    arr = [8, 3, 5, 2, 9, 1, 7, 4, 6, 10];
    selection_sort(&mut arr, &|a, b| a < b);
    println!("Sorted array: {:?}", arr);
}
```

# License

This library is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
