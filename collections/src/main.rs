use std::collections::{HashMap,HashSet};

fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(444);

    println!("Vector item 1 is: {}", v[1]);

    let v2 = vec![6,7,8,9];

    let mut third: &i32 = &v2[2];
    println!("The third element is {}", third);

    third = &3;

    println!("After change, third element is {}", third);
    println!("After change, third element of vec is {}", v2[2]);


    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }

    printvec(&v3);
 


}

fn strings() {

    let s = String::from("mew mew");
    println!("S is: {}", s);
    
    let data = "initial contents";
    
    let s = &data;

    let mut y = data.to_string();
    y.push_str(" yum");
    y = y + " more!";

    println!("S is: {}, y is: {}", s, y);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = return_formatted(&s1, &s2, &s3);
    println!("s: {}", s);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s: {}", s);

}

fn hashmaps() {


    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("hashmap: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // println!("Field name: {}", field_name);
    // println!("hashmap: {:?}", map);

    // field_name = String::from("Blah");

    // println!("Field name: {}", field_name);
    // println!("hashmap: {:?}", map);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(score) => { println!("Score: {}", score); },
        None => println!("No score!"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);



}

fn summary() {
    let vec = vec![1, 3, 4, 4, 12, 19, 22, 29];
    let details = list_vec_details(vec);
    println!("Vector Details: {:?}", details);
}

fn list_vec_details(mut vec: Vec<i32>) -> HashMap<String, f32> {
    let mut sum = 0;
    for i in &vec {
        sum += i;
    }
    let len = vec.len() as i32;
    let mean : f32 = (sum as f32 / len as f32) as f32;
    let halfway : usize = (len / 2) as usize;

    vec.sort();
    let median = vec[halfway] as f32;

    let mut highest_count = (0, 0);
    let mut counts = HashMap::new();
    for i in &vec {
        let count = counts.entry(i.to_string()).or_insert(0);
        *count += 1;
        if *count > highest_count.1 {
            highest_count.0 = *i;
            highest_count.1 = *count;
        }
    }

    let mut result = HashMap::new();
    result.insert(String::from("mean"), mean);
    result.insert(String::from("median"), median);
    result.insert(String::from("mode"), highest_count.0 as f32);
    return result;
}

fn printvec(v3: &Vec<i32>) {
    for i in v3 {
        println!("{}", i);
    }
}

fn return_formatted(s1: &String, s2: &String, s3: &String) -> String {
    let s = format!("{}, {}, {}", s1, s2, s3);
    return s;

}

fn convert_to_piglatin(s: &String) -> String {
    let split_string: Vec<&str> = s.split(" ").collect();
    let mut piglatin_string = String::from("");
    for word in &split_string {
        let first_char = &word[0..1];
        let piglatin_word = match first_char {
            "a" | "e" | "i" | "o" | "u" => format!("{}-hay ", &word),
            _ => format!("{}-{}ay ", &word[1..], first_char),
        };
        piglatin_string.push_str(piglatin_word.as_str());
    }

    return piglatin_string;
}

fn piglatin_test() {
    let original = String::from("first time to eat an apple");
    let converted = convert_to_piglatin(&original);
    println!("Pig Latin string is: {}", converted);
}

fn parse_instruction(instruction: &String) -> (&str, &str) {
    let split_instructions : Vec<&str> = instruction.split(" ").collect();
    let name: &str = split_instructions[1];
    let department: &str = split_instructions[3];
    return (name, department);
}

fn employee_interface() {
    let mut instructions: Vec<String> = Vec::new();
    instructions.push(String::from("Add Sally to Engineering"));
    instructions.push(String::from("Add Amir to Sales"));
    instructions.push(String::from("Add Toby to Marketing"));
    instructions.push(String::from("Add Juanita to Sales"));
    instructions.push(String::from("Add Amir to Sales"));
    let mut employee_db : HashMap<String, HashSet<String>> = HashMap::new();
    for i in &instructions {
        let (name, department) = parse_instruction(i);
        let department = employee_db.entry(String::from(department)).or_insert(HashSet::new());
        department.insert(String::from(name));
    }
    println!("Employee DB: {:?}", employee_db);
}

fn main() {
    vectors();
    strings();
    hashmaps();
    summary();
    piglatin_test();
    employee_interface();
}