//! Rust 过程宏演示程序
//!
//! 这个程序演示了如何使用自定义的派生宏和属性宏。
//! 派生宏：`#[derive(HelloMacroDerive)]` 自动实现 `HelloMacro` trait。
//! 属性宏：`#[route]` 为函数添加路由信息。

// 注意：Rust 2018 不需要 `extern crate`，依赖项通过 Cargo.toml 自动链接。
// 过程宏（派生宏和属性宏）可以直接使用，只要对应的 crate 在依赖项中。

// 导入过程宏
use macros::HelloMacroDerive;
use macros::route;

/// HelloMacro trait 定义
/// 派生宏 `#[derive(HelloMacroDerive)]` 会为类型自动实现这个 trait。
pub trait HelloMacro {
    fn hello(&self) -> String;
}

fn main() {
    println!("=== Rust 过程宏演示 ===\n");

    // 1. 派生宏演示
    println!("1. 派生宏演示：HelloMacro");
    println!("----------------------------------------");

    // 定义一个结构体并使用派生宏自动实现 HelloMacro trait
    #[derive(HelloMacroDerive)]
    struct User {
        name: String,
        age: u32,
    }

    // 创建结构体实例
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };

    // 调用由派生宏自动实现的 hello 方法
    println!("   user.hello() = {}", user.hello());
    // 输出: "Hello, User!"

    // 另一个例子：枚举类型
    #[derive(HelloMacroDerive)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;
    println!("   color.hello() = {}", color.hello());
    // 输出: "Hello, Color!"

    println!("\n2. 属性宏演示：route");
    println!("----------------------------------------");

    // 调用带有 route 属性的函数
    // 注意：属性宏会在编译时修改函数，添加路由注册逻辑
    get_users();
    create_user();
    default_route();

    println!("\n=== 演示结束 ===");
}

// 2. 属性宏演示
// 使用 #[route] 属性宏为函数添加路由信息
// 参数：HTTP 方法和路径

/// 获取用户列表的路由处理函数
#[route(GET, "/users")]
fn get_users() -> String {
    println!("   [get_users] 执行：返回用户列表");
    "用户列表：Alice, Bob, Charlie".to_string()
}

/// 创建用户的路由处理函数
#[route(POST, "/users")]
fn create_user() -> String {
    println!("   [create_user] 执行：创建新用户");
    "用户创建成功".to_string()
}

// 注意：属性宏可以接受不同的参数
// 下面的例子展示了带默认值的属性宏（当不提供参数时使用默认值）

/// 默认路由（使用默认参数）
#[route]
fn default_route() -> String {
    println!("   [default_route] 执行：默认路由");
    "默认路由".to_string()
}

// 测试函数：演示如何调用这些函数
fn _test_calls() {
    // 这些函数可以正常调用，属性宏不会改变函数的调用方式
    let _result1 = get_users();
    let _result2 = create_user();
    let _result3 = default_route();
}

// 注意：
// 1. 派生宏通过 `#[derive(HelloMacroDerive)]` 使用，自动为类型实现指定的 trait。
//    - 要求 trait 在当前作用域中定义或导入。
//    - 宏展开时生成对应的 impl 代码。
// 2. 属性宏通过 `#[route(...)]` 使用，可以修改函数的 AST，添加额外的逻辑。
//    - 可以接受参数（HTTP 方法和路径）。
//    - 在函数执行前插入打印语句（模拟路由注册）。
// 3. 过程宏在编译时执行，不会影响运行时性能（除了生成的代码本身）。
//
// 实际应用场景：
// - 派生宏：序列化/反序列化（如 serde）、Builder 模式、ORM 映射等。
// - 属性宏：Web 框架路由、测试框架、日志装饰器等。