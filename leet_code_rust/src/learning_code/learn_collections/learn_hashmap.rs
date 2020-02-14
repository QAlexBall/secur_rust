use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

fn use_hashmap() {
    // Create new HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // use list to create HashMap
    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];
    let scores1: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // HashMap and ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
//    println!("{}", field_name)
//                   ^^^^^^^^^^ value borrowed here after move

    // visit HashMap's value
    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    println!("key {} : value {:?}", team_name, score);

    // Traversal
    traversal_hashmap(&scores);

    // Update HashMap
    scores.insert(String::from("blue"), 25);
    traversal_hashmap(&scores);

    // insert when key hasn't value
    scores.entry(String::from("yellow")).or_insert(50);
    println!("{:?}", scores);

    // update new value by old value
    let text = "hello world wonderful rust";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
    }
    traversal_hashmap(&map);
}

fn traversal_hashmap<K: Debug + Eq + Hash, V: Debug>(my_map: &HashMap<K, V>) {
    println!("Start traversal hash map {:?}", my_map);
    for (key, value) in my_map {
        println!("{:?}: {:?}", key, value);
    }
}

pub fn run_learn_hashmap() {
    println!("===> Start running learn hashmap!");
    use_hashmap();
    println!("===> End   running learn hashmap!\n");
}