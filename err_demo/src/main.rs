use std::fs::File;

fn main() {
    println!("Hello, world!");
    // 不可恢复的错误
    //panic!("crash and burn");
    // let v = vec![1,2,3];
    // println!("{}", v[3]);


    // 可恢复的错误
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => {
            panic!("error opening file {:?}", error);
        }
    };

    // ？ 表达式
    // OK：赋值，继续执行
    // Err： return err

    // unwrap表达式
    //  
    // 

    // 本质是一个枚举
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
