use owo_colors::OwoColorize;
use std::{num::ParseIntError, path::Path, vec};

use super::*;

/// ArgType::Repl(_) 需要用到 ReplQuestions.
pub struct DialogGenerator {
    /// 从 json_str 转换过来的 Vec<String>.
    /// 也可能是通过 问答式命令行交互 获取到的 Vec<String>.
    pub arguments: Vec<String>,

    /// 当 Self 是从 json_str 转换过来的 Vec<String> 时,
    /// 这个用户标记读取到了哪一个参数.
    pub index: usize,

    /// 是否是从 json_str 转换过来的?
    pub is_from_json: bool,
}

impl DialogGenerator {
    /* private */

    /// Creates a new [`ReplQuestions`].
    /// * `input` :
    ///     1. 如果是 `None`, 则会在命令行里要求用户来提交所需要的参数.  
    ///     2. 如果是 `Some(json_string)`, 则会直接解析并返回所需参数.
    pub fn new(input: Option<&str>) -> Self {
        match input {
            Some(s) => Self::new_from_jsonstr(s),
            None => Self {
                arguments: vec![],
                index: 0,
                is_from_json: false,
            },
        }
    }

    pub fn new_from_jsonstr(str: &str) -> Self {
        // &str -> ReplQuestions

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

impl DialogGenerator {
    pub fn string(self, result_value: &mut String, prompt: &str) -> Self {
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

        *result_value = DialogGeter::get_string(prompt);

        re.arguments.push(result_value.clone());
        re.index = re.arguments.len() - 1;
        return re;
    }

    pub fn string_multiple(self, result_value: &mut Vec<String>, prompt: &str) -> Self {
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

        *result_value = DialogGeter::get_string_multiple(prompt);

        let string = serde_json::to_string(result_value).unwrap();

        re.arguments.push(string);
        re.index = re.arguments.len() - 1;
        return re;
    }

    pub fn number(self, result_value: &mut super::arg_type::Number, prompt: &str) -> Self {
        let mut re = self;

        if re.is_from_json {
            let val = re.arguments.get(re.index);

            if let Some(str) = val {
                let number_from_str: Result<arg_type::Number, std::num::ParseIntError> =
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

        *result_value = DialogGeter::get_number(prompt);

        re.arguments.push(result_value.to_string());
        re.index += re.arguments.len() - 1;
        return re;
    }

    pub fn req_multiple_number(
        self,
        result_value: &mut arg_type::NumberMutiple,
        prompt: &str,
    ) -> Self {
        let mut multiple_string: Vec<String> = vec![];
        let re = self.string_multiple(&mut multiple_string, prompt);

        {
            /* 为 result_value 赋值. */

            *result_value = vec![];
            for str in multiple_string {
                let number_from_str: Result<arg_type::Number, std::num::ParseIntError> =
                    str.parse();

                if let Ok(x) = number_from_str {
                    // 成功获取到了需要的参数
                    result_value.push(x);
                } else {
                    eprintln!("需要的是多个 bool 类型的值, 示例: true false true");

                    let mut asdf = re;
                    asdf.arguments.pop(); // 清理 self.string_multiple(_) 添加的东西.
                    asdf.index = asdf.arguments.len() - 1;
                    return asdf.req_multiple_number(result_value, prompt);
                }
            }
        }

        // re.index += 1;

        return re;
    }

    pub fn yes_or_no(self, result_value: &mut bool, prompt: &str) -> Self {
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

        *result_value = DialogGeter::get_bool(prompt);

        re.arguments.push(result_value.to_string()); // -> "true" or "false"
        re.index += re.arguments.len() - 1;
        return re;
    }

    pub fn yes_or_no_multiple(self, result_value: &mut Vec<bool>, prompt: &str) -> Self {
        let mut multiple_string: Vec<String> = vec![];
        let re = self.string_multiple(&mut multiple_string, prompt);

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
                    asdf.arguments.pop(); // 清理 self.string_multiple(_) 添加的东西.
                    return asdf.yes_or_no_multiple(result_value, prompt);
                }
            }
        }

        return re;
    }

    pub fn path(self, result_value: &mut arg_type::Path, prompt: &str) -> Self {
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

        let str = DialogGeter::get_string(prompt);

        *result_value = Path::new(&str).to_path_buf();

        re.arguments.push(str); // -> "true" or "false"
        re.index = re.arguments.len() - 1;
        return re;
    }

    pub fn path_multiple(self, result_value: &mut arg_type::PathMutiple, prompt: &str) -> Self {
        let mut r: Vec<String> = vec![];
        let re = self.string_multiple(&mut r, prompt);

        {
            /* 为 result_value 赋值. */

            *result_value = vec![];
            for x in r {
                result_value.push(Path::new(&x).to_path_buf());
            }
        }

        return re;
    }

    pub fn select(self, result_value: &mut String, items: &Vec<&str>, prompt: &str) -> Self {
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

        let str = DialogGeter::get_single_selected(prompt, items);

        *result_value = str.to_string();

        re.arguments.push(str.to_string());
        re.index = re.arguments.len() - 1;
        return re;
    }

    pub fn select_multiple(
        self,
        result_value: &mut Vec<String>,
        items: &Vec<&str>,
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

        let str = DialogGeter::get_multiple_selected(prompt, &items);

        *result_value = str.iter().map(|x| x.to_string()).collect();

        let json_string = serde_json::to_string(&str).unwrap();

        re.arguments.push(json_string);
        re.index = re.arguments.len() - 1;
        return re;
    }

    pub fn editor(self, result_value: &mut String, prompt: &str) -> Self {
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

        *result_value = DialogGeter::editor(prompt);
        re.arguments.push(result_value.to_string());
        re.index = re.arguments.len() - 1;
        return re;
    }

    /// 让用户手动输入密码.
    pub fn password(prompt: &str) -> String {
        DialogGeter::password(prompt)
    }

    pub fn password_with_confirmation(prompt: &str) -> String {
        DialogGeter::password_with_confirmation(prompt)
    }
}

#[cfg(test)]
mod test_repl_questions {

