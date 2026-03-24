fn main() {
    println!("Hello, world!");

    // 字符串拼接
    let mut s1 = String::from("s1");
    s1.push_str("_push_"); // 参数字符串切片
    s1.push('1');
    println!("{}", s1);

    let s2 = "s2".to_string();
    println!("{}", s2);

    // 字符串相加
    let s3 = s1 + &s2; //string + str切片 = 
    println!("{}", s3);
    // println!("{}", s1); 
    // s1所有权移动给s3， s1失去所有权。

    // format字符串拼接
    let s4 = String::from("s4");
    let s5 = String::from("s5");
    let s6 = String::from("s6");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s7);


    // string index遍历, 不允许进行索引（因为不能确定有多少个合法字符）
    let s8 = String::from("kowloon");
    println!("{}", s8.len());

    for b in s8.bytes() {
        println!("{}", b);
    }

    for b in s8.chars() {
        println!("{}", b);
    }

    let s9 = &s8[0..4];
    println!("{}", s9);
}
