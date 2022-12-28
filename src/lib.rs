pub fn median(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> f32 {
    arr1.append(&mut arr2);
    arr1.sort();

    let total_len = arr1.len() + arr2.len();
    let middle = total_len / 2;
    if total_len % 2 == 1 {
        arr1[middle] as f32
    } else {
        (arr1[middle] as f32 + arr1[middle - 1] as f32) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2], vec![3, 4], 2.5)]
    #[case(vec![1,3], vec![2, 4, 5], 3.0)]
    fn solutions(#[case] arr1: Vec<i32>, #[case] arr2: Vec<i32>, #[case] expected: f32) {
        assert_eq!(median(arr1, arr2), expected);
    }
}
