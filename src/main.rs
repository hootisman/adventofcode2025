use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    one();
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

    /* Sort id's from smallest -> largest */
    left_ids.sort();
    right_ids.sort();

    /* Sum All Distances */
    let mut total = 0;

    for i in 0..left_ids.len() {
        total += i32::abs(left_ids[i] - right_ids[i]);
    }

    println!("The total is {}", total);
}