    use owo_colors::OwoColorize;

    use super::*;

    // #[test]
    // fn it_works() {
    //     let mut x: super::arg_types::Number = Default::default();
    //
    //     let hand_input = ReplQuestions::new(None).req_number(&mut x, "你想买几个汉堡?");
    //
    //     let from_json_string = ReplQuestions::new_from_jsonstr(r#"["100"]"#.to_string())
    //         .req_number(&mut x, "你想买几个汉堡?");
    //     println!("x 的值是: {}", x);
    //     println!(
    //         "r: {:?}\nr2: {:?}",
    //         hand_input.arguments.clone(),
    //         from_json_string.arguments.clone()
    //     );
    //     assert_eq!(hand_input.arguments, from_json_string.arguments);
    // }

    #[test]
    fn repl_questions_json_vec_是否能相互逆转() {
        let v1 = vec!["hello".to_string(), "wo\"rld".to_string()];

        let json_str = VecString::vec_to_json(&v1);

        let v2 = VecString::json_to_vec(&json_str).expect("json to VecString 失败");

        println!("json_str: {:?}", json_str.cyan());
        println!("v1: {:?}\nv2: {:?}", v1.cyan(), v2.cyan());
        println!("v1 == v2  -> {}  ", v1 == v2); // true 可以还原

        assert_eq!(v1, v2);

        let r = DialogGenerator::new(Some(&json_str));
        let json_str2 = r.to_json_str();

        println!("v1 == v2  -> {}  ", json_str == json_str2);
        assert_eq!(json_str, json_str2);
    }

