fn main() {
    let test_cases: Vec<(Vec<&str>, &str)> = vec![
        (vec![], ""),               // No strings, no common prefix.
        (vec!["match"], "match"),   // Single string, common prefix is the string.
        (vec!["notempty", ""], ""), // Test sorting and bail early on empty string
        (vec!["ab", "a"], "a"),     // Test sorting and shortest match
        (vec!["abc1234567", "abc1234XYZ", "abc1234321"], "abc1234"), // Test sorting and shortest match
        (vec!["flower", "flow", "flight"], "fl"),                    // leetcode match example
        (vec!["dog", "racecar", "car"], ""),                         // leetcode no match example
        (vec!["ab", "a"], "a"),                                      // leetcode - Failed Test Case
    ];

    for (strs, expected) in test_cases {
        let strings: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
        let result = longest_common_prefix(strings.clone());
        println!(
            "strings: {:?}, result: {:?} expected: {:?}",
            strings, result, expected
        );
        assert!(result == expected);
    }
}

pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    // If there are no strings, why continue???
    if strs.len() == 0 {
        return "".to_string();
    }

    // If there is only one string, it's a no brainer regardless of content
    if strs.len() == 1 {
        return strs[0].clone();
    }

    // Sort so we can find the first mismatch faster.
    strs.sort();

    let mut str_iter = 0;

    // TODO: Research if `rustc` will optimize this `len()` call away for me in the `while` loop.
    let strs_len = strs.len();
    let mut max_common_len = 0;

    while str_iter < strs_len {
        // If we are at the end of the list, we are done.
        // We compared it in the previous iteration
        if str_iter + 1 == strs_len {
            break;
        }

        let mut k = 0;
        let lh = strs[str_iter].as_str();
        let rh = strs[str_iter + 1].as_str();

        while k < lh.len() {
            if lh.chars().nth(k) != rh.chars().nth(k) {
                break;
            }

            k += 1;
        }

        if k == 0 {
            return "".to_string();
        }

        if k < max_common_len || max_common_len == 0 {
            max_common_len = k;
        }

        str_iter += 1;
    }

    assert!(max_common_len > 0);

    return strs[0][..max_common_len].to_string();
}
