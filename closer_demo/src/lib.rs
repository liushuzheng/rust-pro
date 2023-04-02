pub fn add(left: usize, right: usize) -> usize {
    left + right
}


fn cal(a: u32, b: u32, opt: fn(u32, u32) -> u32) -> u32 {
    opt(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn m() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }

    #[test]
    fn iterator() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
        println!("{total}")
    }

    #[test]
    fn add_one() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let map = v1.iter().map(|x| x + 1);
        let x1: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        for x in x1.iter() {
            println!("{x}")
        }
    }
}

// iterator operation

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

