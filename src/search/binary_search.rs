use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
  let is_asc = is_asc_arr(arr);

  let mut left = 0;
  let mut right = arr.len();

  while left < right {
      if match_compare(item, arr, &mut left, &mut right, is_asc) {
          return Some(left);
      }
  }

  None
}

fn match_compare<T: Ord>(
  item: &T,
  arr: &[T],
  left: &mut usize,
  right: &mut usize,
  is_asc: bool,
) -> bool {
  let mid = *left + (*right - *left) / 2;
  let cmp_result = item.cmp(&arr[mid]);

  match (is_asc, cmp_result) {
      (true, Ordering::Less) | (false, Ordering::Greater) => {
          *right = mid;
      }
      (true, Ordering::Greater) | (false, Ordering::Less) => {
          *left = mid + 1;
      }
      (_, Ordering::Equal) => {
          *left = mid;
          return true;
      }
  }

  false
}

fn is_asc_arr<T: Ord>(arr: &[T]) -> bool {
  arr.len() > 1 && arr[0] < arr[arr.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (item, arr, expected) = $test_case;
                    assert_eq!(binary_search(&item, arr), expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        items_asc: ("c", &["a", "b", "c", "d"], Some(2)),
        items_desc: (5, &[11, 9, 6, 5, 3, 1], Some(3)),
    }
}