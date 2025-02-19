use super::arg_type;
use crate::helper::StyledString;
use crate::SharedVecString;
use core::fmt;
use std::{default, num::ParseIntError, path::Path, rc::Rc};

pub type ParseResultMessage = String;
pub type ParseResult<T> = Result<T, ParseResultMessage>;

#[derive(Clone)]
pub enum Arg {
    /// 表示这个子命令不需要参数
    Empty(&'static dyn Fn(arg_type::Empty)),

    String(&'static dyn Fn(arg_type::String)),

    StringMutiple(&'static dyn Fn(Rc<arg_type::StringMutiple>)),

    Number(&'static dyn Fn(arg_type::Number)),

    NumberMutiple(&'static dyn Fn(arg_type::NumberMutiple)),

    Path(&'static dyn Fn(arg_type::Path)),

    PathMutiple(&'static dyn Fn(arg_type::PathMutiple)),

    Bool(&'static dyn Fn(arg_type::Bool)),

    BoolMutiple(&'static dyn Fn(arg_type::BoolMutiple)),

    /// 对话式交互.
    Dialog(&'static dyn Fn(&mut arg_type::Dialog)),
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::Empty(_) => write!(f, "ArgType::Empty"),
            Arg::String(_) => write!(f, "ArgType::String"),
            Arg::StringMutiple(_) => write!(f, "ArgType::VecString"),
            Arg::Number(_) => write!(f, "ArgType::Number"),
            Arg::NumberMutiple(_) => write!(f, "ArgType::VecNumber"),
            Arg::Path(_) => write!(f, "ArgType::Path"),
            Arg::PathMutiple(_) => write!(f, "ArgType::VecPath"),
            Arg::Bool(_) => write!(f, "ArgType::Bool"),
            Arg::BoolMutiple(_) => write!(f, "ArgType::VecBool"),
            Arg::Dialog(_) => write!(f, "ArgType::Repl"),
        }
    }
}

impl default::Default for Arg {
    fn default() -> Self {
        Self::Empty(&|_x| {})
    }
}
impl fmt::Debug for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty(_arg0) => f.debug_tuple("Empty(_)").finish(),
            Self::String(_arg0) => f.debug_tuple("String(_)").finish(),
            Self::StringMutiple(_arg0) => f.debug_tuple("StringMutiple(_)").finish(),
            Self::Number(_arg0) => f.debug_tuple("Number(_)").finish(),
            Self::NumberMutiple(_arg0) => f.debug_tuple("NumberMutiple(_)").finish(),
            Self::Path(_arg0) => f.debug_tuple("Path(_)").finish(),
            Self::PathMutiple(_arg0) => f.debug_tuple("PathMutiple(_)").finish(),
            Self::Bool(_arg0) => f.debug_tuple("Bool(_)").finish(),
            Self::BoolMutiple(_arg0) => f.debug_tuple("BoolMutiple(_)").finish(),
            Self::Dialog(_arg0) => f.debug_tuple("Dialog(_)").finish(),
        }
    }
}

impl Arg {
    /// 当音帮助文档时的 arguments 参数说明.
    pub(crate) fn arg_message(&self) -> String {
        let arg_tips = match self {
            Arg::Empty(_) => "".to_string(),
            Arg::String(_) => format!(
                r#"{s} -- 需要 1 个 {s}, 示例: {z}"#,
                s = r#"string"#.styled_arg_type(),
                z = r#""input an string""#.styled_arg(),
            ),
            Arg::StringMutiple(_) => format!(
                r#"{s}... -- 需要 多个 {s}, 示例: {example}"#,
                s = r#"string"#.styled_arg_type(),
                example = r#""input an string" "string 2" "string 3" "#.styled_arg(),
            ),
            Arg::Number(_) => format!(
                r#"{s} -- 需要 1 个 Number, 示例: {z}"#,
                s = r#"Number"#.styled_arg_type(),
                z = r#"100"#.styled_arg(),
            ),
            Arg::NumberMutiple(_) => {
                format!(
                    r#"{s}... -- 需要 多个 {s}, 每个 {s} 用 [空格] 分开, 示例: {z}"#,
                    s = r#"Number"#.styled_arg_type(),
                    z = r#"0 1 2 5 123 100"#.styled_arg(),
                )
            }
            Arg::Path(_) => {
                return format!(
                    r#"{s} -- 需要 1 个 {s}, 示例: {z}"#,
                    s = r#"Path"#.styled_arg_type(),
                    z = r#""./folder/hello.txt""#.styled_arg(),
                );
            }
            Arg::PathMutiple(_) => {
                format!(
                    r#"{s}... -- 需要 多个 {s},  每个 Path 用 [空格] 分开, 示例: {z}"#,
                    s = r#"Path"#.styled_arg_type(),
                    z = r#"0 1 2 5 123 100"#.styled_arg(),
                )
            }
            Arg::Bool(_) => format!(
                r#"{s} -- 需要 1 个 {s} 类型的值, {t} 或者 {f}, 示例: {z}"#,
                s = r#"bool"#.styled_arg_type(),
                t = r#"true"#.styled_arg(),
                f = r#"false"#.styled_arg(),
                z = r#"true"#.styled_arg(),
            ),

            Arg::BoolMutiple(_) => {
                format!(
                    r#"{s}... -- 需要 多个 {s} 类型的值, {t} 或者 {f}, 每个 {s} 用 [空格] 分开, 示例: {z}"#,
                    // s = r#"Bool..."#.styled_arg_type(),
                    s = r#"bool"#.styled_arg_type(),
                    t = r#"true"#.styled_arg(),
                    f = r#"true"#.styled_arg(),
                    z = r#"true false true false"#.styled_arg(),
                )
            }
            Arg::Dialog(_) => "".to_string(),
        };

        format!("    {}", arg_tips)
    }

    // /// 遇到参数类型错误时告诉用户如何输入正确的参数.
    // pub(crate) fn type_error_tips(&self) -> String {
    //     let sdafdsaf: &str = match self {
    //         ArgAction::Empty(_) => "",
    //         ArgAction::String(_) => r#"参数类型: string, 示例: "string" -- 需要 1 个 "string""#,
    //         ArgAction::StringMutiple(_) => {
    //             r#"参数类型: ["string"...]], 示例: "str 1" "string 2" string_three "#
    //         }
    //         ArgAction::Number(_) => r#"参数类型: number, 示例: 999"#,
    //         ArgAction::NumberMutiple(_) => {
    //             r#"参数类型: [number...], 示例: 1 2 3 100 555 -- 需要 多个 Number, 每个 Number 用 [空格] 分开."#
    //         }
    //         ArgAction::Path(_) => r#"参数类型: path, 示例: /path/to/file.txt"#,
    //         ArgAction::PathMutiple(_) => {
    //             r#"参数类型: [path...], 示例: "/path/to/folder/" "./path/to/file.txt" "filename.txt""#
    //         }
    //         ArgAction::Bool(_) => {
    //             format!(
    //                 "参数不正确: 参数的类型是 {btype}, {btype} 类型的值可以是: {t}, {f}, 实际接收到的是: {args:?}",
    //                 btype = "bool".styled_arg_type(),
    //                 t = true.styled_arg(),
    //                 f = false.styled_arg(),
    //                 // args = VecString::vec_to_json(&s).styled_arg(),
    //                 args = self.arg_message(),
    //             );

    //             r#"参数类型: bool, 示例: "true""#
    //         }
    //         ArgAction::BoolMutiple(_) => r#"参数类型: [bool...], 示例: true false true false"#,
    //         ArgAction::Dialog(_) => "",
    //     };

    //     sdafdsaf.to_string()
    // }

    // /// 参数的数量不正确时的报错信息
    // pub(crate) fn cunt_error_tips(&self) {}

    // /// 告诉用户某个类型的参数应该怎么正确填写.
    // pub(crate) fn value_example(&self) -> String {
    //     let re = match self {
    //         ArgAction::Empty(_) => "",
    //         ArgAction::String(_) => r#""thid is an string example.""#,
    //         ArgAction::StringMutiple(_) => r#""str 1" "str 2" "str 3""#,
    //         ArgAction::Number(_) => r#"9"#,
    //         ArgAction::NumberMutiple(_) => r#"5 9 100 12"#,
    //         ArgAction::Path(_) => r#""./path/to/folder/or/file.txt""#,
    //         ArgAction::PathMutiple(_) => r#""./path 1" "/path/2/" "./" "path3.txt""#,
    //         ArgAction::Bool(_) => r#"true"#,
    //         ArgAction::BoolMutiple(_) => r#"true false"#,
    //         ArgAction::Dialog(_) => "",
    //     };
    //
    //     re.to_string()
    // }

    // pub(crate) fn arg_type_display(&self) -> String {
    //     let re = match self {
    //         ArgAction::Empty(_) => "",
    //         ArgAction::String(_) => r#"<String>"#,
    //         ArgAction::StringMutiple(_) => r#"<String>..."#,
    //         ArgAction::Number(_) => r#"<Number>"#,
    //         ArgAction::NumberMutiple(_) => r#"<Number>"#,
    //         ArgAction::Path(_) => r#"<Path>"#,
    //         ArgAction::PathMutiple(_) => r#"<Path>..."#,
    //         ArgAction::Bool(_) => r#"<bool>"#,
    //         ArgAction::BoolMutiple(_) => r#"<bool>..."#,
    //         ArgAction::Dialog(_) => "",
    //     };
    //
    //     re.to_string()
    // }
}

/// 子命令实际接收到的参数
#[derive(Clone)]
pub(crate) struct SubcommandArgsValue {
    /// 子命令的 参数
    pub subcommand_args: SharedVecString,
}

// impl SubcommandArgsValue {
impl SubcommandArgsValue {
    pub fn new(
        // need_arg_type: ArgType,
        value: SharedVecString,
    ) -> Self {
        Self {
            subcommand_args: value,
        }
    }

    pub fn get_empty(self) -> ParseResult<arg_type::Empty> {
        if self.subcommand_args.is_empty() {
            Ok(arg_type::Empty::new())
        } else {
            Err(format!(
                "{}: 此子命令不需要参数, 实际接收到了 {} 个参数: {:?}",
                "参数数量错误".styled_error_marker(),
                self.subcommand_args.len().styled_sub_command(),
                self.subcommand_args,
            ))
        }
    }

    pub fn get_string(self) -> ParseResult<String> {
        let s = self.subcommand_args;

        if s.len() == 1 {
            if let Some(str) = s.first() {
                return ParseResult::Ok(str.clone());
            }
        }
        Err(format!(
            "{}: 需要 1 个参数, 实际接收到了 {} 个参数: {}",
            "参数数量错误".styled_error_marker(),
            s.len().styled_sub_command(),
            format!("{:?}", s).styled_arg(),
        ))
    }

    pub fn get_vec_string(self) -> ParseResult<SharedVecString> {
        Ok(self.subcommand_args)
    }

    pub fn get_number(&self) -> ParseResult<arg_type::Number> {
        let s = &self.subcommand_args;
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
                "{}: 需要 1 个参数, 实际接收到了 {} 个参数: {}",
                "参数数量错误".styled_error_marker(),
                s.len().styled_sub_command(),
                // s.green(),
                format!("{:?}", s).styled_arg(),
            ));
        }
        Err("()".to_string())
    }

    pub fn get_vec_number(self) -> ParseResult<arg_type::NumberMutiple> {
        let s = self.subcommand_args;

        let mut re: Vec<arg_type::Number> = [].to_vec();

        let mut has_err: Option<String> = None;

        s.iter().for_each(|x| {
            let u: Result<arg_type::Number, ParseIntError> = x.parse();
            match u {
                Ok(n) => {
                    re.push(n);
                }
                Err(_err) => {
                    has_err = Some(format!("{}", _err));
                }
            }
        });
        if let Some(err_msg) = has_err {
            Err(err_msg)
        } else {
            Ok(re)
        }
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
                "{}: 需要 1 个参数, 实际接收到了 {} 个参数: {}",
                "参数数量错误".styled_error_marker(),
                s.len().styled_sub_command(),
                // s.green(),
                format!("{:?}", s).styled_arg(),
            ));
        }
        Err("()".to_string())
    }

    pub fn get_vec_path(self) -> ParseResult<arg_type::PathMutiple> {
        let s = self.subcommand_args;

        let mut re: arg_type::PathMutiple = vec![];

        s.iter().for_each(|x| {
            let path_buf = Path::new(&x).to_owned();
            re.push(path_buf);
        });

        Ok(re)
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

                Err(format!(
                    "参数类型错误: 参数的类型是 {btype}, {btype} 类型的值可以是: {t}, {f}, 实际接收到的是: {args}",
                    btype = "bool".styled_arg_type(),
                    t = true.styled_arg(),
                    f = false.styled_arg(),
                    // args = VecString::vec_to_json(&s).styled_arg(),
                    args = format!("{:?}", s).styled_arg(),
                ))
            } else {
                Err(format!(
                    "{}: 需要 1 个 {} 类型的参数, 实际接收到了 {} 个参数: {}",
                    "参数数量错误".styled_error_marker(),
                    "bool".styled_arg_type(),
                    s.len().styled_sub_command(),
                    // s.styled_arg(),
                    format!("{:?}", s).styled_arg(),
                ))
            }
        } else {
            Err(format!(
                "{}: 需要 1 个 {} 类型的参数, 实际接收到了 {} 个参数: {}",
                "参数数量错误".styled_error_marker(),
                "bool".styled_arg_type(),
                s.len().styled_sub_command(),
                // s.styled_arg(),
                format!("{:?}", s).styled_arg(),
            ))
        }
    }

    pub fn get_vec_bool(self) -> ParseResult<Vec<bool>> {
        let s = self.subcommand_args;
        let mut re: Vec<bool> = vec![];
        let mut err_massage: Option<String> = None;

        s.iter().for_each(|x| {
            let lovwercase = x.to_lowercase();

            if lovwercase == "true" {
                re.push(true);
            } else if lovwercase == "false" {
                re.push(false);
            } else {



                err_massage = Some(format!(
                    "{}: 参数的类型是 {btype}, {btype} 类型的值可以是: {t}, {f}, 实际接收到的是: {args}",
                    "参数类型错误".styled_error_marker(),
                    btype = "bool".styled_arg_type(),
                    t = true.styled_arg(),
                    f = false.styled_arg(),
                    // args = VecString::vec_to_json(&s).styled_arg(),
                    args = format!("{:?}", s).styled_arg(),
                ))

            }
        });

        if let Some(msg) = err_massage {
            Err(msg)
        } else {
            Ok(re)
        }
    }

    // pub fn get_repl(self) -> ParseResult<Option<String>> {
    pub fn get_repl(self) -> ParseResult<Option<String>> {
        let subcmd_args = self.subcommand_args; // 子命令的参数.

        if subcmd_args.is_empty() {
            return Ok(None);
        }

        if subcmd_args.len() == 1 {
            if let Some(repl_first_arg) = subcmd_args.first() {
                if repl_first_arg == "stdin" {
                    let re = get_string_from();
                    match re {
                        Ok(json_string) => return Ok(Some(json_string)),
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
            }
        }

        Err(format!(
            "{}: 需要 0 个参数 或者 自动生成的'快捷参数', 实际接收到了 {} 个参数: {:?}",
            "参数数量错误".styled_error_marker(),
            subcmd_args.len().styled_sub_command(),
            subcmd_args,
        ))
    }
}

fn get_string_from() -> ParseResult<String> {
    use std::io::Read;

    let mut buffer = String::new();

    // get string from stdin
    let a = std::io::stdin().read_to_string(&mut buffer);
    match a {
        Ok(_) => Ok(buffer),
        Err(_e) => Err(format!("{}", _e)),
    }
}

#[cfg(test)]
mod arg_check {
    use super::*;

    #[test]
    fn ok_case_bool() {
        {
            let v = SubcommandArgsValue::new(vec!["false".to_string()].into());
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
            let v = SubcommandArgsValue::new([].to_vec().into());
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
            let a = ["2314324".to_string()];
            let v = SubcommandArgsValue::new(a.to_vec().into());
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
            let v = SubcommandArgsValue::new(vec!["2314324".to_string()].into());
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
            let v = SubcommandArgsValue::new(vec!["./path".to_string()].into());
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
            let v =
                SubcommandArgsValue::new(vec!["./path".to_string(), "asdf.txt".to_string()].into());
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
            let v = SubcommandArgsValue::new(vec!["false".to_string(), "true".to_string()].into());
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
            let v = SubcommandArgsValue::new(vec!["234532".to_string(), "5436".to_string()].into());
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
            let v = SubcommandArgsValue::new(vec!["234532".to_string(), "5436".to_string()].into());
            let re = v.get_vec_string();

            // shold be Ok. not Err.
            if let Err(err_message) = re {
                panic!("{}", err_message);
            }
        }
    }

    #[test]
    fn err_case_empty() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()].into());
        let re = v.get_empty();

        // shold be Err, not ok.
        if re.is_ok() {
            panic!("");
        }
    }

    #[test]
    fn err_case_string() {
        let v = SubcommandArgsValue::new(vec![].into());
        let re = v.get_bool();

        // shold be Err, not ok.
        if re.is_ok() {
            panic!("");
        }
    }

    // #[test]
    // fn err_case_path() {
    //     let v = SubcommandArgsValue::new(vec!["false".to_string()]);
    //     let re = v.get_path();

    //     // shold be Err, not ok.
    //     if let Ok(_) = re {
    //         panic!("");
    //     }
    // }

    #[test]
    fn err_case_number() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()].into());
        let re = v.get_number();

        // shold be Err, not ok.
        if re.is_ok() {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_bool() {
        let v = SubcommandArgsValue::new(vec!["faasdflse".to_string()].into());
        let re = v.get_bool();

        // shold be Err, not ok.
        if re.is_ok() {
            panic!("");
        }
    }

    #[test]
    fn err_case_vec_number() {
        let v = SubcommandArgsValue::new(vec!["false".to_string()].into());
        let re = v.get_number();

        // shold be Err, not ok.
        if re.is_ok() {
            panic!("");
        }
    }

    // #[test]
    // fn err_case_vec_path() {
    //     let v = SubcommandArgsValue::new(vec!["".to_string()]);
    //     let re = v.get_path();

    //     // shold be Err, not ok.
    //     if let Ok(_) = re {
    //         panic!("");
    //     }
    // }

    #[test]
    fn err_case_vec_string() {
        let v = SubcommandArgsValue::new(vec!["falseasdf".to_string()].into());
        let re = v.get_bool();

        // shold be Err, not ok.
        if re.is_ok() {
            panic!("");
        }
    }
}
