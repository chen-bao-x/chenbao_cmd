use std::{fmt::format, num::ParseIntError, path::Path, vec};

use owo_colors::OwoColorize;

use super::*;

// /// 问答式命令行交互
// #[derive(Debug)]
// pub struct ReplQA {
//     pub tips: &'static str,

//     /// need ArgType
//     pub need_arg_type: ArgType,

//     pub value: Option<SubcommandArgsValue>,

//     pub when_failed: WhenFailed,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum WhenFailed {
//     Terminate,
//     Continue,
// }

// impl ReplQA {
//     pub fn run(&mut self) {
//         println!("{}\n{}", self.tips, self.need_arg_type.arg_type_tips());

//         let mut input = String::new();
//         let re = std::io::stdin().read_line(&mut input);
//         match re {
//             Ok(_) => {
//                 let input = input.trim_end_matches('\n');

//                 let args = parse_arg_string(input);
//                 println!("{:?}", args);

//                 let v = SubcommandArgsValue::new(self.need_arg_type.clone(), args);

//                 match self.need_arg_type {
//                     ArgType::Empty => {
//                         let re = v.clone().get_empty();

//                         match re {
//                             Ok(_) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);

//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::String => {
//                         let re = v.clone().get_string();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecString => {
//                         let re = v.clone().get_vec_string();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Number => {
//                         let re = v.clone().get_number();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecNumber => {
//                         let re = v.clone().get_number();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Path => {
//                         let re = v.clone().get_path();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecPath => {
//                         let re = v.clone().get_vec_path();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Bool => {
//                         let re = v.clone().get_bool();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecBool => {
//                         let re = v.clone().get_vec_bool();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Repl => todo!(),
//                 };
//             }
//             Err(f) => {
//                 match self.when_failed {
//                     WhenFailed::Terminate => {
//                         println!("{}", f);
//                         exit(0);
//                     }
//                     WhenFailed::Continue => {
//                         self.run();
//                     }
//                 };
//             }
//         };
//     }

//     // yes or no QA
//     // true or false QA
//     // number QA
//     // vec<number> QA
//     // string QA
//     // Vec<string> QA
//     // path QA
//     // Vec<path> QA
//     // password QA whith confirm

//     // enum single selection QA
//     // enum multi selection QA

//     // path single selection QA
//     // path multi selection QA

// }

// #[test]
// fn adsfsadf() {
//     let mut repl = ReplQA {
//         tips: "tips",
//         need_arg_type: ArgType::VecBool,
//         value: None,
//         when_failed: WhenFailed::Continue,
//     };

//     repl.run();

//     println!("{:?}", repl);
// }

// fn dsafdsaf(o: Option<String>) {
//     match o {
//         Some(json) => {
//             // 命令行程序的使用者个此命令传入了一个 JSON,
//             // 需要将此 JSON 转换为需要的数据.
//         }
//         None => {
//             // 问答式命令行交互 来获取所需要的 参数们.
//         }
//     };
// }

// fn asdfasdf() {
//     let mut a = true;
//     ArgType::Bool(Rc::new(|x| {}));

//     asdfsdafasdff(|| {
//         a = false;
//     })
// }

// fn asdfsdafasdff<F>(mut f: F)
// where
//     F: FnMut() -> (),
// {
//     f();
// }

//     // yes or no QA
//     // true or false QA
//     // number QA
//     // vec<number> QA
//     // string QA
//     // Vec<string> QA
//     // path QA
//     // Vec<path> QA
//     // password QA whith confirm

//     // enum single selection QA
//     // enum multi selection QA

//     // path single selection QA
//     // path multi selection QA

/// ArgType::Repl(_) 需要用到 ReplQuestions.
pub struct ReplQuestions {
    /// 从 json_str 转换过来的 Vec<String>.
    /// 也可能是通过 问答式命令行交互 获取到的 Vec<String>.
    arr: Vec<String>,

    /// 当 Self 是从 json_str 转换过来的 Vec<String> 时,
    /// 这个用户标记读取到了哪一个参数.
    index: usize,

    /// 是否是从 json_str 转换过来的?
    is_from_json: bool,
}

impl ReplQuestions {
    /* private */
    pub fn new(input: Option<String>) -> Self {
        match input {
            Some(s) => Self::new_from_jsonstr(s),
            None => Self {
                arr: vec![],
                index: 0,
                is_from_json: false,
            },
        }
    }

    pub fn new_from_jsonstr(str: String) -> Self {
        // str -> Vec<String>

        let parse_result = VecString::json_to_vec(&str);
        match parse_result {
            Ok(v) => {
                return Self {
                    arr: v,
                    index: 0,
                    is_from_json: true,
                };
            }
            Err(_e) => {
                debug_run(|| {
                    println!("转换为 json 是出错: {}", _e);
                });

                return Self {
                    arr: vec![],
                    index: 0,
                    is_from_json: false,
                };
            }
        }
    }

