fn main() {
    let test_cases: Vec<(&str, i32)> = vec![
        ("word", 4),
        ("Hello World", 5),
        ("   fly me   to   the moon  ", 4),
        ("luffy is still joyboy", 6),
    ];

    for (val, expected) in test_cases {
        let result = length_of_last_word(val.to_string());
        println!(
            "string: {:?}, result: {}, expected: {}",
            val, result, expected
        );
        assert!(result == expected);
    }
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut len: i32 = 0;
    let mut iter = s.bytes().rev();

    while let Some(c) = iter.next() {
        if c != b' ' {
            len += 1;
            continue;
        }

        if c == b' ' && len > 0 {
            return len;
        }
    }

    return len;
}
