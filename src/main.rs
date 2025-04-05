/*
    Semester Project Hybrid Sort / Intergalactic Zoo Trip (Knapsack Problem)
    By Aidan Burchett and Geraldo
*/

fn main() {
    // Example usage of the QuickHybridSort function
    //let mut arr = vec![5, 2, 9, 1, 5, 6, 3, 7, 8, 4, 0, 34, 62, 12, 52, 23, 15, 33, 65, 89, 11, 51, 45];
    let mut arr1 = vec![8, 2, 4, 7, 1, 3, 9, 6, 5];
    println!("Original array: {:?}", arr1);
    let k = 3;
    let sorted_arr = quick_hybrid_sort(&mut arr1, k);
    println!("After pivot final array: {:?}", sorted_arr);
}


// Part 1
// This function implements a hybrid sorting algorithm that combines quicksort and insertion sort.
fn quick_hybrid_sort<T: Ord>(arr: &mut [T], k: usize) -> &mut [T] {
    let len = arr.len();
    let mut pivot = len - 1;
    let mut i: isize = -1;
    let mut j = 0;

    while j < arr.len() {
        //condition to check if pivot number is greater than j
        if arr[j] < arr[pivot] {
            i += 1;
            //perform swap
            arr.swap(i as usize, j);
        }
        j += 1;
    }
    //pivot has been reached swap pivot and i + 1
    i += 1;
    arr.swap(i as usize, pivot);

    
    return arr;
}