fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    match v.get(0) {
        Some(first) => println!("the first element is {}", first),
        None => println!("there is no first element"),
    }

    let v2 = vec![1,2,3,4];
    let two = &v2[1];
    println!("the second element is {}", two);

    let mut v3 = vec![1,2,3,4,5];
    let first = v3[0];
    println!("the first element is {}", first);
    v3.push(6);

    let mut v4 = vec![1,3,5,7,9];
    for i in &mut v4 {
        *i += 1;
    }
    for i in v4 {
        println!("{}", i)
    }

    // vec 存放枚举
    #[derive(Debug)]
    enum ExcelDot {
        Int(i32),
        Float(f32),
        Text(String)
    }

    let v5 = vec![
        ExcelDot::Int(1),
        ExcelDot::Float(1.2),
        ExcelDot::Text(String::from("txt"))
    ];

    for i in &v5 {
        println!("{:?}", i)
    }
}
