fn main() {
    let z = another_function(5);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32) -> i64 {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    return y + 8;
}