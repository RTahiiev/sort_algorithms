use std::{cmp::PartialOrd, fmt::Debug, marker::Sized};

fn bubble_sort<T: PartialOrd>(arr: &mut Vec<T>) -> &Vec<T> {
    let arr_length = arr.len();
    for i in 0..arr_length - 1 {
        for j in 0..arr_length - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

fn selection_sort<T: PartialOrd>(arr: &mut Vec<T>) -> &Vec<T> {
    let arr_length = arr.len();
    for i in 0..arr_length - 1 {
        for j in i + 1..arr_length {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
    arr
}

fn insertion_sort<T: PartialOrd>(arr: &mut Vec<T>) -> &Vec<T> {
    let arr_length = arr.len();
    for i in 1..arr_length {
        for j in (0..i + 1).rev() {
            if j > 0 && arr[j - 1] < arr[j] {
                arr.swap(j, j - 1);
            } else {
                break;
            }
        }
    }
    arr.reverse();
    arr
}

fn merge_sort<T: PartialOrd + Debug + Copy>(arr: &mut Vec<T>, l: usize, r: usize) -> &Vec<T> {
    if l < r {
        let m = l + (r - l) / 2;
        merge_sort(arr, l, m);
        merge_sort(arr, m + 1, r);
        merge(arr, l, m, r);
    }
    arr
}

// function for merge sort
fn merge<T: PartialOrd + Sized + Debug + Copy>(
    arr: &mut Vec<T>,
    p: usize,
    q: usize,
    r: usize,
) -> &Vec<T> {
    let l_size = q - p + 1;
    let r_size = r - q;

    let mut left: Vec<T> = Vec::with_capacity(l_size);
    let mut right: Vec<T> = Vec::with_capacity(r_size);

    for i in 0..l_size {
        left.push(arr[i]);
    }

    for i in 0..r_size {
        right.push(arr[i]);
    }

    for i in 0..l_size {
        left[i] = arr[p + i];
    }

    for i in 0..r_size {
        right[i] = arr[q + 1 + i];
    }

    // init all vars
    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    // merge two parts in sorted order
    while i < l_size && j < r_size {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // put last values
    while i < l_size {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < r_size {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }

    arr
}

fn heap_sort<T: PartialOrd + Debug>(arr: &mut Vec<T>) -> &Vec<T> {
    // build heap from vector
    build_heap(arr);
    // heap sort
    let arr_length = arr.len();
    for index in (0..arr_length).rev() {
        arr.swap(0, index);
        heapify(arr, index, 0);
    }
    arr
}

// functions for heap sort
fn heapify<T: PartialOrd>(arr: &mut Vec<T>, size: usize, index: usize) -> &Vec<T> {
    let mut largest_index = index;
    let left = 2 * index + 1;
    let right = 2 * index + 2;
    if left < size && arr[left] > arr[largest_index] {
        largest_index = left;
    }

    if right < size && arr[right] > arr[largest_index] {
        largest_index = right;
    }

    if largest_index != index {
        arr.swap(index, largest_index);
        heapify(arr, size, largest_index);
    }
    arr
}

fn build_heap<T: PartialOrd + Debug>(arr: &mut Vec<T>) -> &Vec<T> {
    let arr_length = arr.len();
    for index in (0..arr_length / 2).rev() {
        heapify(arr, arr_length, index);
    }
    arr
}

fn quick_sort<T: PartialOrd + Sized + Debug + Copy>(
    arr: &mut Vec<T>,
    low: isize,
    high: isize,
) -> &Vec<T> {
    if low < high {
        let pi = partition(arr, low, high) as isize;
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
    arr
}

// function for quick sort
fn partition<T: PartialOrd + Sized + Debug + Copy>(
    arr: &mut Vec<T>,
    low: isize,
    high: isize,
) -> usize {
    let high = high as usize;
    let pivot = arr[high];

    // first iteration i = -1, so we need to cast low after substraction from itype to utype
    let mut i: isize = low - 1;
    let low = low as usize;

    for j in low..high {
        if arr[j] <= pivot {
            i += 1;
            let i = i as usize;
            arr.swap(i, j);
        }
    }
    let i = i as usize;
    arr.swap(i + 1, high);
    i + 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bubble() {
        // input data
        let mut list = vec![1, 12, 9, 5, 6, 10];
        let list = bubble_sort(&mut list);
        assert_eq!(vec![1, 5, 6, 9, 10, 12], *list);
    }

    #[test]
    fn test_selection() {
        // input data
        let mut list = vec![1, 12, 9, 5, 6, 10];
        let list = selection_sort(&mut list);
        assert_eq!(vec![1, 5, 6, 9, 10, 12], *list);
    }
    #[test]
    fn test_insertion() {
        // input data
        let mut list = vec![1, 12, 9, 5, 6, 10];
        let list = insertion_sort(&mut list);
        assert_eq!(vec![1, 5, 6, 9, 10, 12], *list);
    }

    #[test]
    fn test_heap() {
        // input data
        let mut list = vec![1, 12, 9, 5, 6, 10];
        let list = heap_sort(&mut list);
        assert_eq!(vec![1, 5, 6, 9, 10, 12], *list);
    }

    #[test]
    fn test_merge() {
        // input data
        let mut list = vec![1, 12, 9, 5, 6, 10];
        let size = list.len() - 1;
        let list = merge_sort(&mut list, 0, size);

        assert_eq!(vec![1, 5, 6, 9, 10, 12], *list);
    }

    #[test]
    fn test_quick() {
        // input data
        let mut list = vec![1, 12, 9, 5, 6, 10];
        let size = (list.len() - 1) as isize;
        let list = quick_sort(&mut list, 0, size);

        assert_eq!(vec![1, 5, 6, 9, 10, 12], *list);
    }
}