    /// 转换为 json 字符串.
    pub fn to_json_str(&self) -> String {
        return VecString::vec_to_json(&self.arr);
    }
}

impl ReplQuestions {
    fn req_string(self, result_value: &mut String, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            println!("is_from_json");
            let val = re.arr.get(re.index);
            _ = result_value;

            match val {
                Some(str) => {
                    // 成功获取到了需要的参数
                    *result_value = str.clone();
                    _ = result_value;

                    re.index += 1;
                    return re;
                }
                None => {
                    // not string
                }
            }
        }

        *result_value = Dialog::get_string(prompt);
        re.index += 1;
        return re;
    }

    fn req_multiple_string(self, result_value: &mut Vec<String>, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arr.get(re.index);

            if let Some(json_str) = val {
                let result = VecString::json_to_vec(&json_str);
                match result {
                    Ok(vec_str_from_str) => {
                        *result_value = vec_str_from_str;
                        re.index += 1;
                        return re;
                    }
                    Err(_e) => {
                        eprintln!("{}", _e.red());

                        // TODO: remove the panic!.
                        panic!();
                    }
                }
            }
        }

        *result_value = Dialog::get_multiple_str(prompt);

        let string = serde_json::to_string(result_value).unwrap();

        re.arr.push(string);
        re.index += 1;

        return re;
    }

    fn req_number(self, result_value: &mut super::arg_types::Number, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arr.get(re.index);

            if let Some(str) = val {
                let number_from_str: Result<super::arg_types::Number, std::num::ParseIntError> =
                    str.parse();

                if let Ok(x) = number_from_str {
                    // 成功获取到了需要的参数
                    *result_value = x;

                    re.index += 1;
                    return re;
                }
            }
        }

        // get value from REPL.

        *result_value = Dialog::get_number(prompt);
        re.arr.push(result_value.to_string());

        re.index += re.arr.len() - 1;
        return re;
    }

    // fn req_multiple_number(
    //     self,
    //     result_value: &mut Vec<super::arg_types::Number>,
    //     prompt: &str,
    // ) -> Self {

    // }

    fn req_bool(self, result_value: &mut bool, prompt: &str) -> Self {
        let mut re = self;
        if re.is_from_json {
            let val = re.arr.get(re.index);

            if let Some(str) = val {
                if str == "true" {
                    *result_value = true;
                } else if str == "false" {
                    *result_value = false;
                }
                re.index += 1;
                return re;
            }
        }

        // get value from REPL.

        *result_value = Dialog::get_bool(prompt);

        re.arr.push(result_value.to_string()); // -> "true" or "false"

        re.index += re.arr.len() - 1;
        return re;
    }

    // fn req_multiple_bool(self, result_value: &mut Vec<bool>, prompt: &str) -> Self {}

    fn req_path(self, result_value: &mut PathBuf, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arr.get(re.index);
            if let Some(str) = val {
                *result_value = Path::new(&str).to_path_buf();
                re.index += 1;
                return re;
            }
        }

        // get value from REPL.

        let str = Dialog::get_string(prompt);

        *result_value = Path::new(&str).to_path_buf();

        re.arr.push(str); // -> "true" or "false"
        re.index = re.arr.len() - 1;
        return re;
    }

    fn req_multiple_path(self, result_value: &mut Vec<PathBuf>, prompt: &str) -> Self {
        let mut r: Vec<String> = vec![];
        let re = self.req_multiple_string(&mut r, prompt);

        *result_value = vec![];

        for x in r {
            result_value.push(Path::new(&x).to_path_buf());
        }

        return re;
    }
}

#[cfg(test)]
mod test_repl_questions {
    use owo_colors::OwoColorize;
    use std::default;

    use super::*;

