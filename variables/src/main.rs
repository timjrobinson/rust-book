fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let a = 2.0;
    let b: f64 = 3.0;
    let c = b - a;
    println!("The value of c is: {}", c);

    let tup = (3, 5, 8.8);
    let (t, u, p) = tup;
    println!("The value of u is {}", tup.2);

    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("The array is: {}", arr[0]);
}
