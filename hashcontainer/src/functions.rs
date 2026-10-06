use core::num;
use std::collections::{HashMap, HashSet};

pub fn count_words(text:&str)-> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    word_count
}

pub fn emp_database() {
    let mut employees: HashMap<i32, String> = HashMap::new();
    employees.insert(1, "Alice".to_string());
    employees.insert(2, "Bob".to_string());
    employees.insert(3, "Charlie".to_string());
    employees.insert(4, "Diana".to_string());
    employees.insert(5, "Eve".to_string());


    if let Some(name) = employees.get(&1) {
        println!("First employee: {}", name);
    } else {
        println!("No employees found.");
    }
}

pub fn convertToHashSet() {
    let names = vec!["Alice", "Bob", "Charlie", "Alice", "Eve"];
    let unique_names: HashSet<_> = names.into_iter().collect();

    println!("Unique names: {:?}", unique_names);
}
