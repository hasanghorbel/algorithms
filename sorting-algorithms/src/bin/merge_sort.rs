// O(n log n)

fn merge(arr: &mut Vec<u32>, p: usize, q: usize, r: usize) {
    let nl = q - p + 1; // length of A[p:q]
    let nr = r - q; // length of A[q+1:r]
    let left = &arr[p..=q].to_vec(); // copy A[p:q] to L[0:nl-1]
    let right = &arr[q + 1..=r].to_vec(); // copy A[q+1:r] to R[0:nr-1]

    let mut i = 0; // i indexes the smallest remaining element in L
    let mut j = 0; // j indexes the smallest remaining element in R
    let mut k = p; // k indexes the location in A to fill

    // As long as each of the arrays L and R contains an unmerged element,
    // copy the smallest unmerged element back into A[p:r]
    while i < nl && j < nr {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    // Having gone through one of L and R entirely, copy the
    // remainder of the other to the end of A[p:r]
    while i < nl {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < nr {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut Vec<u32>, p: usize, r: usize) {
    if p >= r {
        // zero or one element?
        return;
    }
    let q = (p + r) / 2; // midpoint of A[p:r]
    merge_sort(arr, p, q); // recursively sort A[p:r]
    merge_sort(arr, q + 1, r); // recursively sort A[q+1:r]

    // Merge A[p:q] and A[q+1:r] into A[p:r]
    merge(arr, p, q, r);
}

fn main() {
    use std::time::Instant;
    let mut arr = vec![2, 4, 6, 7, 9, 1, 2, 3, 5];
    let n = arr.len() - 1;
    let now = Instant::now();
    merge_sort(&mut arr, 0, n);
    let elapsed = now.elapsed();
    println!("{:?}", arr);
    println!("Elapsed: {:.2?}", elapsed);
}
