fn main() {
    println!("Hello, world!");

    let user1 = User {
        userName: String::from("kowloon"),
        email: String::from("jiulong.li@qq.com"),
        active: true,
        signInCount: 556,
    };

    println!("{}", user1.userName);

    // 省略语法
    let user2 = buildUser(String::from("kowloon2"));
    println!("{}", user2.userName);

    // 更新语法
    let user3 = User{
        userName: String::from("kowloon3"),
        ..user2
    };
    println!("{}", user3.userName);

    // tuple
    struct Point(i32, i32);
    let x = Point(1,2);
    let y = Point(3,4);

    // 空结构体，没任何字段。
    struct empty {

    };     
}

struct User {
    userName: String,
    email: String,
    signInCount: u64,
    active: bool,
}

// 省略字段语法
fn buildUser(userName: String) -> User {
    User {
        userName,
        email: String::from("email"),
        signInCount: 112,
        active: false,
    }
}