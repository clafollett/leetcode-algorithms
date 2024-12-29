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
        (vec!["cir", "car"], "c"),                                   // leetcode - Failed Test Case
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

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // After a number of attempts at a brute force attack with poor performance, I found this idomatic Rust Solution
    // Ref: https://leetcode.com/problems/longest-common-prefix/discuss/6988/My-4ms-C%2B%2B-solution
    // This code is simply elegant! So much better than they hack in my history! I love Rust!
    let result = strs
        .into_iter()
        .reduce(|a, c| {
            a.chars()
                .zip(c.chars())
                .take_while(|(a, c)| a == c)
                .map(|(c, _)| c)
                .collect()
        })
        .unwrap_or(String::new());

    return result;
}
