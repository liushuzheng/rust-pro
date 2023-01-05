fn main() {
    // loop_d();
    // while_d();
    for_i();
}

fn loop_d() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter value is {}", counter);
    println!("result value is {}", result);
}

fn while_d() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}

fn for_d() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_i(){
   for number in (1..4).rev(){
       println!("{}!", number);
   }
    println!("LIFTOFF!!!");
}
