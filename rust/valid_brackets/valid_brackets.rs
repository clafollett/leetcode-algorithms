fn main() {
    let test_cases: Vec<(&str, bool)> = vec![
        ("", false),
        ("((", false),
        ("))", false),
        ("()", true),
        ("()[]{}", true),
        ("(]", false),
        ("([])", true),
        ("([{}}])", false),
    ];

    for (case, expected) in test_cases {
        let result = is_valid(case.to_string());
        println!(
            "test case: {:?}, result: {:?}, expected: {:?}",
            case, result, expected
        );
        assert!(result == expected);
    }
}

pub fn is_valid(s: String) -> bool {
    // Must at least be an even length to have all matching pairs
    if s.len() == 0 || s.len() % 2 != 0 {
        return false;
    }

    let mut open_bracket_stack: Vec<u8> = Vec::new();

    for c in s.bytes() {
        if c == b'(' || c == b'[' || c == b'{' {
            open_bracket_stack.push(c);
            continue;
        }

        let is_valid_close_bracket = match open_bracket_stack.pop() {
            Some(open_bracket) => match c {
                b')' => open_bracket == b'(',
                b']' => open_bracket == b'[',
                b'}' => open_bracket == b'{',
                _ => false,
            },
            None => false,
        };

        if !is_valid_close_bracket {
            return false;
        }
    }

    return open_bracket_stack.is_empty();
}
