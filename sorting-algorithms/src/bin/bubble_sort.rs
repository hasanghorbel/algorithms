// O(n^2)

fn bubble_sort(arr: &mut Vec<u32>) {
  let mut swapped = true;

  while swapped {
      swapped = false;
      for i in 0..arr.len()-1 {
          if arr[i] > arr[i + 1] {
              arr.swap(i, i + 1);
              swapped = true;
          }
      }
  }

}

fn main() {
  use std::time::Instant;
  let mut arr = vec![5, 2, 4, 6, 1, 3];
  let now = Instant::now();
  bubble_sort(&mut arr);
  let elapsed = now.elapsed();
  println!("{:?}", arr);
  println!("Elapsed: {:.2?}", elapsed);
}
