fn main() {
    let mut test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        (vec![0], 0, 0),
        (vec![0], 1, 1),
        (vec![0, 1], 1, 1),
        (vec![1, 3, 5, 6], 5, 2),
        (vec![1, 3, 5, 6], 2, 1),
        (vec![1, 3, 5, 6], 7, 4),
        // Failed Test Cases
        (vec![1, 3, 5, 6], 0, 0),
    ];

    const LEN: usize = 100;
    let mut long_array = [0; LEN];

    for i in 0..LEN {
        long_array[i] = (i as i32 + 1) * 2;
    }

    test_cases.push((long_array.to_vec(), LEN as i32 * 2, LEN as i32 - 1));

    for (nums, target, expected) in test_cases {
        let result = search_insert(nums.clone(), target);
        println!(
            "nums: {:?}, target: {}, expected: {:?}, actual: {:?}",
            nums, target, expected, result
        );
        assert!(result.eq(&expected));
    }
}

use std::cmp::Ordering;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums[0] >= target {
        return 0;
    }

    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = (left + right) / 2;

        match target.cmp(&nums[mid]) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid + 1,
        }
    }

    return left as i32;
}
