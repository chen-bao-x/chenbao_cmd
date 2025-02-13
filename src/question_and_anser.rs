use crate::arg_type::ReplArgStore;
use crate::helper::*;
use owo_colors::OwoColorize;
use std::{num::ParseIntError, path::Path, vec};

use super::*;
use arg_type::key_gen;

const DEFAULTINDEX: usize = 1;
// #[derive(Debug)]
/// ArgType::Repl(_) 需要用到 ReplQuestions.  
pub struct DialogGenerator {
    /// 从 json_str 转换过来的 Vec<String>.
    /// 也可能是通过 问答式命令行交互 获取到的 Vec<String>.
    pub arguments: ReplArgStore,

    /// 当 Self 是从 json_str 转换过来的 Vec<String> 时,
    /// 这个用户标记读取到了哪一个参数.
    pub index: usize,

    /// 是否是从 json_str 转换过来的?
    pub is_from_json: bool,

    theme: dialoguer::theme::ColorfulTheme,
}
impl Default for DialogGenerator {
    fn default() -> Self {
        Self::new()
    }
}
impl DialogGenerator {
    /* private */

    /// Creates a new [`ReplQuestions`].
    /// * `input` :
    ///     1. 如果是 `None`, 则会在命令行里要求用户来提交所需要的参数.  
    ///     2. 如果是 `Some(json_string)`, 则会直接解析并返回所需参数.
    /// ```rs
    /// let cmd = crate::DialogGenerator::new(None);
    /// let cmd2 = crate::DialogGenerator::new(Some(r#"["hello"]"#));
    /// ```
    pub fn new() -> Self {
        Self {
            arguments: ReplArgStore::new(),
            index: DEFAULTINDEX,
            is_from_json: false,
            theme: dialoguer::theme::ColorfulTheme::default(),
        }
    }

    /// ```rs
    /// let cmd = crate::DialogGenerator::new_from_jsonstr(r#"["hello"]"#);
    /// ```
    pub fn new_from_toml(str: &str) -> Result<Self, String> {
        ReplArgStore::from_toml(str)
            .map(|art_store| Self {
                arguments: art_store,
                index: DEFAULTINDEX,
                is_from_json: true,
                theme: dialoguer::theme::ColorfulTheme::default(),
            })
            .map_err(|_e| format!("{}{}转换为 json 时出错: {}", file!(), line!(), _e))
    }

    /// 转换为 json 字符串.
    /// ```rs
    /// let cmd = DialogGenerator::new(Some(r#"["hello"]"#));
    /// let json_string = cmd.to_json_str();
    /// ```
    pub fn to_toml(&self) -> String {
        self.arguments.to_toml().unwrap()
    }
}

impl DialogGenerator {
    // _string
    pub fn string(&mut self, prompt: &str) -> arg_type::String {
        if self.is_from_json {
            let result_value = self.arguments.get(self.index, prompt).unwrap().get_string();

            self.ret(result_value)
        } else {
            let result_value = DialogerWraper::get_string(prompt, &self.theme);

            self.arguments.add(
                self.index,
                prompt,
                arg_type::ReplArg::String(result_value.clone()),
            );
            self.ret(result_value)
        }
    }
    // _string_multiple
    pub fn string_multiple(&mut self, prompt: &str) -> arg_type::StringMutiple {
        if self.is_from_json {
            let result_value = self
                .arguments
                .get(self.index, prompt)
                .unwrap_or_else(|| panic!("没找到需要的参数: {}", key_gen(self.index, prompt)))
                // .unwrap()
                .get_string_multiple();

            self.ret(result_value)
        } else {
            let result_value = DialogerWraper::get_string_multiple(prompt, &self.theme);

            self.arguments.add(
                self.index,
                prompt,
                arg_type::ReplArg::StringMultiple(result_value.clone()),
            );
            self.ret(result_value)
        }
    }

