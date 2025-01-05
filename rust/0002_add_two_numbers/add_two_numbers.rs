fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> = vec![
        (vec![0], vec![0], vec![0]),
        (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
        (
            vec![9, 9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9],
            vec![8, 9, 9, 9, 0, 0, 0, 1],
        ),
    ];

    for (list1, list2, expected) in test_cases {
        let list_node1 = match list1.is_empty() {
            true => None,
            false => Some(Box::new(ListNode::from(&list1))),
        };

        let list_node2 = match list2.is_empty() {
            true => None,
            false => Some(Box::new(ListNode::from(&list2))),
        };

        let result = match add_two_numbers(list_node1, list_node2) {
            None => vec![],
            Some(node) => node.into(),
        };

        println!(
            "list1: {:?}, list2: {:?}, expected: {:?}, actual: {:?}",
            list1, list2, expected, result
        );

        assert!(result.eq(&expected));
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<&Vec<i32>> for ListNode {
    fn from(value: &Vec<i32>) -> Self {
        if value.is_empty() {
            return ListNode::new(0);
        }

        let mut head = Box::new(ListNode::new(value[0]));
        let mut current = &mut head;

        for i in 1..value.len() {
            current.next = Some(Box::new(ListNode::new(value[i])));
            current = current.next.as_mut().unwrap();
        }

        return *head;
    }
}

impl Into<Vec<i32>> for Box<ListNode> {
    fn into(self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = Some(self);
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        return result;
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let val1: i32 = list_to_int(&l1);
    let val2: i32 = list_to_int(&l2);

    let result = val1 + val2;

    return Some(Box::new(int_to_list(result)));
}

const ASCII_ZERO: i32 = 48;

fn list_to_int(node: &Option<Box<ListNode>>) -> i32 {
    let mut result = String::new();
    let mut current = node;

    while let Some(node) = current {
        result.push((node.val + ASCII_ZERO) as u8 as char);
        current = &node.next;
    }

    result = result.chars().rev().collect();

    return match result.parse() {
        Ok(value) => value,
        Err(_) => 0,
    };
}

fn int_to_list(val: i32) -> ListNode {
    let nums: Vec<i32> = val
        .to_string()
        .bytes()
        .rev()
        .map(|b| b as i32 - ASCII_ZERO)
        .collect();

    let mut head = Box::new(ListNode::new(nums[0]));
    let mut current = &mut head;

    for i in 1..nums.len() {
        current.next = Some(Box::new(ListNode::new(nums[i])));
        current = current.next.as_mut().unwrap();
    }

    return *head;
}
