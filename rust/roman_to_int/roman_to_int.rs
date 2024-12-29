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
    // Refactored version based on further studies. Converted for loop
    // to an `rfold` call with an accumlator. This works because we are
    // adding from right to left so when we meet a token that fits our rule
    // we can just do a straight up subtraction. Roman Numerals will never all
    // a lesser valued numeral on the left of a higher one unless its to notate a subtraction
    return s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 100 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    });
}
