// 两数想加

// 给你两个非空的链表，表示两个非负的整数。它们每位数字都是按照逆序的方式存储的，并且每个节点只能存储一位数字。
//
// 请你将两个数相加，并以相同形式返回一个表示和的链表。
//
// 你可以假设除了数字 0 之外，这两个数都不会以 0开头。


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            next,
            val,
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        add_two_numbers_up(l1, l2, 0)

}

// 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// 输出：[8,9,9,9,0,0,0,1]

pub fn add_two_numbers_up(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut up: i32) -> Option<Box<ListNode>> {
    let mut flag = false;
    let mut v1: Option<Box<ListNode>> = None;
    let mut v2: Option<Box<ListNode>> = None;
    if let Some(v) = l1 {
        up += v.val;
        v1 = v.next;
        flag = true;
    }

    if let Some(v) = l2 {
        up += v.val;
        v2 = v.next;
        flag = true;
    }
    let num = up % 10;
    let up = up / 10;

    if !flag && up == 0 {
        return None;
    }
    if flag && up > 0 {
        return Some(Box::new(ListNode::new(num, Some(Box::new(ListNode::new(up, None))))));
    }

    Some(Box::new(ListNode::new(num, add_two_numbers_up(v1, v2, up))))
}


// 输入：l1 = [2,4,3], l2 = [5,6,4]
// 输出：[7,0,8]
// 解释：342 + 465 = 807.
#[test]
fn test_f() {
    let l1 = ListNode::new(2, Some(Box::new(ListNode::new(4, Some(Box::new(ListNode::new(3, None)))))));
    let l2 = ListNode::new(7, Some(Box::new(ListNode::new(0, Some(Box::new(ListNode::new(8, None)))))));


    let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    if let Some(i) = result {
        println!("Matched {:?}!", i);
    }
}

