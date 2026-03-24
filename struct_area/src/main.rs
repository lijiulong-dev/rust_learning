fn main() {
    println!("Hello, world!");
    println!("{}", area((3,4)));

    let r = Rect {
        width: 2,
        length: 4,
    };

    // :?
    println!("{:#?}", r);
    // :#?
    println!("{:?}", r);
    
    println!("{:#?}", r.area_rect());

    let o = Rect {
        width: 1,
        length: 2,
    };
    println!("{}", r.can_hold(&o));

    println!("{:?}", Rect::square(3));
}

// 自定义Debug注解
#[derive(Debug)]
struct Rect {
    width: u32,
    length: u32,
}

// 函数三种传参：借用，可变借用， 所有权转移。
impl Rect {
    fn area_rect(&self) -> u32 {
        self.width + self.length
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.length > other.length
    }

    // 关联函数， 非方法
    fn square(size: u32) -> Rect {
        Rect{
            width: size,
            length: size,
        }
    }
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 + dim.1
}