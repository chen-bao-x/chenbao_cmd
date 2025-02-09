use owo_colors::OwoColorize;
use std::{fs::read_link, num::ParseIntError, path::Path, vec};

use super::*;

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
    arguments: Vec<String>,

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
                arguments: vec![],
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
                    arguments: v,
                    index: 0,
                    is_from_json: true,
                };
            }
            Err(_e) => {
                debug_run(|| {
                    println!("转换为 json 是出错: {}", _e);
                });

                return Self {
                    arguments: vec![],
                    index: 0,
                    is_from_json: false,
                };
            }
        }
    }

    /// 转换为 json 字符串.
    pub fn to_json_str(&self) -> String {
        return VecString::vec_to_json(&self.arguments);
    }
}

impl ReplQuestions {
    fn req_string(self, result_value: &mut String, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            println!("is_from_json");
            let val = re.arguments.get(re.index);
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
            let val = re.arguments.get(re.index);

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

        *result_value = Dialog::get_string_multiple(prompt);

        // serde_json::to_string(_) 这个函数转换出来的 json——string 是带有转义符号 '\' 的.
        let string = serde_json::to_string(result_value).unwrap();

        re.arguments.push(string);
        re.index += 1;

        return re;
    }

    fn req_number(self, result_value: &mut super::arg_types::Number, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arguments.get(re.index);

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
        re.arguments.push(result_value.to_string());

        re.index += re.arguments.len() - 1;
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
            let val = re.arguments.get(re.index);

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

        re.arguments.push(result_value.to_string()); // -> "true" or "false"

        re.index += re.arguments.len() - 1;
        return re;
    }

    fn req_bool_multiple(self, result_value: &mut Vec<bool>, prompt: &str) -> Self {
        let mut multiple_string: Vec<String> = vec![];
        let re = self.req_multiple_string(&mut multiple_string, prompt);

        {
            /* 为 result_value 赋值. */

            *result_value = vec![];
            for str in multiple_string {
                if str == "true" {
                    result_value.push(true);
                } else if str == "false" {
                    result_value.push(false);
                } else {
                    eprintln!("需要的是多个 bool 类型的值, 示例: true false true");

                    let mut asdf = re;
                    asdf.arguments.pop();
                    return asdf.req_bool_multiple(result_value, prompt);
                }
            }
        }

        return re;
    }

    fn req_path(self, result_value: &mut PathBuf, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arguments.get(re.index);
            if let Some(str) = val {
                *result_value = Path::new(&str).to_path_buf();
                re.index += 1;
                return re;
            }
        }

        // get value from REPL.

        let str = Dialog::get_string(prompt);

        *result_value = Path::new(&str).to_path_buf();

        re.arguments.push(str); // -> "true" or "false"
        re.index = re.arguments.len() - 1;
        return re;
    }

    fn req_multiple_path(self, result_value: &mut Vec<PathBuf>, prompt: &str) -> Self {
        let mut r: Vec<String> = vec![];
        let re = self.req_multiple_string(&mut r, prompt);

        {
            /* 为 result_value 赋值. */

            *result_value = vec![];
            for x in r {
                result_value.push(Path::new(&x).to_path_buf());
            }
        }

        return re;
    }

    fn req_multiple_select(
        self,
        result_value: &mut Vec<String>,
        items: Vec<&str>,
        prompt: &str,
    ) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arguments.get(re.index);
            if let Some(str) = val {
                *result_value = VecString::json_to_vec(&str).unwrap();
                re.index += 1;
                return re;
            }
        }

        // get value from REPL.

        let mut str = Dialog::get_multiple_selected(prompt, &items);

        *result_value = str.iter().map(|x| x.to_string()).collect();

        let json_string = serde_json::to_string(&str).unwrap();

        re.arguments.push(json_string);
        re.index = re.arguments.len() - 1;
        return re;
    }

    fn req_single_select(self, result_value: &mut String, items: Vec<&str>, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arguments.get(re.index);
            if let Some(str) = val {
                *result_value = str.to_string();
                re.index += 1;
                return re;
            }
        }

        // get value from REPL.

        let mut str = Dialog::get_single_selected(prompt, &items);

        *result_value = str.to_string();

        re.arguments.push(str.to_string());
        re.index = re.arguments.len() - 1;
        return re;
    }
}

#[cfg(test)]
mod test_repl_questions {
    use owo_colors::OwoColorize;
    use std::{default, iter};

    use super::*;

    #[test]
    fn it_works() {
        let mut x: super::arg_types::Number = Default::default();

        let r = ReplQuestions::new(None).req_number(&mut x, "你想买几个汉堡?");

        let r2 = ReplQuestions::new_from_jsonstr(r#"["100"]"#.to_string())
            .req_number(&mut x, "你想买几个汉堡?");
        println!("x 的值是: {}", x);
        println!(
            "r: {:?}\nr2: {:?}",
            r.arguments.clone(),
            r2.arguments.clone()
        );
        assert_eq!(r.arguments, r2.arguments);
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

    #[test]
    fn test_req_multiple_bool() {
        // 已测试, 可以逆转.

        {
            let mut x: Vec<bool> = vec![];

            let repl = ReplQuestions::new(None).req_bool_multiple(&mut x, "get mutiple bool");

            println!("输入的是: {:?}", x);

            println!("json_str: {}", repl.to_json_str());
            assert_eq!(repl.is_from_json, false);
        }

        {
            let mut x: Vec<bool> = vec![];

            let repl = ReplQuestions::new(Some(r#" ["[\"true\",\"false\"]"]  "#.to_string()))
                .req_bool_multiple(&mut x, "get mutiple path");

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn test_req_selected() {
        // 已测试, 可以逆转.

        // {
        //     let mut x: String = "".to_string();
        //     let iterms = vec!["one", "two"];

        //     let repl =
        //         ReplQuestions::new(None).req_single_select(&mut x, iterms, "get mutiple bool");

        //     println!("输入的是: {:?}", x);

        //     println!("json_str: {}", repl.to_json_str());
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x: String = "".to_string();
            let iterms = vec!["one", "two"];

            let repl = ReplQuestions::new(Some(r#" ["two"] "#.to_string())).req_single_select(
                &mut x,
                iterms,
                "get mutiple path",
            );

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn test_req_multiple_select() {
        // 已测试, 可以逆转.

        // {
        //     let mut x: Vec<String> = vec![];
        //     let iterms = vec!["one", "two"];

        //     let repl =
        //         ReplQuestions::new(None).req_multiple_select(&mut x, iterms, "get mutiple bool");

        //     println!("输入的是: {:?}", x);

        //     println!("json_str: {}", repl.to_json_str());
        //     assert_eq!(repl.is_from_json, false);
        // } 

        {
            let mut x: Vec<String> = vec![];
            let iterms = vec!["one", "two"];

            let repl = ReplQuestions::new(Some(r#" ["[\"one\",\"two\"]"] "#.to_string())).req_multiple_select(
                &mut x,
                iterms,
                "get mutiple path",
            );

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
    pub fn get_string_multiple(prompt: &str) -> Vec<String> {
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
                return Dialog::get_string_multiple(prompt); // 继续本次问题
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
            .default(0)
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
        let arr = Dialog::get_string_multiple("请输入多个字符串");
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