    #[test]
    fn it_works() {
        let mut x: super::arg_types::Number = Default::default();

        let r = ReplQuestions::new(None).req_number(&mut x, "你想买几个汉堡?");

        let r2 = ReplQuestions::new_from_jsonstr(r#"["100"]"#.to_string())
            .req_number(&mut x, "你想买几个汉堡?");
        println!("x 的值是: {}", x);
        println!("r: {:?}\nr2: {:?}", r.arr.clone(), r2.arr.clone());
        assert_eq!(r.arr, r2.arr);
    }

    #[test]
    fn repl_questions_json_vec_是否能相互逆转() {
        let v1 = vec!["hello".to_string(), "wo\"rld".to_string()];

        let json_str = VecString::vec_to_json(&v1);

        let v2 = VecString::json_to_vec(&json_str).expect("json to VecString 失败");

        println!("json_str: {:?}", json_str.cyan());
        println!("v1: {:?}\nv2: {:?}", v1.cyan(), v2.cyan());
        println!("v1 == v2  -> {}  ", v1 == v2); // true 可以还原

        assert_eq!(v1, v2);

        let r = ReplQuestions::new(Some(json_str.clone()));
        let json_str2 = r.to_json_str();

        println!("v1 == v2  -> {}  ", json_str == json_str2);
        assert_eq!(json_str, json_str2);
    }

    #[test]
    fn test_req_string() {
        {
            let mut x = String::new();

            let repl = ReplQuestions::new(None).req_string(&mut x, "");

            println!("输入的是: {:?}", x);
            assert_eq!(repl.is_from_json, false);
        }

        {
            let mut x = String::new();
            let repl = ReplQuestions::new(Some(r#"["hello"]"#.to_string())).req_string(&mut x, "");

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn test_req_multiple_string() {
        // 已测试, 可以逆转.

        // {
        //     let mut x: Vec<String> = vec![];

        //     let repl = ReplQuestions::new(None).req_multiple_string(&mut x, "");

        //     println!("输入的是: {:?}", x);
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x: Vec<String> = vec![];
            // let repl = ReplQuestions::new(Some(r#"    [ "\"[\"sa dfadsf\",\"sadfadsf\",\"sa dfadsf\"]\""]  "#.to_string()))
            let repl = ReplQuestions::new(Some(r#" ["[\"asdfasdf\",\"sadfsadf\"]"] "#.to_string()))
                .req_multiple_string(&mut x, "");

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn test_req_bool() {
        // 已测试, 可以逆转.

        // {
        //     let mut x: bool = true;

        //     let repl = ReplQuestions::new(None).req_bool(&mut x, "get an bool");

        //     println!("输入的是: {:?}", x);

        //     println!("json_str: {}", repl.to_json_str());
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x: bool = true;
            // let repl = ReplQuestions::new(Some(r#"    [ "\"[\"sa dfadsf\",\"sadfadsf\",\"sa dfadsf\"]\""]  "#.to_string()))
            let repl = ReplQuestions::new(Some(r#"   ["false"]    "#.to_string()))
                .req_bool(&mut x, "get an bool");

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn tese_req_path() {
        // 已测试, 可以逆转.

        // {
        //     let mut x: PathBuf = PathBuf::new();

        //     let repl = ReplQuestions::new(None).req_path(&mut x, "get an path");

        //     println!("输入的是: {:?}", x);

        //     println!("json_str: {}", repl.to_json_str());
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x: PathBuf = PathBuf::new();

            let repl = ReplQuestions::new(Some(r#"  ["./hello/sadf.txt"]   "#.to_string()))
                .req_path(&mut x, "get an bool");

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn test_req_multiple_path() {
        // 已测试, 可以逆转.

        // {
        //     let mut x: Vec<PathBuf> = vec![];

        //     let repl = ReplQuestions::new(None).req_multiple_path(&mut x, "get mutiple path");

        //     println!("输入的是: {:?}", x);

        //     println!("json_str: {}", repl.to_json_str());
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x: Vec<PathBuf> = vec![];

            let repl = ReplQuestions::new(Some(r#" ["[\"a\",\"b.txt\",\"./\"]"]  "#.to_string()))
                .req_multiple_path(&mut x, "get mutiple path");

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }
}

// ------- REPL Functions -------

pub struct Dialog();
impl Dialog {
    /// 示例:
    /// ```rust
    ///    let a = Dialog::get_string("你要吃几个汉堡?").unwrap();
    ///    println!("最终获得的数字是: {}", a);
    /// ```
    pub fn get_string(prompt: &str) -> String {
        let re = dialoguer::Input::<String>::with_theme(&ColoredTheme {})
            .with_prompt(prompt)
            .interact_text();

        match re {
            Ok(s) => {
                return s;
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return Dialog::get_string(prompt); // 继续本次问题
            }
        }
    }

    /// ```rust
    ///     let arr = Dialog::get_multiple_str("hello");
    ///     println!("{:?}", arr);
    /// ``````
    pub fn get_multiple_str(prompt: &str) -> Vec<String> {
        println!("{}", prompt.bright_green());

        let re = dialoguer::Input::<String>::with_theme(&ColoredTheme {})
            .with_prompt(prompt)
            .interact_text();

        match re {
            Ok(input) => {
                return parse_arg_string(&input);
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return Dialog::get_multiple_str(prompt); // 继续本次问题
            }
        }
    }

    /// 示例:
    /// ```rust
    ///     let a = Dialog::get_number("你要吃几个汉堡?");
    ///     println!("最终获得的数字是: {}", a);
    /// ```
    pub fn get_number(prompt: &str) -> super::Number {
        println!("{}", prompt.bright_green());

        let mut input = "".to_string();
        let re = std::io::stdin().read_line(&mut input);

        match re {
            Ok(_n) => {
                let input = input.trim();
                // 用户说输入了某些东西
                let parse_result: Result<super::Number, ParseIntError> = input.parse();
                // println!("input is: {}", input);

                match parse_result {
                    Ok(num) => {
                        return num;
                    }
                    Err(_e) => {
                        let err_message = format!("{}", _e).red().to_string();
                        eprintln!("{}", err_message);

                        // TODO: 打印正确的书写方式;
                        println!("需要输入一个数字, 示例: 123  ");
                        return Dialog::get_number(prompt); // 继续本次问题
                    }
                };
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return Dialog::get_number(prompt); // 继续本次问题
            }
        }
    }

    /// 示例:
    /// ```rust
    ///     let b = ReplFunctions::repl_req_bool("test_repl_req_bool").expect("获取 bool 是出错");
    ///     println!("最终获得的数字是: {:?}", b);
    /// ```
    pub fn get_bool(prompt: &str) -> bool {
        println!("{}", prompt.bright_green());

        let re = dialoguer::Confirm::with_theme(&ColoredTheme {})
            .with_prompt("Y 键 N 键选择, 回车键确认: ")
            .wait_for_newline(true)
            .interact();

        match re {
            Ok(b) => {
                return b;
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return Dialog::get_bool(prompt); // 继续本次问题
            }
        }
    }

    /// 示例子:
    /// ```rust
    ///     use chenbao_cmd::Dialog;
    ///     let items = vec!["foo", "bar", "baz"];
    ///     let b = Dialog::get_single_selected("prompt", &items);
    ///     println!("最终获得的数字是: {:?}", b);
    /// ```
    pub fn get_single_selected<'a, T>(prompt: &str, items: &'a [T]) -> &'a T
    where
        T: ToString + Clone,
    {
        println!("{}", prompt.bright_green());

        let re = dialoguer::Select::with_theme(&ColoredTheme {})
            .with_prompt("What do you choose?")
            .items(&items)
            .interact();

        match re {
            Ok(selection) => {
                return &(items[selection]);
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return Dialog::get_single_selected(prompt, &items); // 继续本次问题
            }
        }
    }

    /// 示例:
    /// ```rust
    ///     let items = vec!["foo", "bar", "baz"];
    ///     let b = Dialog::get_multiple_selected("prompt", &items);
    ///     println!("最终获得的数字是: {:?}", b);
    /// ```
    pub fn get_multiple_selected<T>(prompt: &str, items: &[T]) -> Vec<T>
    where
        T: ToString + Clone,
    {
        println!("{}", prompt.bright_green());

        let re = dialoguer::MultiSelect::with_theme(&ColoredTheme {})
            .with_prompt("What do you choose?")
            .items(&items)
            .interact();

        match re {
            Ok(selection) => {
                let mut re: Vec<T> = vec![];

                for i in selection {
                    re.push(items[i].clone());
                }

                return re;
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return Dialog::get_multiple_selected(prompt, &items); // 继续本次问题
            }
        }
    }

    /// Launches the editor to edit a string.  
    ///  
    /// Returns `None` if the file was not saved or otherwise the  
    /// entered text.  
    /// 示例:  
    /// ```rust
    ///    let b = Dialog::editor("prompt").unwrap();
    ///    println!("最终获得的数字是: {:?}", b);
    /// ```
    pub fn editor(prompt: &str) -> Option<String> {
        let re = dialoguer::Editor::new().edit(prompt);

        match re {
            Ok(ostr) => return ostr,
            Err(_e) => {
                eprintln!("{}", _e.red());
                return None;
            }
        }
    }
}

#[cfg(test)]
mod test_repl_functions {

    use super::*;

    #[test]
    fn test_get_string() {
        let a = Dialog::get_string("请输入一个字符串");
        println!("最终获得的 string 是: {}", a);
    }

    #[test]
    fn test_repl_get_number() {
        let a = Dialog::get_number("你要吃几个汉堡?");
        println!("最终获得的数字是: {}", a);
    }

    #[test]
    fn test_repl_get_multiple_string() {
        let arr = Dialog::get_multiple_str("请输入多个字符串");
        println!("{:?}", arr);
    }

    #[test]
    fn test_repl_req_bool() {
        let b = Dialog::get_bool("test_repl_req_bool");
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_repl_req_string() {
        let b = Dialog::get_string("请输入一个字符串");
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_get_single_selected() {
        let items = vec!["foo", "bar", "baz"];

        let b = Dialog::get_single_selected("prompt", &items);
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_get_multiple_selected() {
        let items = vec!["foo", "bar", "baz"];
        let b = Dialog::get_multiple_selected("prompt", &items);
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_edit() {
        let b = Dialog::editor("prompt").unwrap();
        println!("最终获得的数字是: {:?}", b);
    }
}
