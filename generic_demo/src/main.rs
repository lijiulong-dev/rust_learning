fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 15, 50, 21, 29];
    let result = larest_num(&number_list);
    println!("larest num is {}", result);

    let p = Point{x: 1, y: 2};
    println!("p.x = {}", p.x())
}

fn larest_num(list: &[i32]) -> i32 {
    let mut ret = list[0];
    for &item in list {
        if item > ret {
            ret = item
        }
    }

    return ret
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}