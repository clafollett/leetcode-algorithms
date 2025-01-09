fn main() {
    let test_cases: Vec<(&str, &str, &str)> = vec![
        ("11", "1", "100"),
        ("1010", "1011", "10101"),
        ("1111", "1111", "11110"),
    ];

    for (a, b, expected) in test_cases {
        let result = add_binary(a.to_string(), b.to_string());
        println!(
            "nums: {:?}, target: {}, expected: {:?}, actual: {:?}",
            a, b, expected, result
        );
        assert!(result.eq(&expected));
    }
}

pub fn add_binary(a: String, b: String) -> String {
    // No need to convert to the numbers, just work with the chars as bytes
    const ZERO: u8 = b'0';
    const ONE: u8 = b'1';

    // Setup our iterators and current state vars making sure
    // to reverse since we do add math from right to left
    let mut iter_a = a.bytes().rev();
    let mut iter_b = b.bytes().rev();

    let mut cur_a = iter_a.next();
    let mut cur_b = iter_b.next();

    let mut carry = ZERO;
    let mut result = Vec::<u8>::new();

    // Pretty simple, keep iterating while one of the iterators
    // has a value or we are carring over the last ONE
    while cur_a.is_some() || cur_b.is_some() || carry == ONE {
        // If we have reached the end of the iterators, there is
        // only one other option, we are carrying over the ONE
        if cur_a.is_none() && cur_b.is_none() {
            result.push(ONE);
            break;
        }

        let l = match cur_a {
            Some(val) => val,
            None => ZERO,
        };

        let r = match cur_b {
            Some(val) => val,
            None => ZERO,
        };

        // Simple, cheesy if blocks based on bit math rules
        // Rule #1: 0 + 0 = 0 carry 0
        if l == ZERO && r == ZERO {
            let sum = if carry == ZERO { ZERO } else { ONE };
            result.push(sum);
            carry = ZERO;
        // Rule #2: 1 + 1 = 0 carry 1
        } else if l == ONE && r == ONE {
            let sum = if carry == ZERO { ZERO } else { ONE };
            result.push(sum);
            carry = ONE;
        // Rule #1: 1 + 0 OR 0 + 1 = 1 carry 0
        } else {
            let sum = if carry == ZERO { ONE } else { ZERO };
            result.push(sum);
            // Leave carry untouched and let it `carry` over
        }

        // Don't forget to iterate...
        cur_a = iter_a.next();
        cur_b = iter_b.next();
    }

    result.reverse();

    return String::from_utf8(result).unwrap();
}
