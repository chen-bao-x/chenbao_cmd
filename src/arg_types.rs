use std::{
    num::ParseIntError,
    path::{self, Path},
    rc::Rc,
};

use owo_colors::OwoColorize;

pub type Number = isize;

pub type PathBuf = path::PathBuf;

pub type ParseResultMessage = String;
pub type ParseResult<T> = Result<T, ParseResultMessage>;

/// 用来表示这个 subcommand 不需要参数.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone)]
pub enum ArgType {
    Empty(Rc<dyn Fn() -> ()>),

    /// String
    String(Rc<dyn Fn(String) -> ()>),
    /// Vec<String>
    VecString(Rc<dyn Fn(Vec<String>) -> ()>),
    /// isize
    Number(Rc<dyn Fn(Number) -> ()>),
    /// Vec<isize>
    VecNumber(Rc<dyn Fn(Vec<Number>) -> ()>),
    /// Path
    Path(Rc<dyn Fn(Rc<PathBuf>) -> ()>),
    /// Vec<Path>
    VecPath(Rc<dyn Fn(Rc<Vec<PathBuf>>) -> ()>),
    /// bool
    Bool(Rc<dyn Fn(bool) -> ()>),
    /// Vec<bool>
    VecBool(Rc<dyn Fn(Vec<bool>) -> ()>),

    Repl(Rc<dyn Fn(Option<String>) -> ()>),
}

impl std::fmt::Display for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgType::Empty(_) => write!(f, "{}", "ArgType::Empty"),
            ArgType::String(_) => write!(f, "{}", "ArgType::String"),
            ArgType::VecString(_) => write!(f, "{}", "ArgType::VecString"),
            ArgType::Number(_) => write!(f, "{}", "ArgType::Number"),
            ArgType::VecNumber(_) => write!(f, "{}", "ArgType::VecNumber"),
            ArgType::Path(_) => write!(f, "{}", "ArgType::Path"),
            ArgType::VecPath(_) => write!(f, "{}", "ArgType::VecPath"),
            ArgType::Bool(_) => write!(f, "{}", "ArgType::Bool"),
            ArgType::VecBool(_) => write!(f, "{}", "ArgType::VecBool"),
            ArgType::Repl(_) => write!(f, "{}", "ArgType::Repl"),
        }
    }
}

impl ArgType {
    pub fn arg_message(&self) -> String {
        let arg_tips = match self {
            ArgType::Empty(_) => "".to_string(),
            ArgType::String(_) => format!(
                r#"{s} -- 需要 1 个 {z}"#,
                s = r#""string""#.magenta(),
                z = r#""字符串""#.green(),
            ),
            ArgType::VecString(_) => format!(
                r#"{s} -- 需要 多个 {z}"#,
                s = r#"["string"...]"#.magenta(),
                z = r#""字符串""#.green(),
            ),
            ArgType::Number(_) => format!(
                r#"{s} -- 需要 1 个 Number, 示例: {z}"#,
                s = r#"Number"#.magenta(),
                z = r#"100"#.green(),
            ),
            ArgType::VecNumber(_) => {
                format!(
                    r#"{s} -- 需要 多个 Number, 每个 Number 用 [空格] 分开, 示例: {z}"#,
                    s = r#"[Number...]"#.magenta(),
                    z = r#"0 1 2 5 123 100"#.green(),
                )
            }
            ArgType::Path(_) => format!(r#"Path -- 需要 1 个 "Path""#),
            ArgType::VecPath(_) => {
                format!(
                    r#"{s} -- 需要 多个 "Path",  每个 Path 用 [空格] 分开, 示例: {z}"#,
                    s = r#"[Path...]"#.magenta(),
                    z = r#"0 1 2 5 123 100"#.green(),
                )
            }
            ArgType::Bool(_) => format!(
                r#"{s} -- 需要 1 个 Bool 类型的值, {t} 或者 {f}."#,
                s = r#"bool"#.magenta(),
                t = r#"true"#.green(),
                f = r#"true"#.green(),
            ),
            ArgType::Repl(_) => "".to_string(),
            ArgType::VecBool(_) => {
                format!(
                    r#"{s} -- 需要 多个 {b} 类型的值, {t} 或者 {f}, 每个 {b} 用 [空格] 分开."#,
                    s = r#"[Bool...]"#.magenta(),
                    b = r#"bool"#.magenta(),
                    t = r#"true"#.green(),
                    f = r#"true"#.green(),
                )
            }
        };

        return format!("    {}", arg_tips);
    }

    pub fn arg_type_tips(&self) -> String {
        let sdafdsaf: &str = match self {
            ArgType::Empty(_) => "",
            ArgType::String(_) => r#"参数类型: string, 示例: "string" -- 需要 1 个 "string""#,
            ArgType::VecString(_) => {
                r#"参数类型: ["string"...]], 示例: "str 1" "string 2" string_three "#
            }
            ArgType::Number(_) => r#"参数类型: number, 示例: 999"#,
            ArgType::VecNumber(_) => {
                r#"参数类型: [number...], 示例: 1 2 3 100 555 -- 需要 多个 Number, 每个 Number 用 [空格] 分开."#
            }
            ArgType::Path(_) => r#"参数类型: path, 示例: /path/to/file.txt"#,
            ArgType::VecPath(_) => {
                r#"参数类型: [path...], 示例: "/path/to/folder/" "./path/to/file.txt" "filename.txt""#
            }
            ArgType::Bool(_) => r#"参数类型: bool, 示例: "true""#,
            ArgType::VecBool(_) => r#"参数类型: [bool...], 示例: true false true false"#,
            ArgType::Repl(_) => "",
        };

        return sdafdsaf.to_string();
    }

    pub fn value_example(&self) -> String {
        let re = match self {
            ArgType::Empty(_) => "",
            ArgType::String(_) => r#""thid is an string example.""#,
            ArgType::VecString(_) => r#""str 1" "str 2" "str 3"#,
            ArgType::Number(_) => r#"9"#,
            ArgType::VecNumber(_) => r#"5 9 100 12"#,
            ArgType::Path(_) => r#""./path/to/folder/or/file.txt""#,
            ArgType::VecPath(_) => r#""./path 1" "/path/2/" "./" "path3.txt""#,
            ArgType::Bool(_) => r#"true"#,
            ArgType::VecBool(_) => r#"true false"#,
            ArgType::Repl(_) => "",
        };

        return re.to_string();
    }
}

/// 子命令实际接收到的参数
#[derive(Clone)]
pub struct SubcommandArgsValue {
    /// 子命令的 参数
    pub subcommand_args: Vec<String>,
    // pub need_arg_type: ArgType,
}

impl SubcommandArgsValue {
    pub fn new(
        // need_arg_type: ArgType,
        value: Vec<String>,
    ) -> Self {
        Self {
            subcommand_args: value,
            // need_arg_type,
        }
    }