    #[test]
    fn test_req_string() {
        // {
        //     let mut x = String::new();
        //
        //     let repl = ReplQuestions::new(None).req_string(&mut x, "");
        //
        //     println!("输入的是: {:?}", x);
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x = String::new();
            let repl = DialogGenerator::new(Some(r#"["hello"]"#)).string(&mut x, "");

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
            let repl = DialogGenerator::new(Some(r#" ["[\"asdfasdf\",\"sadfsadf\"]"] "#))
                .string_multiple(&mut x, "");

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
            let repl =
                DialogGenerator::new(Some(r#"   ["false"]    "#)).yes_or_no(&mut x, "get an bool");

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn test_req_multiple_bool() {
        // 已测试, 可以逆转.

        // {
        //     let mut x: Vec<bool> = vec![];

        //     let repl = ReplQuestions::new(None).req_bool_multiple(&mut x, "get mutiple bool");

        //     println!("输入的是: {:?}", x);

        //     println!("json_str: {}", repl.to_json_str());
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x: Vec<bool> = vec![];

            let repl = DialogGenerator::new(Some(r#" ["[\"true\",\"false\"]"]  "#))
                .yes_or_no_multiple(&mut x, "get mutiple path");

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
            let mut x: arg_type::Path = arg_type::Path::new();

            let repl = DialogGenerator::new(Some(r#"  ["./hello/sadf.txt"]   "#))
                .path(&mut x, "get an bool");

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
            let mut x: arg_type::PathMutiple = vec![];

            let repl = DialogGenerator::new(Some(r#" ["[\"a\",\"b.txt\",\"./\"]"]  "#))
                .path_multiple(&mut x, "get mutiple path");

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

            let repl = DialogGenerator::new(Some(r#" ["two"] "#)).select(
                &mut x,
                &iterms,
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

            let repl = DialogGenerator::new(Some(r#" ["[\"one\",\"two\"]"] "#)).select_multiple(
                &mut x,
                &iterms,
                "get mutiple path",
            );

            println!("输入的是: {:?}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }

    #[test]
    fn test_整体是否能还原() {
        // 已测试, 可以逆转.

        //         {
        //             let mut did_like_green: bool = false;
        //             let mut eat_howmuch_hanbager: arg_types::Number = 0;
        //             let mut 配菜: Vec<String> = vec![];
        //             let all配菜 = vec!["生菜", "蕃茄酱", "西红柿片"];

        //             let repl = ReplQuestions::new(None)
        //                 .req_bool(&mut did_like_green, "喜欢绿色吗?")
        //                 .req_number(&mut eat_howmuch_hanbager, "吃几个汉堡?")
        //                 .req_multiple_select(&mut 配菜, all配菜, "需要哪些配菜?");

        //             println!(
        //                 r#"
        // did_like_green: {}
        // eat_howmuch_hanbager: {}
        // 配菜: {:?}
        //         "#,
        //                 did_like_green, eat_howmuch_hanbager, 配菜
        //             );

        //             println!("json_str: {}", repl.to_json_str());

        //             assert_eq!(repl.is_from_json, false);
        //         }

        {
            let mut did_like_green: bool = false;
            let mut eat_howmuch_hanbager: arg_type::Number = 0;
            let mut 配菜: Vec<String> = vec![];
            let all配菜 = vec!["生菜", "蕃茄酱", "西红柿片"];

            let repl = DialogGenerator::new(Some(
                r#"        ["true","8","[\"生菜\",\"西红柿片\"]"]       "#,
            ))
            .yes_or_no(&mut did_like_green, "喜欢绿色吗?")
            .number(&mut eat_howmuch_hanbager, "吃几个汉堡?")
            .select_multiple(&mut 配菜, &all配菜, "需要哪些配菜?");

            println!(
                r#"
did_like_green: {}
eat_howmuch_hanbager: {}
配菜: {:?}
        "#,
                did_like_green, eat_howmuch_hanbager, 配菜
            );

            println!("json_str: {}", repl.to_json_str());

            assert_eq!(repl.is_from_json, true);
        }

        // ["true","8","[\"生菜\",\"西红柿片\"]"]
    }

    #[test]
    fn test_editor() {
        // 已测试, 可以逆转.

        // {
        //     let mut x = String::new();

        //     let repl = Dialog::new(None).editor(&mut x, "testing editor");

        //     println!("输入的是: {:?}", x);

        //     println!("json_str: {}", repl.to_json_str());
        //     assert_eq!(repl.is_from_json, false);
        // }

        {
            let mut x = String::new();
            let repl = DialogGenerator::new(Some(r#"      ["aaaasdfdsaf jsdal;fj lsd;kjf lksdjafl jsadl jflsa;djk f saj;df\nsadf\nas \ndf\ns a\nf \nsad\nf \nsad\nf \nsadf\n \nsadf \ns\nadf \nsad\nf \nsa\ndf  \\sadf\\sad \\f\\sadf\\\\sdaf\\\\'\\'\\'\\'\\'\\'\\'\n asd\nf\nsdaf\nsa\ndf\ndsa\nf\n\n\n\n\nasdf\n s\nadf\n sa\ndf\n as\ndf\n as\nd"]               "#)).string(&mut x, "");

            println!("输入的是: {}", x);

            assert_eq!(repl.is_from_json, true);
        }
    }
}

// ------- REPL Functions -------

struct DialogGeter();
impl DialogGeter {
    /// 示例:
    /// ```rust
    ///    let a = Dialog::get_string("你要吃几个汉堡?").unwrap();
    ///    println!("最终获得的数字是: {}", a);
    /// ```
    fn get_string(prompt: &str) -> String {
        let re = dialoguer::Input::<String>::with_theme(&ColoredTheme::new())
            .with_prompt(prompt)
            .interact_text();

        match re {
            Ok(s) => {
                return s;
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return DialogGeter::get_string(prompt); // 继续本次问题
            }
        }
    }

    /// ```rust
    ///     let arr = Dialog::get_multiple_str("hello");
    ///     println!("{:?}", arr);
    /// ``````
    fn get_string_multiple(prompt: &str) -> Vec<String> {
        let re = dialoguer::Input::<String>::with_theme(&ColoredTheme::new())
            .with_prompt(prompt)
            .interact_text();

        match re {
            Ok(input) => {
                return parse_arg_string(&input);
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return DialogGeter::get_string_multiple(prompt); // 继续本次问题
            }
        }
    }

    /// 示例:
    /// ```rust
    ///     let a = Dialog::get_number("你要吃几个汉堡?");
    ///     println!("最终获得的数字是: {}", a);
    /// ```
    fn get_number(prompt: &str) -> arg_type::Number {
        let input = DialogGeter::get_string(prompt);
        let input = input.trim();
        // 用户说输入了某些东西
        let parse_result: Result<arg_type::Number, ParseIntError> = input.parse();

        match parse_result {
            Ok(num) => {
                return num;
            }
            Err(_e) => {
                let err_message = format!("{}", _e).red().to_string();
                eprintln!("{}", err_message);

                println!("需要输入一个数字, 示例: {x}", x = "123".styled_arg());
                return DialogGeter::get_number(prompt); // 继续本次问题
            }
        };
    }

    /// 示例:
    /// ```rust
    ///     let b = ReplFunctions::repl_req_bool("test_repl_req_bool").expect("获取 bool 是出错");
    ///     println!("最终获得的数字是: {:?}", b);
    /// ```
    fn get_bool(prompt: &str) -> bool {
        let re = dialoguer::Confirm::with_theme(&ColoredTheme::new())
            // .with_prompt("Y 键 N 键选择, 回车键确认: ")
            .with_prompt(prompt)
            .wait_for_newline(true)
            .interact();

        match re {
            Ok(b) => {
                return b;
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return DialogGeter::get_bool(prompt); // 继续本次问题
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
    fn get_single_selected<'a, T>(prompt: &str, items: &'a [T]) -> &'a T
    where
        T: ToString + Clone,
    {
        let re = dialoguer::FuzzySelect::with_theme(&ColoredTheme::new())
            // .with_prompt("What do you choose?")
            .with_prompt(prompt)
            .items(&items)
            .default(0)
            .interact();

        match re {
            Ok(selection) => {
                return &(items[selection]);
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                return DialogGeter::get_single_selected(prompt, &items); // 继续本次问题
            }
        }
    }

    /// 示例:
    /// ```rust
    ///     let items = vec!["foo", "bar", "baz"];
    ///     let b = Dialog::get_multiple_selected("prompt", &items);
    ///     println!("最终获得的数字是: {:?}", b);
    /// ```
    fn get_multiple_selected<T>(prompt: &str, items: &[T]) -> Vec<T>
    where
        T: ToString + Clone,
    {
        let re = dialoguer::MultiSelect::with_theme(&ColoredTheme::new())
            // .with_prompt("What do you choose?")
            .with_prompt(prompt)
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
                return DialogGeter::get_multiple_selected(prompt, &items); // 继续本次问题
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
    fn editor(prompt: &str) -> arg_type::String {
        let re = dialoguer::Editor::new().edit(prompt);

        match re {
            Ok(ostr) => {
                return ostr.unwrap_or("".to_string());
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                // panic!("---------------------");
                return "".to_string();
            }
        }
    }

    fn password(prompt: &str) -> String {
        let prompt_message = if prompt == "" {
            "input password"
        } else {
            prompt
        };

        let re = dialoguer::Password::with_theme(&ColoredTheme::new())
            .with_prompt(prompt_message)
            .interact();
        match re {
            Ok(password) => {
                return password;
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                panic!();
            }
        }
    }

    fn password_with_confirmation(prompt: &str) -> String {
        println!("{}", prompt.bright_green());

        let re = dialoguer::Password::new()
            .with_prompt("New Password")
            .with_confirmation("Confirm password", "Passwords mismatching")
            .interact();
        match re {
            Ok(password) => {
                return password;
            }
            Err(_e) => {
                eprintln!("{}", _e.red());
                panic!();
            }
        }
    }
}

#[cfg(test)]
mod test_dialog {

    use super::*;

    #[test]
    fn test_get_string() {
        let a = DialogGeter::get_string("请输入一个字符串");
        println!("最终获得的 string 是: {}", a);
    }

    #[test]
    fn test_repl_get_number() {
        let a = DialogGeter::get_number("你要吃几个汉堡?");
        println!("最终获得的数字是: {}", a);
    }

    #[test]
    fn test_repl_get_multiple_string() {
        let arr = DialogGeter::get_string_multiple("请输入多个字符串");
        println!("{:?}", arr);
    }

    #[test]
    fn test_repl_req_bool() {
        let b = DialogGeter::get_bool("test_repl_req_bool");
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_repl_req_string() {
        let b = DialogGeter::get_string("请输入一个字符串");
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_get_single_selected() {
        let items = vec!["foo", "bar", "baz"];

        let b = DialogGeter::get_single_selected("prompt", &items);
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_get_multiple_selected() {
        let items = vec!["foo", "bar", "baz"];
        let b = DialogGeter::get_multiple_selected("prompt", &items);
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_edit() {
        let b = DialogGeter::editor("prompt");
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_password() {
        let b = DialogGeter::password("");
        println!("最终获得的数字是: {:?}", b);
    }

    #[test]
    fn test_password_with_confirmation() {
        let b = DialogGeter::password_with_confirmation("password_with_confirmation");
        println!("最终获得的数字是: {:?}", b);
    }
}
