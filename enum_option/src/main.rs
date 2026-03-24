fn main() {
    println!("Hello, world!");

    // option<t> 和 t
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z = y.unwrap();
    let z2 = y.expect("null");

    let sum = x + z + z2;
    println!("{}", sum);

    let five = Some(5);
    let six = plus_one(five);
    println!("{}", six.unwrap());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}