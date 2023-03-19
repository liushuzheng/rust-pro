use std::collections::HashMap;

fn new_hash() {
    // let a ="a";
    // let str  =  String::from("hello");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn hash_collect() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let iter = teams.iter();
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> =
        teams.iter().zip(initial_scores.iter()).collect();
}

fn get_key() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}
#[test]
fn entry_d(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}