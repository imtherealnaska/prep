fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..arr.len() {
        let mut swapped = false;

        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
