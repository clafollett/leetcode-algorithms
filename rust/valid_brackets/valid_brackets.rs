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

// Refactored original to use a more Rust idiomatic wayx
pub fn is_valid(s: String) -> bool {
    if s.len() == 0 || s.len() % 2 != 0 {
        return false;
    }

    let mut stack = Vec::new();

    for c in s.bytes() {
        match c {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b')' | b']' | b'}' if stack.pop() != Some(c) => return false,
            _ => (),
        }
    }

    return stack.is_empty();
}
