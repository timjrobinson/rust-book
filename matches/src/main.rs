#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Nebraska,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State is: {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = Coin::Nickel;
    let stateCoin = Coin::Quarter(UsState::Nebraska);
    println!("Value of coin is: {}", value_in_cents(coin));
    println!("Value of stateCoin is: {}", value_in_cents(stateCoin));
   

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = Some(3u8);
    // if let Some(3) = some_u8_value { // Why is this a single equals instead of double?
    //     println!("three");
    // }

    if Some(3) == some_u8_value {
        println!("three");
    }


    println!("All done!");
}
