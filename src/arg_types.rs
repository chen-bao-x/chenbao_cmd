use std::{
    num::ParseIntError,
    path::{self, PathBuf},
};

pub type Number = usize;

pub type Path = path::Path;

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArgType {
    Empty,
    String,
    VecString,
    Number,
    VecNumber,
    Path,
    VecPath,
    Bool,
    VecBool,
}

impl std::fmt::Display for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgType::Empty => write!(f, "{}", "ArgType::Empty"),
            ArgType::String => write!(f, "{}", "ArgType::String"),
            ArgType::VecString => write!(f, "{}", "ArgType::VecString"),
            ArgType::Number => write!(f, "{}", "ArgType::Number"),
            ArgType::VecNumber => write!(f, "{}", "ArgType::VecNumber"),
            ArgType::Path => write!(f, "{}", "ArgType::Path"),
            ArgType::VecPath => write!(f, "{}", "ArgType::VecPath"),
            ArgType::Bool => write!(f, "{}", "ArgType::Bool"),
            ArgType::VecBool => write!(f, "{}", "ArgType::VecBool"),
        }
    }
}

impl ArgType {
    pub fn arg_message(&self) -> String {
        let sdafdsaf: &str = match self {
            ArgType::Empty => "",
            ArgType::String => r#""string" -- 需要 1 个 "字符串""#,
            ArgType::VecString => r#"["string"...] -- 需要 多个 "字符串""#,
            ArgType::Number => r#"Number -- 需要 1 个 Number, 示例: 0 1 2 5 123 100"#,
            ArgType::VecNumber => r#"[Number...] -- 需要 多个 Number, 每个 Number 用 [空格] 分开."#,
            ArgType::Path => r#"Path -- 需要 1 个 "Path""#,
            ArgType::VecPath => r#"[Path...] -- 需要 多个 "Path",  每个 Path 用 [空格] 分开."#,
            ArgType::Bool => r#"Bool -- 需要 1 个 Bool, true 或者 false"#,
            ArgType::VecBool => {
                r#"[Bool...] -- 需要 多个 Bool, true 或者 false, 每个 bool 用 [空格] 分开."#
            }
        };

        return sdafdsaf.to_string();
    }
}

/// 子命令实际接收到的参数
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubcommandArgsValue {
    pub value: Vec<String>,
    pub need_arg_type: ArgType,
}

impl SubcommandArgsValue {
    pub fn new(need_arg_type: ArgType, value: Vec<String>) -> Self {
        Self {
            value,
            need_arg_type,
        }
    }

    fn arg_check(&self, geting_argtype: ArgType) -> Result<(), String> {
        // 检查需要的参数类型和实际获取的参数类型是否一致.
        if self.need_arg_type == geting_argtype {
            return Ok(());
        }

        return Err(format!(
            "你告诉这个命令行程序的用户这个子命令需要 {} 类型的参数, 而你却在获取 {} 类型的参数",
            self.need_arg_type, geting_argtype
        ));
    }

    pub fn get_empty(self) -> ParseResult<Empty> {
        if let Err(e) = self.arg_check(ArgType::Empty) {
            return Err(e);
        }

        return Ok(Empty::new());
    }

