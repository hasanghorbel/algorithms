pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
      ($($name:ident: $test_case:expr,)*) => {
          $(
              #[test]
              fn $name() {
                  let (mut arr, expected) = $test_case;
                  bubble_sort(&mut arr);
                  assert_eq!(arr, expected);
              }
          )*
      };
  }

    test_cases! {
        empty: ([] as [&usize; 0], [] as [&usize; 0]),
        items_desc: ([6, 5, 4, 3, 2, 1], [1, 2, 3, 4, 5, 6]),
        random: ([7, 2, 1, 8, 0, 5], [0, 1, 2, 5, 7, 8]),
    }
}
