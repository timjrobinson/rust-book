

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Why does this one not contain parenthesis around the values it's taking in?
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        match self {
            Message::Write(word) => println!("value: {}", word),
            Message::ChangeColor(red, green, blue) => println!("New RGB Value: {}, {}, {}", red, green, blue),
            _ => println!("It's not a write!")
        }
    }
}



fn route(ip_kind: IpAddr) {


}

fn print_option(option: Option<i32>) {
    match option {
        Some(data) => println!("value: {}", data),
        None => println!("The data is invalid!")
    }

}

fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let c = Message::ChangeColor(3, 4, 8);
    c.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    print_option(some_number);
    print_option(absent_number);
}
