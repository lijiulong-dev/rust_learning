fn main() {
    println!("Hello, world!");

    println!("{:?}", IpAddrKind::V4);
    println!("{:?}", IpAddrKind::V6);

    let v4 = IpAddrKindString::V4(127,0,0,1);
    let v6 = IpAddrKindString::V6(String::from(":1"));
    println!("{:?}", v4);
    println!("{:?}", v6);

    let q = Message::Quit;
    q.call();
}


// 不带类型的枚举，仅仅表示type
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
// 带类型的枚举， 可以表示type和值
// 可以嵌入任何类型的数据
#[derive(Debug)]
enum IpAddrKindString {
    V4(u8, u8, u8, u8),
    V6(String),
}
// IpAddr
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// eg
enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

