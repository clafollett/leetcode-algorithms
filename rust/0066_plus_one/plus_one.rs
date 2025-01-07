fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
        (vec![1, 2, 3], vec![1, 2, 4]),
        (vec![4, 3, 2, 1], vec![4, 3, 2, 2]),
        (vec![9], vec![1, 0]),
        (
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ),
    ];

    for (digits, expected) in test_cases {
        let result = plus_one(digits.clone());
        println!(
            "nums: {:?}, expected: {:?}, actual: {:?}",
            digits, expected, result
        );
        assert!(result.eq(&expected));
    }
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut remainder = 0;

    let len = digits.len();

    for i in (0..len).rev() {
        let mut val = digits[i] + remainder;

        if i == len - 1 {
            val += 1;
        }

        if val >= 10 {
            val -= 10;
            remainder = 1;
        } else {
            remainder = 0;
        }

        digits[i] = val;
    }

    if remainder > 0 {
        digits.insert(0, 1);
    }

    return digits;
}
