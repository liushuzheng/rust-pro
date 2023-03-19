use str_demo::str;

mod str_demo;

fn vec_d(){
    let mut v = vec![1, 2, 3, 4, 5];
    let mut first = &v[0];
    // v.push(6);
    println!("The first element is: {}", first);
}

fn for_vector() {
    let mut v  = vec![1,2,3];
    for i in  &mut v {
        *i += 50;
    }

}

fn main() {
    // str::push_str();
}
