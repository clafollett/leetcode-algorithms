fn main() {
    let test_cases: Vec<(&str, i32)> = vec![
        ("III", 3),
        ("IV", 4),
        ("IX", 9),
        ("LVIII", 58),
        ("MCMXCIV", 1994),
    ];

    for (num, expected) in test_cases {
        let result = roman_to_int(num.to_string());
        println!(
            "test case: {}, result: {}, expected: {}",
            num, result, expected
        );
        assert!(result == expected);
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut val = 0;
    let mut prev_char: char = '\0';

    for c in s.chars() {
        // Get the current tokens value...
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

        // If our previous numeral is also a decrementer, check to see
        // if the current numeral is target. Subtract double decrementer's
        // token value to account the previous and current iterations.
        // It all comes out in the wash!!!
        token_val -= match prev_char {
            'I' if c == 'V' || c == 'X' => 2,
            'X' if c == 'L' || c == 'C' => 20,
            'C' if c == 'D' || c == 'M' => 200,
            _ => 0,
        };

        prev_char = c;
        val += token_val;
    }

    return val;
}
