use super::arg_type;
use crate::helper::*;
use owo_colors::OwoColorize;
use std::{num::ParseIntError, path::Path, rc::Rc};

// pub type Number = i128;
// pub type PathBuf = path::PathBuf;

pub type ParseResultMessage = String;
pub type ParseResult<T> = Result<T, ParseResultMessage>;

#[derive(Clone)]
pub enum ArgAction {
    Empty(Rc<dyn Fn() -> ()>),

    /// String
    String(Rc<dyn Fn(String) -> ()>),

    /// Vec<String>
    StringMutiple(Rc<dyn Fn(arg_type::StringMutiple) -> ()>),

    /// isize
    Number(Rc<dyn Fn(arg_type::Number) -> ()>),

    /// Vec<isize>
    NumberMutiple(Rc<dyn Fn(arg_type::NumberMutiple) -> ()>),

    /// Path
    Path(Rc<dyn Fn(Rc<arg_type::Path>) -> ()>),

    /// Vec<Path>
    PathMutiple(Rc<dyn Fn(Rc<arg_type::PathMutiple>) -> ()>),

    /// bool
    Bool(Rc<dyn Fn(arg_type::Bool) -> ()>),

    /// Vec<bool>
    BoolMutiple(Rc<dyn Fn(arg_type::BoolMutiple) -> ()>),

    // Repl(Rc<dyn Fn(Option<String>) -> ()>),
    Dialog(Rc<dyn Fn(arg_type::Dialog) -> ()>),
}

impl std::fmt::Display for ArgAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgAction::Empty(_) => write!(f, "{}", "ArgType::Empty"),
            ArgAction::String(_) => write!(f, "{}", "ArgType::String"),
            ArgAction::StringMutiple(_) => write!(f, "{}", "ArgType::VecString"),
            ArgAction::Number(_) => write!(f, "{}", "ArgType::Number"),
            ArgAction::NumberMutiple(_) => write!(f, "{}", "ArgType::VecNumber"),
            ArgAction::Path(_) => write!(f, "{}", "ArgType::Path"),
            ArgAction::PathMutiple(_) => write!(f, "{}", "ArgType::VecPath"),
            ArgAction::Bool(_) => write!(f, "{}", "ArgType::Bool"),
            ArgAction::BoolMutiple(_) => write!(f, "{}", "ArgType::VecBool"),
            ArgAction::Dialog(_) => write!(f, "{}", "ArgType::Repl"),
        }
    }
}

impl ArgAction {
    /// 当音帮助文档时的 arguments 参数说明.
    pub fn arg_message(&self) -> String {
        let arg_tips = match self {
            ArgAction::Empty(_) => "".to_string(),
            ArgAction::String(_) => format!(
                r#"{s} -- 需要 1 个 {g}, 示例: {z}"#,
                s = r#""string""#.styled_arg_type(),
                g = r#""string""#.styled_arg_type(),
                z = r#""input an string""#.styled_arg(),
            ),
            ArgAction::StringMutiple(_) => format!(
                r#"{s} -- 需要 多个 {z}, 示例: {example}"#,
                s = r#"["string"...]"#.styled_arg_type(),
                z = r#""string""#.styled_arg(),
                example = r#""input an string" "string 2" "string 3" "#.styled_arg(),
            ),
            ArgAction::Number(_) => format!(
                r#"{s} -- 需要 1 个 Number, 示例: {z}"#,
                s = r#"Number"#.styled_arg_type(),
                z = r#"100"#.styled_arg(),
            ),
            ArgAction::NumberMutiple(_) => {
                format!(
                    r#"{s} -- 需要 多个 Number, 每个 Number 用 [空格] 分开, 示例: {z}"#,
                    s = r#"[Number...]"#.styled_arg_type(),
                    z = r#"0 1 2 5 123 100"#.styled_arg(),
                )
            }
            ArgAction::Path(_) => {
                return format!(
                    r#"{s} -- 需要 1 个 {s}, 示例: {z}"#,
                    s = r#"Path"#.styled_arg_type(),
                    z = r#""./folder/hello.txt""#.styled_arg(),
                );
            }
            ArgAction::PathMutiple(_) => {
                format!(
                    r#"{s} -- 需要 多个 "Path",  每个 Path 用 [空格] 分开, 示例: {z}"#,
                    s = r#"[Path...]"#.styled_arg_type(),
                    z = r#"0 1 2 5 123 100"#.styled_arg(),
                )
            }
            ArgAction::Bool(_) => format!(
                r#"{s} -- 需要 1 个 {s} 类型的值, {t} 或者 {f}, 示例: {z}"#,
                s = r#"bool"#.styled_arg_type(),
                t = r#"true"#.styled_arg(),
                f = r#"true"#.styled_arg(),
                z = r#"true"#.styled_arg(),
            ),

            ArgAction::BoolMutiple(_) => {
                format!(
                    r#"{s} -- 需要 多个 {b} 类型的值, {t} 或者 {f}, 每个 {b} 用 [空格] 分开, 示例: {z}"#,
                    s = r#"[Bool...]"#.styled_arg_type(),
                    b = r#"bool"#.styled_arg_type(),
                    t = r#"true"#.styled_arg(),
                    f = r#"true"#.styled_arg(),
                    z = r#"true false true false"#.styled_arg(),
                )
            }
            ArgAction::Dialog(_) => "".to_string(),
        };

        return format!("    {}", arg_tips);
    }

