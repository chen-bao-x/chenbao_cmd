// lib.rs 文件中定义过程宏
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// # 示例:
/// ```rs
/// let app = App::new()
///     .add_command(
///         cmd!("test")
///             .action(ArgAction::String(&|s|{
///                 println!(r#"I got "{}""#, s)
///             })
///         )
///     );
/// ```
#[proc_macro]
pub fn cmd(input: TokenStream) -> TokenStream {
    // 解析输入
    let input = parse_macro_input!(input as LitStr);

    // 获取字面量的字符串值
    let string_value = input.token().to_string();

    if let Some(c) = contains_invalid_char(&string_value) {
        return syn::Error::new_spanned(input, format!("子命令名称不能包含字符: '{c}' "))
            .to_compile_error()
            .into();
    } else if contains_default_flags(&string_value) {
        return syn::Error::new_spanned(
            input,
            format!("{string_value} 不能用于子命令名称, {string_value} 已经有了默认实现.",),
        )
        .to_compile_error()
        .into();
    } else if string_value == r##"r#""#"## || string_value == r##""""## {
        // let msg = format!(r#"name 不能是空字符串 "", name 至少需要一个字符."#,);

        return syn::Error::new_spanned(input, r#"name 不能是空字符串 "", name 至少需要一个字符."#)
            .to_compile_error()
            .into();
    }

    let cmd = quote! {
        chenbao_cmd::SubCommand::create_an_sub_command(#input)
    };
    return cmd.into();
}

fn contains_invalid_char(value: &str) -> Option<char> {
    value.chars().find(|&c| {
        c.is_ascii_control()
            || c.is_whitespace()
            || c.is_control()
            || c.is_ascii_whitespace()
            || ['$', '!', '(', ')', '[', ']', '\\', '\'', '\n'].contains(&c)
    })
}

fn contains_default_flags(value: &str) -> bool {
    let string_value = value.to_string();
    let string_value = string_value.trim_start_matches('"').to_string();
    let string_value = string_value.trim_end_matches('"').to_string();

    if string_value == "-h" {
        return true;
    }

    let arr: Vec<String> = vec![
        "-h".to_string(),
        "--help".to_string(),
        // "-e".to_string(),
        // "--example".to_string(),
        "-v".to_string(),
        "--version".to_string(),
        // "--list-all-commands".to_string(),
    ];

    arr.contains(&string_value)
}