    // _number
    pub fn number(&mut self, prompt: &str) -> arg_type::Number {
        // let mut self = self;

        if self.is_from_json {
            // let val = self.arguments.get(self.index, prompt);
            let val = self.arguments.get(self.index, prompt);

            if let Some(str) = val {
                let result_value = str.get_number();

                return self.ret(result_value);
            }

            panic!("key: {}", key_gen(self.index, prompt));
        } else {
            // get value from REPL.

            let result_value = DialogerWraper::get_number(prompt, &self.theme);
            // let string = serde_json::to_string(&result_value).unwrap();

            self.arguments
                .add(self.index, prompt, arg_type::ReplArg::Number(result_value));
            self.ret(result_value)
        }
    }
    // _number_multiple
    pub fn number_multiple(&mut self, prompt: &str) -> arg_type::NumberMutiple {
        if self.is_from_json {
            let result_value = self
                .arguments
                .get(self.index, prompt)
                // .expect(&format!("{:?}", key_gen(self.index, prompt)))
                .unwrap_or_else(|| panic!("{:?}", key_gen(self.index, prompt)))
                .get_number_multiple();
            self.ret(result_value)
        } else {
            let multiple_string = DialogerWraper::get_string_multiple(prompt, &self.theme);

            let mut result_value: Vec<arg_type::Number> = vec![];
            {
                /* 为 result_value 赋值. */

                for str in &multiple_string {
                    let number_from_str: Result<arg_type::Number, std::num::ParseIntError> =
                        str.parse();

                    if let Ok(x) = number_from_str {
                        // 成功获取到了需要的参数
                        result_value.push(x);
                    } else {
                        eprintln!("需要的是多个 bool 类型的值, 示例: true false true");

                        panic! {"{:?}", multiple_string};
                    }
                }
            }
            self.arguments.add(
                self.index,
                prompt,
                arg_type::ReplArg::NumberMultiple(result_value.clone()),
            );
            self.ret(result_value)
        }
    }
    // _yes_or_no
    pub fn yes_or_no(&mut self, prompt: &str) -> arg_type::Bool {
        if self.is_from_json {
            let result_value = self.arguments.get(self.index, prompt).unwrap().get_bool();
            self.ret(result_value)
        } else {
            // get value from REPL.

            let result_value = DialogerWraper::get_bool(prompt, &self.theme);

            self.arguments
                .add(self.index, prompt, arg_type::ReplArg::Bool(result_value)); // -> "true" or "false"
            self.ret(result_value)
        }
    }
    // _path
    pub fn path(&mut self, prompt: &str) -> arg_type::Path {
        if self.is_from_json {
            let val = self.arguments.get(self.index, prompt);
            if let Some(str) = val {
                let result_value = str.get_path();
                return self.ret(result_value);
            }
            panic!();
        } else {
            // get value from REPL.

            let str = DialogerWraper::get_string(prompt, &self.theme);

            let result_value = Path::new(&str).to_path_buf();

            self.arguments
                .add(self.index, prompt, arg_type::ReplArg::String(str));
            self.ret(result_value)
        }
    }
    // _path_multiple
    pub fn path_multiple(&mut self, prompt: &str) -> arg_type::PathMutiple {
        if self.is_from_json {
            let result_value = self
                .arguments
                .get(self.index, prompt)
                .unwrap()
                .get_path_multiple();

            self.ret(result_value)
        } else {
            let multiple_string = DialogerWraper::get_string_multiple(prompt, &self.theme);

            let result_value: arg_type::PathMutiple = multiple_string
                .iter()
                .map(|x| Path::new(&x).to_path_buf())
                .collect();

            self.arguments.add(
                self.index,
                prompt,
                arg_type::ReplArg::StringMultiple(multiple_string),
            );
            self.ret(result_value)
        }
    }
    // _select
    pub fn select(&mut self, prompt: &str, items: &Vec<&str>) -> arg_type::String {
        if self.is_from_json {
            let result_value = self.arguments.get(self.index, prompt).unwrap().get_string();
            self.ret(result_value)
        } else {
            // get value from REPL.

            let str = DialogerWraper::get_single_selected(prompt, items, &self.theme);

            let result_value = arg_type::ReplArg::String(str.to_string());

            self.arguments.add(self.index, prompt, result_value);
            self.ret(str.to_string())
        }
    }
    // _select_multiple
    pub fn select_multiple(&mut self, prompt: &str, items: &Vec<&str>) -> arg_type::StringMutiple {
        if self.is_from_json {
            let result_value = self
                .arguments
                .get(self.index, prompt)
                .unwrap()
                .get_string_multiple();

            self.ret(result_value)
        } else {
            // get value from REPL.

            let result_value: Vec<String> =
                DialogerWraper::get_multiple_selected(prompt, items, &self.theme)
                    .iter()
                    .map(|x| x.to_string())
                    .collect();

            let arg = arg_type::ReplArg::StringMultiple(result_value.clone());

            self.arguments.add(self.index, prompt, arg);

            self.ret(result_value)
        }
    }
    // _editor
    pub fn editor(&mut self, prompt: &str) -> arg_type::String {
        if self.is_from_json {
            println!("is_from_json");
            let val = self.arguments.get(self.index, prompt);

            match val {
                Some(str) => {
                    // 成功获取到了需要的参数
                    let result_value = str.get_string();

                    return self.ret(result_value);
                }
                None => {
                    // not string
                }
            }

            panic!();
        } else {
            let result_value = DialogerWraper::get_string_from_editor(prompt);

            self.arguments.add(
                self.index,
                prompt,
                arg_type::ReplArg::String(result_value.clone()),
            );

            self.ret(result_value)
        }
    }
    // _password
    /// 让用户手动输入密码.
    pub fn password(&mut self, prompt: &str) -> String {
        // 密码不应该被输出到 self.arguments 里面.

        DialogerWraper::password(prompt, &self.theme)
    }