    /// 遇到参数类型错误时告诉用户如何输入正确的参数.
    pub fn arg_type_tips(&self) -> String {
        let sdafdsaf: &str = match self {
            ArgAction::Empty(_) => "",
            ArgAction::String(_) => r#"参数类型: string, 示例: "string" -- 需要 1 个 "string""#,
            ArgAction::StringMutiple(_) => {
                r#"参数类型: ["string"...]], 示例: "str 1" "string 2" string_three "#
            }
            ArgAction::Number(_) => r#"参数类型: number, 示例: 999"#,
            ArgAction::NumberMutiple(_) => {
                r#"参数类型: [number...], 示例: 1 2 3 100 555 -- 需要 多个 Number, 每个 Number 用 [空格] 分开."#
            }
            ArgAction::Path(_) => r#"参数类型: path, 示例: /path/to/file.txt"#,
            ArgAction::PathMutiple(_) => {
                r#"参数类型: [path...], 示例: "/path/to/folder/" "./path/to/file.txt" "filename.txt""#
            }
            ArgAction::Bool(_) => r#"参数类型: bool, 示例: "true""#,
            ArgAction::BoolMutiple(_) => r#"参数类型: [bool...], 示例: true false true false"#,
            ArgAction::Dialog(_) => "",
        };

        return sdafdsaf.to_string();
    }

    pub fn value_example(&self) -> String {
        let re = match self {
            ArgAction::Empty(_) => "",
            ArgAction::String(_) => r#""thid is an string example.""#,
            ArgAction::StringMutiple(_) => r#""str 1" "str 2" "str 3""#,
            ArgAction::Number(_) => r#"9"#,
            ArgAction::NumberMutiple(_) => r#"5 9 100 12"#,
            ArgAction::Path(_) => r#""./path/to/folder/or/file.txt""#,
            ArgAction::PathMutiple(_) => r#""./path 1" "/path/2/" "./" "path3.txt""#,
            ArgAction::Bool(_) => r#"true"#,
            ArgAction::BoolMutiple(_) => r#"true false"#,
            ArgAction::Dialog(_) => "",
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

    pub fn get_empty(self) -> ParseResult<arg_type::Empty> {
        if self.subcommand_args.is_empty() {
            return Ok(arg_type::Empty::new());
        }
        return Err(format!(
            "参数数量不正确: 此子命令不需要参数, 实际接收到了 {} 个参数: {:?}",
            self.subcommand_args.len().styled_sub_command(),
            self.subcommand_args,
        ));
    }

    pub fn get_string(self) -> ParseResult<String> {
        let s = self.subcommand_args;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                return ParseResult::Ok(str.clone());
            }
        }
        return Err(format!(
            "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
            s.len().styled_sub_command(),
            format!("{:?}", s).styled_arg(),
        ));
    }

    pub fn get_vec_string(self) -> ParseResult<Vec<String>> {
        return Ok(self.subcommand_args);
    }

    pub fn get_number(self) -> ParseResult<arg_type::Number> {
        let s = self.subcommand_args;
        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re: Result<arg_type::Number, core::num::ParseIntError> = str.parse();
                match re {
                    Ok(n) => return Ok(n),
                    Err(_e) => {
                        return Err(format!("{:?}", _e));
                    }
                }
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
                s.len().styled_sub_command(),
                // s.green(),
                format!("{:?}", s).styled_arg(),
            ));
        }
        return Err("()".to_string());
    }

    pub fn get_vec_number(self) -> ParseResult<arg_type::NumberMutiple> {
        let s = self.subcommand_args;

        let mut re: Vec<arg_type::Number> = vec![];

        for x in s {
            let u: Result<arg_type::Number, ParseIntError> = x.parse();
            match u {
                Ok(n) => {
                    re.push(n);
                }
                Err(_err) => {
                    return Err(format!("{}", _err));
                }
            }
        }

        return Ok(re);
    }

    pub fn get_path(self) -> ParseResult<arg_type::Path> {
        let s = self.subcommand_args;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re = Path::new(str).to_owned();

                return Ok(re);
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
                s.len().styled_sub_command(),
                // s.green(),
                format!("{:?}", s).styled_arg(),
            ));
        }
        return Err("()".to_string());
    }

    pub fn get_vec_path(self) -> ParseResult<arg_type::PathMutiple> {
        let s = self.subcommand_args;

        let mut re: arg_type::PathMutiple = vec![];

        for x in s {
            let path_buf = Path::new(&x).to_owned();
            re.push(path_buf);
        }

        return Ok(re);
    }

    pub fn get_bool(self) -> ParseResult<bool> {
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
                    "bool".styled_arg_type(),
                    "bool".styled_arg_type(),
                    true.styled_arg(),
                    false.styled_arg(),
                    lovwercase.styled_arg(),
                ));
            } else {
                return Err(format!(
                    "参数不正确: 参数的类型是 {}, {} 类型的值可以是: {}, {}",
                    "bool".styled_arg_type(),
                    "bool".styled_arg_type(),
                    true.styled_arg(),
                    false.styled_arg(),
                ));
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个 {} 类型的参数, 实际接收到了 {} 个参数: {:?}",
                "bool".styled_arg_type(),
                s.len().styled_sub_command(),
                // s.styled_arg(),
                format!("{:?}", s).styled_arg(),
            ));
        }
    }

    pub fn get_vec_bool(self) -> ParseResult<Vec<bool>> {
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
            s.len().styled_sub_command(),
            // s.styled_arg(),
            format!("{:?}", s).styled_arg(),
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
