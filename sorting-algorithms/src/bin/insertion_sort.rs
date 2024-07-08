// O(n^2)

fn insertion_sort(arr: &mut Vec<u32>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    use std::time::Instant;
    let mut arr = vec![5, 2, 4, 6, 1, 3];
    let now = Instant::now();
    insertion_sort(&mut arr);
    let elapsed = now.elapsed();
    println!("{:?}", arr);
    println!("Elapsed: {:.2?}", elapsed);
}