    // pub fn arg_check(&self, geting_argtype: ArgType) -> Result<(), String> {
    //     return Ok(());

    // // 检查需要的参数类型和实际获取的参数类型是否一致.
    // if self.need_arg_type == geting_argtype {
    //     return Ok(());
    // }

    // return Err(format!(
    //     "你告诉这个命令行程序的用户这个子命令需要 {} 类型的参数, 而你却在获取 {} 类型的参数",
    //     self.need_arg_type, geting_argtype
    // ));
    // }

    pub fn get_empty(self) -> ParseResult<Empty> {
        // if let Err(e) = self.arg_check(ArgType::Empty) {
        //     return Err(e);
        // }

        return Ok(Empty::new());
    }

    pub fn get_string(self) -> ParseResult<String> {
        // if let Err(e) = self.arg_check(ArgType::String) {
        //     return Err(e.clone());
        // }

        let s = self.subcommand_args;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                return ParseResult::Ok(str.clone());
            }
        }
        return Err(format!(
            "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
            s.len().cyan(),
            s.green(),
        ));
    }

    pub fn get_vec_string(self) -> ParseResult<Vec<String>> {
        // if let Err(e) = self.arg_check(ArgType::VecString) {
        //     return Err(e.clone());
        // }
        return Ok(self.subcommand_args);
    }

    pub fn get_number(self) -> ParseResult<Number> {
        // if let Err(e) = self.arg_check(ArgType::Number) {
        //     return Err(e.clone());
        // }

        let s = self.subcommand_args;
        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re: isize = str.parse().unwrap();
                return Ok(re);
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
                s.len().cyan(),
                s.green(),
            ));
        }
        return Err("()".to_string());
    }

    pub fn get_vec_number(self) -> ParseResult<Vec<Number>> {
        // if let Err(e) = self.arg_check(ArgType::VecNumber) {
        //     return Err(e.clone());
        // }
        let s = self.subcommand_args;

        let mut re: Vec<Number> = vec![];

        for x in s {
            let u: Result<Number, ParseIntError> = x.parse();
            match u {
                Ok(n) => {
                    re.push(n);
                }
                Err(_err) => {
                    // todo!(); // 将 Optinal 类型修改为 Result 类型.
                    return Err(format!("{}", _err));
                }
            }
        }

        return Ok(re);
    }

    pub fn get_path(self) -> ParseResult<PathBuf> {
        // if let Err(e) = self.arg_check(ArgType::Path) {
        //     return Err(e.clone());
        // }
        let s = self.subcommand_args;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re = Path::new(str).to_owned();

                return Ok(re);
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
                s.len().cyan(),
                s.green(),
            ));
        }
        return Err("()".to_string());
    }

    pub fn get_vec_path(self) -> ParseResult<Vec<PathBuf>> {
        // if let Err(e) = self.arg_check(ArgType::VecPath) {
        //     return Err(e.clone());
        // }
        let s = self.subcommand_args;

        let mut re: Vec<PathBuf> = vec![];

        for x in s {
            let path_buf = Path::new(&x).to_owned();
            re.push(path_buf);
        }

        return Ok(re);
    }

    pub fn get_bool(self) -> ParseResult<bool> {
        // if let Err(e) = self.arg_check(ArgType::Bool) {
        //     return Err(e.clone());
        // }
        let s = self.subcommand_args;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                let lovwercase = str.to_lowercase();
                {
                    if lovwercase == "true" {
                        return Ok(true);
                    }
                }

                {
                    if lovwercase == "false" {
                        return Ok(false);
                    }
                }

                return Err(format!(
                    "参数不正确: 参数的类型是 {}, {} 类型的值可以是: {}, {}, 实际接收到的是: {}",
                    "bool".magenta(),
                    "bool".magenta(),
                    true.green(),
                    false.green(),
                    lovwercase.green(),
                ));
            } else {
                return Err(format!(
                    "参数不正确: 参数的类型是 bool, bool 类型的值可以是: {}, {}",
                    true.green(),
                    false.green(),
                ));
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个 bool 类型的参数, 实际接收到了 {} 个参数: {:?}",
                s.len().cyan(),
                s.green(),
            ));
        }
    }

    pub fn get_vec_bool(self) -> ParseResult<Vec<bool>> {
        // if let Err(e) = self.arg_check(ArgType::VecBool) {
        //     return Err(e.clone());
        // }
        let s = self.subcommand_args;
        let mut re: Vec<bool> = vec![];

        for x in s {
            let lovwercase = x.to_lowercase();
            {
                if lovwercase == "true" {
                    re.push(true);
                    continue;
                }
            }

            {
                if lovwercase == "false" {
                    re.push(false);
                    continue;
                }
            }

            return Err(format!(
                "参数不正确: 参数的类型是 bool.\nbool 类型的值可以是: true , false ",
            ));
        }

        return Ok(re);
    }

    pub fn get_repl(self) -> ParseResult<Option<String>> {
        let s = self.subcommand_args;

        if s.len() == 0 {
            return Ok(None);
        }

        if s.len() == 1 {
            if let Some(str) = s.first() {
                return Ok(Some(str.clone()));
            }
        }

        return Err(format!(
            "参数数量不正确: 需要 0 个 或者 1 个 参数, 实际接收到了 {} 个参数: {:?}",
            s.len().cyan(),
            s.green(),
        ));
    }
}