    pub fn get_string(self) -> ParseResult<String> {
        if let Err(e) = self.arg_check(ArgType::String) {
            return Err(e.clone());
        }

        let s = self.value;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                return ParseResult::Ok(str.clone());
            }
        }
        return Err(format!(
            "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
            s.len(),
            s,
        ));
    }

    pub fn get_vec_string(self) -> ParseResult<Vec<String>> {
        if let Err(e) = self.arg_check(ArgType::VecString) {
            return Err(e.clone());
        }
        return Ok(self.value);
    }

    pub fn get_number(self) -> ParseResult<Number> {
        if let Err(e) = self.arg_check(ArgType::Number) {
            return Err(e.clone());
        }

        let s = self.value;
        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re: usize = str.parse().unwrap();
                return Ok(re);
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
                s.len(),
                s,
            ));
        }
        return Err("()".to_string());
    }

    pub fn get_vec_number(self) -> ParseResult<Vec<Number>> {
        if let Err(e) = self.arg_check(ArgType::VecNumber) {
            return Err(e.clone());
        }
        let s = self.value;

        let mut re: Vec<Number> = vec![];

        for x in s {
            let u: Result<usize, ParseIntError> = x.parse();
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
        if let Err(e) = self.arg_check(ArgType::Path) {
            return Err(e.clone());
        }
        let s = self.value;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re = Path::new(str).to_owned();

                return Ok(re);
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
                s.len(),
                s,
            ));
        }
        return Err("()".to_string());
    }

    pub fn get_vec_path(self) -> ParseResult<Vec<PathBuf>> {
        if let Err(e) = self.arg_check(ArgType::VecPath) {
            return Err(e.clone());
        }
        let s = self.value;

        let mut re: Vec<PathBuf> = vec![];

        for x in s {
            let u = Path::new(&x).to_owned();
            re.push(u);
        }

        return Ok(re);
    }

    pub fn get_bool(self) -> ParseResult<bool> {
        if let Err(e) = self.arg_check(ArgType::Bool) {
            return Err(e.clone());
        }
        let s = self.value;

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
                    "参数不正确: 参数的类型是 bool, bool 类型的值可以是: true  false ",
                ));
            } else {
                return Err(format!(
                    "参数不正确: 参数的类型是 bool, bool 类型的值可以是: true  false ",
                ));
            }
        } else {
            return Err(format!(
                "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
                s.len(),
                s,
            ));
        }
    }

    pub fn get_vec_bool(self) -> ParseResult<Vec<bool>> {
        if let Err(e) = self.arg_check(ArgType::VecBool) {
            return Err(e.clone());
        }
        let s = self.value;
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
}

#[cfg(test)]
mod arg_check {
    use super::*;

    #[test]
    fn ok_case_bool() {
        {
            let v = SubcommandArgsValue::new(ArgType::Bool, vec!["false".to_string()]);
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
            let v = SubcommandArgsValue::new(ArgType::Empty, vec![]);
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
            let v = SubcommandArgsValue::new(ArgType::Number, vec!["2314324".to_string()]);
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
            let v = SubcommandArgsValue::new(ArgType::String, vec!["2314324".to_string()]);
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
            let v = SubcommandArgsValue::new(ArgType::Path, vec!["./path".to_string()]);
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
            let v = SubcommandArgsValue::new(
                ArgType::VecPath,
                vec!["./path".to_string(), "asdf.txt".to_string()],
            );
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
            let v = SubcommandArgsValue::new(
                ArgType::VecBool,
                vec!["false".to_string(), "true".to_string()],
            );
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
            let v = SubcommandArgsValue::new(
                ArgType::VecNumber,
                vec!["234532".to_string(), "5436".to_string()],
            );
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
            let v = SubcommandArgsValue::new(
                ArgType::VecString,
                vec!["234532".to_string(), "5436".to_string()],
            );
            let re = v.get_vec_string();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }

    #[test]
    fn err_case_empty() {
        let v = SubcommandArgsValue::new(ArgType::Empty, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_string() {
        let v = SubcommandArgsValue::new(ArgType::String, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_path() {
        let v = SubcommandArgsValue::new(ArgType::Path, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_number() {
        let v = SubcommandArgsValue::new(ArgType::Number, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_bool() {
        let v = SubcommandArgsValue::new(ArgType::VecBool, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_number() {
        let v = SubcommandArgsValue::new(ArgType::VecNumber, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_path() {
        let v = SubcommandArgsValue::new(ArgType::VecPath, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_string() {
        let v = SubcommandArgsValue::new(ArgType::VecString, vec!["false".to_string()]);
        let re = v.get_bool();

        // shold be Err, not ok.
        if let Ok(_) = re {
            panic!("");
        }
    }
}
