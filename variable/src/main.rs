fn main() {
    let x = 1;
    let x = x + 2;
    let x = x * 2;
    println!("x = {}", x);
    println!("Hello, world!");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("space = {}", spaces);

    let x = 1.1;
    let y: f64 = 2.2;
    println!("x = {}", x);
    println!("y = {}", y);

    let tup :(i32, f32, i64) = (1, 2.2, 10);
    let (x, y, z) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("{},{},{}", tup.0, tup.1, tup.2);

    let arr1:[i32; 5] = [1,2,3,4,5];
    let arr2 = [3;5];
    println!("{}", arr1[1]);

    printFunc();

    let y = {
        let x = 1;
        x + 1
    };
    println!("y = {}", y);

    println!("five() = {}", five());

    let arr:[i32;5] = [1,2,3,4,5];

    for item in arr.iter() {
        println!("{}", item)
    }

    let mut index = 0;
    while index < 5 {
        println!("{}", arr[index]);
        index = index + 1;
    }

    let mut index = 0;
    loop {
        if index >= 5 {
            break;
        }
        println!("{}", arr[index]);
        index = index + 1;
    }


}

fn printFunc() {
    println!("printFunc")
}

fn five() -> i32 {
    5
}
