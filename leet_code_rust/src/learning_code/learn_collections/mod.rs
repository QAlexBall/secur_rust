mod learn_vector;
mod learn_string;
mod learn_hashmap;

pub fn run_learn_collections() {
    println!("===> Start running learning collections!");
    learn_vector::run_learn_vector();
    learn_string::run_learn_string();
    learn_hashmap::run_learn_hashmap();
    println!("===> End running learning collections!\n");
}