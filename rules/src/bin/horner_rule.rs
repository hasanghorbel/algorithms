fn horner_rule(arr: &mut Vec::<u32>, x: u32) -> u32 {
  let mut p = 0;
  for i in (0..arr.len()).rev() {
    p = arr[i] + x * p;
  }
  p
}

fn main() {
    use std::time::Instant;
    let mut arr = vec![1, 2, 3];
    let now = Instant::now();
    let result = horner_rule(&mut arr, 2);
    let elapsed = now.elapsed();
    println!("{:?}", result);
    println!("Elapsed: {:.2?}", elapsed);
}
