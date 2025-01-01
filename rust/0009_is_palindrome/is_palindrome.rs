fn main() {
    let test_cases: Vec<(i32, bool)> = vec![
        (0, true),
        (1, true),
        (11, true),
        (121, true),
        (1221, true),
        (123, false),
        (12321, true),
        (123321, true),
        (123456, false),
        (1234567, false),
        (12345678, false),
        (12345789, false),
    ];

    for (num, expected) in test_cases {
        let result = is_palindrome(num);
        assert!(result == expected);
        println!("num: {}, is_palindrome: {}", num, result);
    }
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x >= 0 && x < 10 {
        return true;
    }

    let x_str = x.to_string();
    return x_str.chars().eq(x_str.chars().rev());
}
