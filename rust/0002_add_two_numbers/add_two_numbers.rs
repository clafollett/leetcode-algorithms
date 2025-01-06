fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> = vec![
        (vec![0], vec![0], vec![0]),
        (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
        (
            vec![9, 9, 9, 9, 9, 9, 9],
            vec![9, 9, 9, 9],
            vec![8, 9, 9, 9, 0, 0, 0, 1],
        ),
        // Failed test case
        (
            vec![9],
            vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ),
        (
            vec![
                2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2,
                4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
                3, 2, 4, 3, 9,
            ],
            vec![
                5, 6, 4, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2,
                4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
                3, 9, 9, 9, 9,
            ],
            vec![
                7, 0, 8, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4,
                8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8,
                6, 1, 4, 3, 9, 1,
            ],
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
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;
    let mut list1 = &mut list1;
    let mut list2 = &mut list2;
    let mut result = Some(Box::new(ListNode::new(0)));
    let mut head = &mut result;
    let mut remainder = 0;

    while list1.is_some() || list2.is_some() || remainder > 0 {
        let mut sum = remainder;

        if let Some(node) = list1 {
            sum += node.val;
            list1 = &mut node.next;
        }

        if let Some(node) = list2 {
            sum += node.val;
            list2 = &mut node.next;
        }

        remainder = sum / 10;

        if let Some(node) = head {
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            head = &mut node.next;
        }
    }

    return result.unwrap().next;
}