#[cfg(test)]
mod arg_check {
    use super::*;

    #[test]
    fn ok_case_bool() {
        {
            // let v = SubcommandArgsValue::new(ArgType::Bool, vec!["false".to_string()]);
            let v = SubcommandArgsValue::new(vec!["false".to_string()]);
            let re = v.get_bool();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }

    #[test]
    fn ok_case_empty() {
        {
            // let v = SubcommandArgsValue::new(ArgType::Empty, vec![]);
            let v = SubcommandArgsValue::new(vec![]);
            let re = v.get_empty();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }

    #[test]
    fn ok_case_number() {
        {
            let v = SubcommandArgsValue::new(vec!["2314324".to_string()]);
            let re = v.get_number();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }

    #[test]
    fn ok_case_string() {
        {
            let v = SubcommandArgsValue::new(vec!["2314324".to_string()]);
            let re = v.get_string();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }
    #[test]
    fn ok_case_path() {
        {
            let v = SubcommandArgsValue::new(vec!["./path".to_string()]);
            let re = v.get_path();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }
    #[test]
    fn ok_case_vec_path() {
        {
            let v = SubcommandArgsValue::new(vec!["./path".to_string(), "asdf.txt".to_string()]);
            let re = v.get_vec_path();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }
    #[test]
    fn ok_case_vec_bool() {
        {
            let v = SubcommandArgsValue::new(vec!["false".to_string(), "true".to_string()]);
            let re = v.get_vec_bool();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }
    #[test]
    fn ok_case_vec_number() {
        {
            let v = SubcommandArgsValue::new(vec!["234532".to_string(), "5436".to_string()]);
            let re = v.get_vec_number();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }

    #[test]
    fn ok_case_vec_string() {
        {
            let v = SubcommandArgsValue::new(vec!["234532".to_string(), "5436".to_string()]);
            let re = v.get_vec_string();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }

    #[test]
    fn err_case_empty() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_string() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_path() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_number() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_bool() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_number() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_path() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_string() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }
}
