// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

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

    if !flag && up == 0 && num > 0 {
        return Some(Box::new(ListNode { val: num, next: None }));
    }

    if !flag && up == 0 {
        return None;
    }
    if !flag && up > 0 {
        return Some(Box::new(ListNode { val: num, next: Some(Box::new(ListNode { val: up, next: None })) }));
    }


    Some(Box::new(ListNode { val: num, next: add_two_numbers_up(v1, v2, up) }))
}


pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_up(l1, l2, 0)
}


// 输入：l1 = [2,4,3], l2 = [5,6,4]
// 输出：[7,0,8]
// 解释：342 + 465 = 807.

// [9,9,9,9,9,9,9]
// [9,9,9,9]
// [8,9,9,9,0,0,0,1]
#[test]
fn test_f() {
    let l1 = ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode { val: 9, next: None })),
                        })),
                    })),
                })),
            })),
        })),
    };

    let l2 = ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: None,
                })),
            })),
        })),
    };


    let result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    if let Some(i) = result {
        println!("Matched {:?}!", i);
    }
}