    // _password_with_confirmation
    pub fn password_with_confirmation(&mut self, prompt: &str) -> String {
        _ = self;

        // 密码不应该被输出到 self.arguments 里面.
        DialogerWraper::password_with_confirmation(prompt)
    }

    pub fn finesh(&mut self, app_name: &String, command_name: &String) {
        let app_name = app_name.cyan();
        let command_name = command_name.bright_cyan();
        // println!("runing command: {app_name} {command_name} stdin << '###_marker_###'\n{}\n###_marker_###\n", self.to_toml().green());
        println!(
            r#"
Executed command: {app_name} {command_name} stdin << '{marker}'
{toml_str}
{marker}
"#,
            toml_str = self.to_toml().green(),
            // marker = r#"###_marker_###"#,
            marker = r#""""""""""""#,
        );
    }

    fn ret<T>(&mut self, result_value: T) -> T {
        self.index += 1;
        result_value
    }
}

#[cfg(test)]
mod test_repl_new_api_style {
    use super::*;

    use std::fmt::Debug;

    // #[test]
    fn dialog_generator_tester<F, R>(val: Option<&str>, f: F)
    where
        F: Fn(&mut DialogGenerator) -> R,
        R: Debug,
    {
        let mut repl = match val {
            Some(str) => DialogGenerator::new_from_toml(str).unwrap(),
            None => DialogGenerator::new(),
        };
        let x = f(&mut repl);

        let is_from_json = val.is_some();

        println!("输入的是: {:?}", x);
        println!("json_str: {}", repl.to_toml());
        assert_eq!(repl.is_from_json, is_from_json);

        // repl.editor("prompt");
    }

    // #[test]
    // fn it_works() {
    // // 此测试需要手动测试.
    //     let mut repl1 = DialogGenerator::new(None);
    //     {
    //         let items = vec!["one", "two", "tree", "four"];

    //         let _str = repl1.string("string");
    //         let _string_multiple = repl1.string_multiple("string_multiple");
    //         let _num = repl1.number("number");
    //         let _number_multiple = repl1.number_multiple("number_multiple");
    //         let _number_multiple = repl1.path("path");
    //         let _number_multiple = repl1.path_multiple("path_multiple");
    //         let _number_multiple = repl1.select("select", &items);
    //         let _number_multiple = repl1.select_multiple("select_multiple", &items);
    //         let _password = repl1.password("password");
    //         let _password_with_confirmation = repl1.password_with_confirmation("password");
    //         let _editor = repl1.editor("editor");
    //         let _yes_or_no = repl1.yes_or_no("yes_or_no");
    //     }

    //     let json_str = repl1.to_json_str();
    //     let mut _repl2 = DialogGenerator::new(Some(&json_str));

    //     println!(
    //         "repl1: {:?}\nrepl2: {:?}",
    //         repl1.arguments, _repl2.arguments
    //     );
    //     assert_eq!(repl1.arguments, _repl2.arguments);
    // }

