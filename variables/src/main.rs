fn main() {
    // mut_test();
    type_guess();
}

fn type_guess() {
    let guess: u32 = "43".parse().expect("Not a number!");
    println!("guess number is {}", guess)
}

fn mut_test() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable `x`
    x = 6;
    println!("The value of x is: {}", x);
}
