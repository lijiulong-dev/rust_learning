fn main() {
    println!("Hello, world!");

    println!("{}", coin_value(Coin::Penny));
    println!("{}", coin_value(Coin::Quarter(Province::Shanghai)));

    let u: i8 = 3;
    match u {
        1 => println!("{}", 2),
        2 => println!("{}", 4),
        _ => println!("{}", 10),
    }

    if let 4 = u {
        println!("{}", u)
    } else {
        println!("{}", u+1)
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Province),
}

#[derive(Debug)]
enum Province {
    Hangzhou,
    Shanghai, 
}

fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("{}", 1);
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(pro) => {
            println!("{:?}", pro);
            25
        },
    }
}