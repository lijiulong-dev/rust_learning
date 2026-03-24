// mod+分号， 会引入以该mod为文件名的文件内容
mod front_of_house;

mod backend_of_house {
    fn cook() {}
}

// use 绝对路径
use crate::front_of_house::hosting;
// use 可以相对路径

// use 默认是文件私有的， 可以使用pub use将引入的代码重新导出。

fn eat() {

}

pub fn eat_at_restaurant() {
    // 绝对路径调用方式
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径调用方式
    front_of_house::hosting::add_to_waitlist();

    // use引用的包 
    hosting::add_to_waitlist();
}

/*
绝对路径: 从crate root跟开始
相对路径: self,super,当前模块标识符

父不能访问子里面的私有
子可以访问父的所有
可以访问同级别的私有方法。

*/