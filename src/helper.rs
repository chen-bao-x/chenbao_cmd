use owo_colors::OwoColorize;
use prettytable::{format::TableFormat, table, Row, Table};

// pub(crate) fn is_debug_mode() -> bool {
//     return cfg!(debug_assertions);
// }

// 我有这样的一个字符串: r#" "a" "b" c "d" e 32424 "32543" "a b dsaf""#
// 我希望将这个字符串解析成 Vec<String> vec!["a", "b", "c", "d", "e", "32424", "32543", "a b dsaf"]
// 帮我实现这个函数
// Gemini:
// TODO: 让命令行字符串支持 单引号
pub(crate) fn parse_arg_string(input: &str) -> Vec<String> {
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

pub(crate) fn table_formater() -> TableFormat {
    let mut f = TableFormat::new();
    {
        f.column_separator(' ');
        f.padding(4, 0);
    }

    f
}

pub(crate) trait StyledString {
    fn styled_sub_command(&self) -> String;
    fn styled_arg_type(&self) -> String;
    fn styled_arg(&self) -> String;
    fn styled_error_marker(&self) -> String;
    // fn styled_repl_prompt(&self) -> String;
    // fn styled_repl_input(&self) -> String;
    // fn styled_repl_selected(&self) -> String;
}

impl<T: ToString> StyledString for T {
    fn styled_sub_command(&self) -> String {
        if self.to_string().is_empty() {
            String::new()
        } else {
            self.to_string().cyan().to_string()
        }
    }
    fn styled_arg_type(&self) -> String {
        if self.to_string().is_empty() {
            String::new()
        } else {
            self.to_string().magenta().to_string()
        }
    }
    fn styled_arg(&self) -> String {
        if self.to_string().is_empty() {
            String::new()
        } else {
            self.to_string().green().to_string()
        }
    }

    fn styled_error_marker(&self) -> String {
        if self.to_string().is_empty() {
            String::new()
        } else {
            self.to_string().bright_red().to_string()
        }
    }

    // fn styled_repl_prompt(&self) -> String {
    //     if self.to_string().is_empty() {
    //         String::new()
    //     } else {
    //         self.to_string()
    //     }
    // }

    // fn styled_repl_input(&self) -> String {
    //     if self.to_string().is_empty() {
    //         String::new()
    //     } else {
    //         self.to_string().bright_green().to_string()
    //     }
    // }

    // fn styled_repl_selected(&self) -> String {
    //     if self.to_string().is_empty() {
    //         String::new()
    //     } else {
    //         self.to_string().bright_magenta().to_string()
    //     }
    // }
}

// pub(crate) struct ColoredTheme {}
// impl ColoredTheme {
//     pub fn new() -> dialoguer::theme::ColorfulTheme {
//         dialoguer::theme::ColorfulTheme::default()
//     }
// }
// impl Theme for ColoredTheme {
//     /// Formats a confirm prompt.
//     fn format_confirm_prompt(
//         &self,
//         f: &mut dyn fmt::Write,
//         prompt: &str,
//         default: Option<bool>,
//     ) -> fmt::Result {
//         if !prompt.is_empty() {
//             write!(f, "{} ", &prompt)?;
//         }
//         match default {
//             None => write!(f, "[y/n] ")?,
//             Some(true) => write!(f, "[{}/n] ", " Yes ".styled_arg())?,
//             Some(false) => write!(f, "[y/{}]", " No ".styled_arg())?,
//         }
//         Ok(())
//     }

//     /// Formats an input prompt.
//     fn format_input_prompt(
//         &self,
//         f: &mut dyn fmt::Write,
//         prompt: &str,
//         default: Option<&str>,
//     ) -> fmt::Result {
//         let prompt = prompt.styled_repl_prompt();

//         match default {
//             Some(default) if prompt.is_empty() => {
//                 writeln!!(f, "[{}]: \n", default.styled_repl_input())
//             }
//             Some(default) => write!(f, "{} [{}]: ", prompt, default.styled_repl_input()),
//             None => write!(f, "{}: ", prompt),
//         }
//     }

//     /// Formats an input prompt after selection.
//     fn format_input_prompt_selection(
//         &self,
//         f: &mut dyn fmt::Write,
//         prompt: &str,
//         sel: &str,
//     ) -> fmt::Result {
//         write!(f, "{}: {}", prompt, sel.styled_repl_input())
//     }

//     /// Formats a select prompt item.
//     fn format_select_prompt_item(
//         &self,
//         f: &mut dyn fmt::Write,
//         text: &str,
//         active: bool,
//     ) -> fmt::Result {
//         write!(
//             f,
//             "{} {}",
//             if active { ">" } else { " " },
//             // text.bright_green(),
//             if active {
//                 text.styled_repl_selected()
//             } else {
//                 text.to_string()
//             }
//         )
//     }

//     /// Formats a multi select prompt item.
//     fn format_multi_select_prompt_item(
//         &self,
//         f: &mut dyn fmt::Write,
//         text: &str,
//         checked: bool,
//         active: bool,
//     ) -> fmt::Result {
//         write!(
//             f,
//             "{} {}",
//             match (checked, active) {
//                 (true, true) => "> [x]",
//                 (true, false) => "  [x]",
//                 (false, true) => "> [ ]",
//                 (false, false) => "  [ ]",
//             },
//             // text
//             if checked {
//                 text.styled_repl_selected()
//             } else {
//                 text.to_string()
//             }
//         )
//     }
// }

pub fn vec_row_to_table(arr: Vec<Row>) -> Table {
    let mut table = table!();
    table.set_format(table_formater());

    for x in arr {
        table.add_row(x);
    }

    table
}