    #[test]
    fn test_string() {
        // 已测试, 可以逆转.

        // // form terminal repl.
        // dialog_generator_tester(None, |x| {
        //     return x.string("prompt").to_string();
        // });

        // form json string.
        dialog_generator_tester(Some(r#"     ["sadfdsaf dsf sad f"]    "#), |x| {
            x.string("string").to_string()
        });
    }

    #[test]
    fn test_string_multiple() {
        // 已测试, 可以逆转.

        // form terminal repl.
        // dialog_generator_tester(None, |x| {
        //     return x.string_multiple("prompt");
        // });

        // form json string.
        dialog_generator_tester(Some(r#"   ["[\"sadfdsafsadf\",\"sdaf\"]"]   "#), |x| {
            x.string_multiple("prompt")
        });
    }

    #[test]
    fn test_number() {
        // 已测试, 可以逆转.

        // // form terminal repl.
        // dialog_generator_tester(None, |x| {
        //     return x.number("prompt");
        // });

        // form json string.
        dialog_generator_tester(Some(r#"       ["325435325"]      "#), |x| {
            x.number("prompt")
        });
    }

    #[test]
    fn test_number_multiple() {
        // 已测试, 可以逆转.

        // form terminal repl.
        // dialog_generator_tester(None, |x| {
        //     return x.number_multiple("prompt");
        // });

        // form json string.
        dialog_generator_tester(
            Some(r#"      ["[\"1\",\"2\",\"3\",\"4\",\"5\"]"]     "#),
            |x| {
                x.number_multiple("prompt")
            },
        );
    }

    #[test]
    fn test_yes_or_no() {
        // 已测试, 可以逆转.

        // // form terminal repl.
        // dialog_generator_tester(None, |x| {
        //     return x.yes_or_no("prompt");
        // });

        // form json string.
        dialog_generator_tester(Some(r#"      ["false"]     "#), |x| {
            x.yes_or_no("prompt")
        });
    }
}

// ------- REPL Functions -------

/// 对 dialoguer crate 的二次封装.
struct DialogerWraper();
impl DialogerWraper {
    fn get_string(prompt: &str, theme: &dialoguer::theme::ColorfulTheme) -> String {
        let re = dialoguer::Input::<String>::with_theme(theme)
            .with_prompt(prompt)
            .interact_text();

        match re {
            Ok(s) => s,
            Err(_e) => {
                eprintln!("{}", _e.red());
                DialogerWraper::get_string(prompt, theme) // 继续本次问题
            }
        }
    }

    fn get_string_multiple(prompt: &str, theme: &dialoguer::theme::ColorfulTheme) -> Vec<String> {
        let re = dialoguer::Input::<String>::with_theme(theme)
            .with_prompt(prompt)
            .interact_text();

        match re {
            Ok(input) => crate::helper::parse_arg_string(&input),
            Err(_e) => {
                eprintln!("{}", _e.red());
                DialogerWraper::get_string_multiple(prompt, theme) // 继续本次问题
            }
        }
    }

    fn get_number(prompt: &str, theme: &dialoguer::theme::ColorfulTheme) -> arg_type::Number {
        let input = DialogerWraper::get_string(prompt, theme);
        let input = input.trim();
        // 用户说输入了某些东西
        let parse_result: Result<arg_type::Number, ParseIntError> = input.parse();

        match parse_result {
            Ok(num) => num,
            Err(_e) => {
                let err_message = format!("{}", _e).red().to_string();
                eprintln!("{}", err_message);

                println!("需要输入一个数字, 示例: {x}", x = "123".styled_arg());

                DialogerWraper::get_number(prompt, theme) // 继续本次问题
            }
        }
    }

    fn get_bool(prompt: &str, theme: &dialoguer::theme::ColorfulTheme) -> bool {
        let re = dialoguer::Confirm::with_theme(theme)
            // .with_prompt("Y 键 N 键选择, 回车键确认: ")
            .with_prompt(prompt)
            .wait_for_newline(true)
            .interact();

        match re {
            Ok(b) => b,
            Err(_e) => {
                eprintln!("{}", _e.red());
                DialogerWraper::get_bool(prompt, theme) // 继续本次问题
            }
        }
    }

