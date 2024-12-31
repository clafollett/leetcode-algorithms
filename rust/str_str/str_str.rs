fn main() {
    let test_cases: Vec<(&str, &str, i32)> = vec![
        ("a", "b", -1),
        ("sadbutsad", "sad", 0),
        ("leetcode", "leeto", -1),
        ("abcdefghijklmnopqrstuvwxyz", "lmnop", 11),
        // Failed test case
        ("abb", "abaaa", -1),
    ];

    for (haystack, needle, expected) in test_cases {
        let result = str_str(haystack.to_string(), needle.to_string());
        println!(
            "haystack: {:?}, needle: {:?}, expected: {}, actual: {}",
            haystack, needle, expected, result
        );
        assert!(result == expected);
    }
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let needle_len = needle.len();
    let haystack_len = haystack.len();

    if needle_len > haystack_len {
        return -1;
    }

    for i in 0..(haystack_len - needle_len + 1) {
        if haystack[i..i + needle_len] == needle {
            return i as i32;
        }
    }

    return -1;
}
