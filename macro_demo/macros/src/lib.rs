//! 这是一个 Rust 过程宏的演示库，包含派生宏和属性宏。
//!
//! 派生宏示例：`HelloMacro` trait 和 `#[derive(HelloMacro)]` 派生宏。
//! 属性宏示例：`#[route]` 属性宏，模拟 Web 框架的路由装饰器。
//!
//! 注意：过程宏必须定义在独立的 crate 中，并在 Cargo.toml 中设置 `proc-macro = true`。
//! 此外，过程宏 crate 不能导出除宏以外的任何项，因此 `HelloMacro` trait 需要在使用者 crate 中定义。

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

/// 派生宏：为结构体或枚举自动实现 `HelloMacro` trait。
///
/// # 用法
/// ```
/// // 需要在使用宏之前定义 HelloMacro trait 或将其导入作用域
/// trait HelloMacro {
///     fn hello(&self) -> String;
/// }
///
/// #[derive(HelloMacro)]
/// struct MyStruct;
///
/// let obj = MyStruct;
/// println!("{}", obj.hello()); // 输出: "Hello, MyStruct!"
/// ```
///
/// # 实现细节
/// 1. 使用 `syn::parse_macro_input!` 解析输入 TokenStream 为 `DeriveInput`。
/// 2. 提取类型名称（结构体或枚举名）。
/// 3. 使用 `quote::quote!` 生成实现代码。
/// 4. 将生成的代码转换为 `TokenStream` 返回。
///
/// # 注意
/// 该宏假设 `HelloMacro` trait 已经在当前作用域中定义。
/// 如果 trait 在其他模块中，需要使用 `use` 导入。
#[proc_macro_derive(HelloMacroDerive)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 将输入解析为 DeriveInput 结构体（表示一个 `#[derive(...)]` 的目标）
    let ast = parse_macro_input!(input as DeriveInput);

    // 提取类型名称（标识符）
    let name = &ast.ident;

    // 使用 quote 宏生成 Rust 代码
    let expanded = quote! {
        // 为类型实现 HelloMacro trait
        // 注意：这里假设 HelloMacro trait 在当前作用域中可用
        impl HelloMacro for #name {
            fn hello(&self) -> String {
                // 生成字符串 "Hello, <类型名>!"
                format!("Hello, {}!", stringify!(#name))
            }
        }
    };

    // 将生成的代码转换回 TokenStream
    TokenStream::from(expanded)
}

/// 属性宏：为函数添加路由信息，模拟 Web 框架的路由装饰器。
///
/// # 用法
/// ```
/// #[route(GET, "/users")]
/// fn get_users() -> String {
///     "User list".to_string()
/// }
/// ```
///
/// # 参数
/// 属性宏接受两个参数：HTTP 方法和路径。
/// 例如：`#[route(GET, "/users")]`
///
/// # 实现细节
/// 1. 使用 `syn::parse_macro_input!` 将属性参数解析为 `syn::AttributeArgs`（`Vec<syn::NestedMeta>`）。
/// 2. 使用 `syn::parse_macro_input!` 将函数项解析为 `ItemFn`。
/// 3. 提取函数名称、属性等。
/// 4. 生成一个新函数，包装原始函数，并添加路由注册逻辑。
/// 5. 注意：这里只是演示，实际的路由注册逻辑需要全局注册表。
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析属性参数：期望两个 token，例如 `GET, "/users"`
    // 在 syn v1.0 中，AttributeArgs 是 Vec<NestedMeta> 的别名
    let attr_args = parse_macro_input!(attr as syn::AttributeArgs);

    // 解析函数定义
    let input_fn = parse_macro_input!(item as ItemFn);

    // 提取函数信息
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;

    // 从属性参数中提取 HTTP 方法和路径（简化处理）
    let (http_method, path) = if attr_args.len() == 2 {
        // 第一个参数应该是 HTTP 方法（标识符）
        let method = match &attr_args[0] {
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                // 提取路径的最后一段作为方法名
                path.segments.last().unwrap().ident.to_string()
            }
            _ => "GET".to_string(),
        };

        // 第二个参数应该是路径字符串
        let path_str = match &attr_args[1] {
            syn::NestedMeta::Lit(syn::Lit::Str(lit_str)) => lit_str.value(),
            _ => "/".to_string(),
        };

        (method, path_str)
    } else {
        // 如果参数数量不对，使用默认值
        ("GET".to_string(), "/".to_string())
    };

    // 生成新的函数代码
    // 在实际应用中，这里可能会将路由信息注册到全局路由表
    // 这里我们只是添加一个 println! 来演示
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // 在函数执行前打印路由信息（模拟路由注册）
            println!(
                "路由注册: {} {} -> 函数 {}",
                #http_method,
                #path,
                stringify!(#fn_name)
            );

            // 执行原始函数体
            #fn_block
        }
    };

    TokenStream::from(expanded)
}