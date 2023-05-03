use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}
// fn main() {
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
// }

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[test]
fn add() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}


fn test1() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

fn men() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

#[test]
fn test2() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 12;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {}


fn split() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid],
//      &mut slice[mid..])
// }

fn other() {
    let address = 0x01234usize;
    let r = address as *mut i32;
    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}


use advanced::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}