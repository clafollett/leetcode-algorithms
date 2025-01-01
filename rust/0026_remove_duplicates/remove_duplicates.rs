fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![], 0, vec![]),
        (vec![0], 1, vec![0]),
        (vec![1, 1, 2], 2, vec![1, 2]),
        (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5, vec![0, 1, 2, 3, 4]),
    ];

    for (mut nums, expected_uniques, expected_nums) in test_cases {
        // We need to clone the original nums since we mutate the test input
        let original_nums = nums.clone();
        let actual_uniques = remove_duplicates(&mut nums);
        let actual_nums = nums[0..actual_uniques as usize].to_vec();

        println!(
            "nums: {:?}, expected_uniques: {}, actual_uniques: {}, expected_nums: {:?}, actual_nums: {:?}",
            original_nums, expected_uniques, actual_uniques, expected_nums, actual_nums
        );

        assert!(actual_uniques == expected_uniques);
        assert!(actual_nums.eq(&expected_nums));
    }
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut head = 0;

    for i in 1..nums.len() {
        if nums[i] == nums[head] {
            continue;
        }

        head += 1;
        nums[head] = nums[i];
    }

    return head as i32 + 1 as i32;
}