    fn get_single_selected<'a, T>(
        prompt: &str,
        items: &'a [T],
        theme: &dialoguer::theme::ColorfulTheme,
    ) -> &'a T
    where
        T: ToString + Clone,
    {
        let re = dialoguer::FuzzySelect::with_theme(theme)
            .with_prompt(prompt)
            .items(items)
            .default(0)
            .interact();

        match re {
            Ok(selection) => &(items[selection]),
            Err(_e) => {
                eprintln!("{}", _e.red());

                // 继续本次问题
                DialogerWraper::get_single_selected(prompt, items, theme)
            }
        }
    }

    fn get_multiple_selected<T>(
        prompt: &str,
        items: &[T],
        theme: &dialoguer::theme::ColorfulTheme,
    ) -> Vec<T>
    where
        T: ToString + Clone,
    {
        let re = dialoguer::MultiSelect::with_theme(theme)
            // .with_prompt("What do you choose?")
            .with_prompt(prompt)
            .items(items)
            .interact();

        match re {
            Ok(selection) => {
                let mut re: Vec<T> = vec![];

                for i in selection {
                    re.push(items[i].clone());
                }

                re
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                DialogerWraper::get_multiple_selected(prompt, items, theme) // 继续本次问题
            }
        }
    }

    fn get_string_from_editor(prompt: &str) -> arg_type::String {
        let re = dialoguer::Editor::new().edit(prompt);

        match re {
            Ok(ostr) => ostr.unwrap_or("".to_string()),
            Err(_e) => {
                panic!("{}", _e.red());
            }
        }
    }

    fn password(prompt: &str, theme: &dialoguer::theme::ColorfulTheme) -> String {
        let prompt_message = if prompt.is_empty() {
            "input password"
        } else {
            prompt
        };

        let re = dialoguer::Password::with_theme(theme)
            .with_prompt(prompt_message)
            .interact();
        match re {
            Ok(password) => password,
            Err(_e) => {
                panic!("{}", _e.red());
            }
        }
    }

    fn password_with_confirmation(prompt: &str) -> String {
        println!("{}", prompt.bright_green());

        let re = dialoguer::Password::new()
            .with_prompt(if prompt.is_empty() {
                "New Password"
            } else {
                prompt
            })
            .with_confirmation("Confirm password", "Passwords mismatching")
            .interact();
        match re {
            Ok(password) => password,
            Err(_e) => panic!("{}", _e.red()),
        }
    }
}

#[cfg(test)]
mod test_dialog {
    // 这里面都是一些 dialoguer 式交互, 需要手动来测试.

    // use super::*;

    // #[test]
    // fn test_get_string() {
    //     let a = DialogGeter::get_string("请输入一个字符串");
    //     println!("最终获得的 string 是: {}", a);
    // }

    // #[test]
    // fn test_repl_get_number() {
    //     let a = DialogGeter::get_number("你要吃几个汉堡?");
    //     println!("最终获得的数字是: {}", a);
    // }

    // #[test]
    // fn test_repl_get_multiple_string() {
    //     let arr = DialogGeter::get_string_multiple("请输入多个字符串");
    //     println!("{:?}", arr);
    // }

    // #[test]
    // fn test_repl_req_bool() {
    //     let b = DialogGeter::get_bool("test_repl_req_bool");
    //     println!("最终获得的数字是: {:?}", b);
    // }

    // #[test]
    // fn test_repl_req_string() {
    //     let b = DialogGeter::get_string("请输入一个字符串");
    //     println!("最终获得的数字是: {:?}", b);
    // }

    // #[test]
    // fn test_get_single_selected() {
    //     let items = vec!["foo", "bar", "baz"];

    //     let b = DialogGeter::get_single_selected("prompt", &items);
    //     println!("最终获得的数字是: {:?}", b);
    // }

    // #[test]
    // fn test_get_multiple_selected() {
    //     let items = vec!["foo", "bar", "baz"];
    //     let b = DialogGeter::get_multiple_selected("prompt", &items);
    //     println!("最终获得的数字是: {:?}", b);
    // }

    // #[test]
    // fn test_edit() {
    //     let b = DialogGeter::editor("prompt");
    //     println!("最终获得的数字是: {:?}", b);
    // }

    // #[test]
    // fn test_password() {
    //     let b = DialogGeter::password("");
    //     println!("最终获得的数字是: {:?}", b);
    // }

    // #[test]
    // fn test_password_with_confirmation() {
    //     let b = DialogGeter::password_with_confirmation("password_with_confirmation");
    //     println!("最终获得的数字是: {:?}", b);
    // }
}
