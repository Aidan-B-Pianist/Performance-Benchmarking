/*
    Semester Project Hybrid Sort / Intergalactic Zoo Trip (Knapsack Problem)
    By Aidan Burchett and Geraldo
*/
use std::time::Instant;
use rand::Rng;

fn main() {
    let sizes = [10, 100, 1000, 10000, 100000];

    let k_sizes = [5, 10, 20, 35, 50];

    for size in sizes {
        println!("Testing array size of {}.", size);

        let mut rng = rand::thread_rng();
        let array: Vec<i32> = (0..size).map(|_| rng.gen_range(0..1000000)).collect();

        for k in k_sizes {
            
            let mut test_array = array.clone();

            let start_time = Instant::now();
            quick_hybrid_sort(&mut test_array, k);
            let duration = start_time.elapsed();

            let is_sorted = test_array.windows(2).all(|n| n[0] <= n[1]);

            println!("The array is sorted: {}", is_sorted);
            println!("The array length is {}, with k-value: {}, sorted in {:?},", size, k, duration);
        }
    }
}


// Part 1
// This function implements a hybrid sorting algorithm that combines quicksort and insertion sort.
fn quick_hybrid_sort<T: Ord + Clone>(arr: &mut [T], k: usize) -> &mut [T] {
    if arr.len() <= 1 {
        return arr;
    }

    quick_sort(arr, 0, (arr.len() - 1) as isize, k);

    return arr;
}

//recursively call quick sort until subarray reaches k, then call insertion sort
fn quick_sort<T: Ord + Clone>(arr: &mut [T], start: isize, end: isize, k: usize) {
    if start <= end {

        let threshold = end - start + 1;

        if threshold as usize > k {
            let pivot = partition(arr, start, end);

            quick_sort(arr, start, pivot - 1, k);
            quick_sort(arr, pivot + 1, end, k);
        } else {
            insertion_sort(arr);
        }
    }
}

//Divide the quicksort into partitions
fn partition<T: Ord + Clone>(arr: &mut [T], start: isize, end: isize) -> isize {
    let pivot = arr[end as usize].clone();
    let mut i = start - 1;

    for j in start..end {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    let pivot_index = i + 1;
    arr.swap(pivot_index as usize, end as usize);
    return pivot_index;
}


fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {

        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}