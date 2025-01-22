use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    two();
}

fn one(){
    let mut file = File::open("input").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read to string");


    /* Parse Strings -> Integer Vectors */
    let mut left_ids: Vec<i32> = Vec::new();
    let mut right_ids: Vec<i32> = Vec::new();

    /* Dict for counting how many times right id is used */
    let mut id_counter: HashMap<i32, i32>= HashMap::new();

    for line in contents.lines(){
        // split spaces + to vector
        let splitted = line.split(' ');
        let mut collection: Vec<&str> = splitted.collect();

        // str
        let str_1 = collection[0];
        let str_2= collection.pop().unwrap();

        // str -> i32
        let num_1 = str_1.parse::<i32>().expect("Integer Parse Error");
        let num_2 = str_2.parse::<i32>().expect("Integer Parse Error");

        
        left_ids.push(num_1);
        right_ids.push(num_2);

        //increment right id count
        let old_count = id_counter.get(&num_2);
        id_counter.insert(num_2, old_count.unwrap_or(&0) + 1);
    }

    /* Similarity score; sim_score += left_id * occurances in right list */
    let mut sim_score = 0;
    for id in left_ids.iter(){
        match id_counter.get(id) {
            Some(val) => sim_score += id * val,
            None => (),
        }
    }

    println!("sim score is {}", sim_score);


}

fn two(){
    let mut file = File::open("input").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read to string");

    let mut safe = 0;
    for line in contents.lines(){
        let splitted = line.split(' ');
        let mut is_safe: bool = true;
        splitted.for_each(|x| );
        
    }
}