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
    let mut val = 0;
    let mut prev_char: char = '\0';

    for (_i, c) in s.chars().enumerate() {
        let mut token_val = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        // Subtract twice the value of the token to account for the previous token value being added
        if prev_char == 'I' && (c == 'V' || c == 'X') {
            token_val -= 2;
        } else if prev_char == 'X' && (c == 'L' || c == 'C') {
            token_val -= 20;
        } else if prev_char == 'C' && (c == 'D' || c == 'M') {
            token_val -= 200;
        }

        prev_char = c;
        val += token_val;
    }

    return val;
}
