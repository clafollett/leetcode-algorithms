fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        (vec![0], 0, 0),
        (vec![0], 1, 1),
        (vec![0, 1], 1, 1),
        (vec![1, 3, 5, 6], 5, 2),
        (vec![1, 3, 5, 6], 2, 1),
        (vec![1, 3, 5, 6], 7, 4),
    ];

    for (nums, target, expected) in test_cases {
        let result = search_insert(nums.clone(), target);
        println!(
            "nums: {:?}, target: {}, expected: {:?}",
            nums, target, expected
        );
        assert!(result.eq(&expected));
    }
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums[0] == target {
        return 0;
    }

    let n = nums.len();

    for i in 0..n - 1 {
        if nums[i] == target {
            return i as i32;
        }

        if (nums[i + 1] == target) || (nums[i] < target && nums[i + 1] > target) {
            return i as i32 + 1;
        }
    }

    return n as i32;
}
