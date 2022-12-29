use core::cmp::Ordering::*;

pub fn median(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> f64 {
    let total_len = arr1.len() + arr2.len();
    let half = (total_len - 1) / 2;

    // arr2 should be the shorter one
    if arr1.len() < arr2.len() {
        std::mem::swap(&mut arr1, &mut arr2);
    }

    // get all values before and after a possible split assuming we take `num` values from arr2
    let lohi = |num| {
        let arr2_lo = if num > 0 { arr2.get(num - 1) } else { None };
        let arr2_hi = arr2.get(num);

        let arr1_lo = if half >= num {
            arr1.get(half - num)
        } else {
            None
        };
        let arr1_hi = arr1.get(1 + half - num);

        (arr1_lo, arr1_hi, arr2_lo, arr2_hi)
    };

    let eval_index = |index| {
        let (arr1_lo, arr1_hi, arr2_lo, arr2_hi) = lohi(index);
        if arr1_lo > arr2_hi {
            Less
        } else if arr2_lo > arr1_hi {
            Greater
        } else {
            Equal
        }
    };

    // ultimately, we want to find out how many elements of arr2 should be before the median value
    // for each element we move right in arr2, we move one element left in arr1, to keep the number
    // of elements before the median value constant
    let center = binary_search_by_index(&arr2, eval_index);

    // determine the values that are adjacent/equal to the median
    let (arr1_lo, arr1_hi, arr2_lo, arr2_hi) = lohi(center);
    let lo = arr1_lo.max(arr2_lo).unwrap().clone();
    let hi = [arr2_hi, arr1_hi]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(&lo) // in case arr2 is empty
        .clone();

    if total_len % 2 == 1 {
        lo as f64
    } else {
        (lo as f64 + hi as f64) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], vec![1], 1.0)]
    #[case(vec![1,2], vec![3, 4], 2.5)]
    #[case(vec![3,4], vec![1, 2], 2.5)]
    #[case(vec![1,3], vec![2, 4, 5], 3.0)]
    fn solutions(#[case] arr1: Vec<i32>, #[case] arr2: Vec<i32>, #[case] expected: f64) {
        assert_eq!(median(arr1, arr2), expected);
    }
}

// modified version of core lib function
#[inline]
pub fn binary_search_by_index<'a, T, F>(slice: &'a [T], mut f: F) -> usize
where
    F: FnMut(usize) -> core::cmp::Ordering,
{
    // INVARIANTS:
    // - 0 <= left <= left + size = right <= slice.len()
    // - f returns Less for everything in slice[..left]
    // - f returns Greater for everything in slice[right..]
    let mut size = slice.len();
    let mut left = 0;
    let mut right = size;
    while left < right {
        let mid = left + size / 2;
        let cmp = f(mid);

        // The reason why we use if/else control flow rather than match
        // is because match reorders comparison operations, which is perf sensitive.
        // This is x86 asm for u8: https://rust.godbolt.org/z/8Y8Pra.
        if cmp == Less {
            left = mid + 1;
        } else if cmp == Greater {
            right = mid;
        } else {
            return mid;
        }

        size = right - left;
    }

    left
}
