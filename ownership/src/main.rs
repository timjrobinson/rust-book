fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..];

    println!("slice: {:?}", slice);

    
}
