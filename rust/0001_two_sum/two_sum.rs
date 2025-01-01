fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
        (
            vec![4, 13, 19, 45, 179, 192, 253, 284, 429, 1287, 2947],
            537,
            vec![6, 7],
        ),
    ];

    for (nums, target, expected) in test_cases {
        let result = two_sum(nums.clone(), target);
        println!(
            "nums: {:?}, target: {}, expected: {:?}",
            nums, target, expected
        );
        assert!(result.eq(&expected));
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let nums_len = nums.len() as i32;

    while i < nums_len {
        let mut j = i + 1;

        while j < nums_len {
            if nums[i as usize] + nums[j as usize] == target {
                return [i, j].to_vec();
            }

            if nums[j as usize - 1] + nums[j as usize] == target {
                return [j - 1, j].to_vec();
            }

            j += 1;
        }

        i += 1;
    }

    return vec![];
}
