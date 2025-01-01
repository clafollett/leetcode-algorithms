use std::vec;

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> = vec![
        (vec![], vec![], vec![]),
        (vec![], vec![0], vec![0]),
        (vec![0], vec![1], vec![0, 1]),
        (vec![1], vec![0], vec![0, 1]),
        (vec![0, 1], vec![], vec![0, 1]),
        (vec![], vec![0, 1], vec![0, 1]),
        (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
        // Failed Publish Test Cases
        (vec![5], vec![1, 2, 4], vec![1, 2, 4, 5]),
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

        let result = merge_two_lists(list_node1, list_node2);
        let result_vec: Vec<i32> = match result.is_none() {
            true => vec![],
            false => result.unwrap().into(),
        };

        println!(
            "list1: {:?}, list2: {:?}, expected: {:?}, result: {:?}",
            list1, list2, expected, result_vec
        );

        assert!(result_vec.eq(&expected));
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

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // This is my initial solution which is not as efficient as the one below
    // I'm not worthy of the one below but my solution works and scored a 100% @ 0ms
    // but it only scored a 23% on memory
    // https://leetcode.com/problems/merge-two-sorted-lists/submissions/1492957621/

    // // No point going further if either the first or second list is empty
    // if list1.is_none() {
    //     return list2;
    // }

    // if list2.is_none() {
    //     return list1;
    // }

    // let mut vec1: Vec<i32> = list1.unwrap().into();
    // let mut vec2: Vec<i32> = list2.unwrap().into();
    // let mut result = Vec::new();

    // result.append(&mut vec1);
    // result.append(&mut vec2);
    // result.sort();

    // return Some(Box::new(ListNode::from(&result)));

    // Barrowed from here:
    // https://leetcode.com/problems/merge-two-sorted-lists/solutions/2947855/simple-and-efficient-rust-8-liner
    // I did not submit this solution but do want to keep a copy of it for future reference. this is some SWEET code!!!
    // I renamed the vars to make it clearer for me to read and added comments to explain what's happening.
    let mut current = &mut list1;

    while list2.is_some() {
        // if list1 was empty to start with or the value of list2 is less than the value of list1
        if current.is_none() || list2.as_ref()?.val < current.as_ref()?.val {
            // move head of list2 to the front of list1
            std::mem::swap(current, &mut list2);
        }
        // regardless of which list was moved to the front, we need to move the current pointer to the next node
        current = &mut current.as_mut()?.next;
    }

    return list1;
}
