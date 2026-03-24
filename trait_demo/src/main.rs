fn main() {
    println!("Hello, world!");

    let article = Article{Name: String::from("lijiulong")};
    println!("{}", article.summarize());

    println!("{}", notify(article));
}


pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub Name: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("name is {}", self.Name)
    }
}

fn notify<T: Summary>(item : T) -> String {
    item.summarize()
}