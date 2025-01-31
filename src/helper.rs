use owo_colors::OwoColorize;
use prettytable::format::TableFormat;

pub fn is_debug_mode() -> bool {
    return cfg!(debug_assertions);
}

pub fn debug_run<F>(f: F)
where
    F: Fn() -> (),
{
    if cfg!(debug_assertions) {
        f()
    }
}

// 我又这样的一个字符串: r#" "a" "b" c "d" e 32424 "32543" "a b dsaf""#
// 我希望将这个字符串解析成 Vec<String> vec!["a", "b", "c", "d", "e", "32424", "32543", "a b dsaf"]
// 帮我实现这个函数
pub fn parse_arg_string(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for c in input.chars() {
        if escaped {
            current_token.push(c);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            if in_quotes {
                result.push(current_token.clone());
                current_token.clear();
                in_quotes = false;
            } else {
                in_quotes = true;
            }
        } else if c == ' ' && !in_quotes {
            if !current_token.is_empty() {
                result.push(current_token.clone());
                current_token.clear();
            }
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        result.push(current_token);
    }

    result
}
#[test]
fn test_parse_string() {
    let input = r#" "a" "b" c "d" e 32424 "32543" "a b dsaf" asdfsaf 767544  "a b c\" d e""#;
    let result = parse_arg_string(input);
    println!("{:?}", result); // ["a", "b", "c", "d", "e", "32424", "32543", "a b dsaf", "asdfsaf", "767544", "a b c\" d e"]

    let s = result.last().unwrap();
    println!("{}", s); //  a b c" d e

    let input = r#" a b     c  d "e  f """#;
    let result = parse_arg_string(input);
    println!("{:?}", result); // ["a", "b", "c", "d", "e  f "]
}

// arg_color        green
// sub_cmd_color    cyan
// type_color       magenta

pub fn table_formater() -> TableFormat {
    let mut f = TableFormat::new();
    f.column_separator(' ');
    f.padding(4, 0);

    return f;
}
