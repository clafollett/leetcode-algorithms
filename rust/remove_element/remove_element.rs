fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>, i32)> = vec![
        (vec![], 0, vec![], 0),
        (vec![0], 0, vec![], 0),
        (vec![1], 0, vec![1], 1),
        (vec![3, 2, 2, 3], 3, vec![2, 2], 2),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 4, 0, 3], 5),
    ];

    for (mut nums, val, expected_nums, expected_len) in test_cases {
        // We need to clone the original nums since we mutate the test input
        let original_nums = nums.clone();
        let actual_uniques = remove_element(&mut nums, val);
        let actual_nums = nums[0..actual_uniques as usize].to_vec();

        println!(
            "nums: {:?}, expected_uniques: {}, actual_uniques: {}, expected_nums: {:?}, actual_nums: {:?}",
            original_nums, expected_len, actual_uniques, expected_nums, actual_nums
        );

        assert!(actual_uniques == expected_len);
        assert!(actual_nums.eq(&expected_nums));
    }
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    for i in 0..nums.len() {
        while i < nums.len() && nums[i] == val {
            nums.swap_remove(i);
        }
    }

    return nums.len() as i32;
}
