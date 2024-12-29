fn main() {
    let test_cases: Vec<(String, i32)> = vec![
        ("III".to_string(), 3),
        ("IV".to_string(), 4),
        ("IX".to_string(), 9),
        ("LVIII".to_string(), 58),
        ("MCMXCIV".to_string(), 1994),
    ];

    for (num, expected) in test_cases {
        let result = roman_to_int(num.clone());
        assert!(result == expected);
        println!(
            "test case: {}, result: {}, expected: {}",
            num, result, expected
        );
    }
}

pub fn roman_to_int(s: String) -> i32 {
    for (i, c) in s.chars().enumerate() {
        println!("i: {}, c: {}", i, c);
    }

    return 0;
}
