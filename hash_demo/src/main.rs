use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // 读写
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    println!("{}", scores["Blue"]);

    match scores.get(&String::from("Blue")) {
        Some(s) => println!("{}", s),
        None => println!("not exist"),
    }

    // tuple元组 转换成 hashmap方法。
    let teams = vec![String::from("A1"), String::from("B1")];
    let scores2 = vec![10,20];

    let scores_map: HashMap<_, _> = teams.iter().zip(scores2.iter()).collect();
    println!("{}", scores_map[&String::from("B1")]);

    // 所有权
    // copy trait值会被复制到hashmap中
    // 拥有所有权的值，会被移动到hashmap

    // 遍历hashmap
    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }

    // 更新hashmap
    // 已存在： 会被替换
    // 不存在： 添加即可
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 15);
    println!("{:?}", scores);
     
    // 不存在，才插入数据
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);

    // 统计字符串空格分割的单词个数
    let text = "Hello World World Hello Key Value";
    let mut m_count = HashMap::new();

    for word in text.split_whitespace() {
        let value = m_count.entry(word).or_insert(0);
        *value += 1;
    }
    println!("{:?}", m_count);
}
