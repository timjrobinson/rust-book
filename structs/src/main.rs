fn main() {
    println!("Hello, world!");

    struct Point(i32, i32, i32);

    struct User {
        username: String,
        email: String,
        active: bool,
        pos: Point
    };

    let me = User {
        username: String::from("timjrobinson"),
        email: String::from("tim@timjrobinson.com"),
        active: true,
        pos: Point(0,0,5)
    };

    println!("Me: {}", me.username);
    println!("My location: {}", me.pos.2);

}
