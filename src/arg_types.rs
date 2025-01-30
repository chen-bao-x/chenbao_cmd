use std::{
    num::ParseIntError,
    path::{self, PathBuf},
};

pub type Number = usize;

pub type Path = path::Path;

pub type ParseResult<T> = Result<T, String>;

/// 用来表示这个 subcommand 不需要参数.
pub struct Empty {}

/// String
/// Number
/// Path
/// bool
/// Empty
pub trait SubcommandArgsType {
    /// subcommand_args: 子命令的参数.
    fn parse_from_args(subcommand_args: Vec<String>) -> ParseResult<Box<Self>>;
}

// ------- Zero -------

impl SubcommandArgsType for Empty {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        if s.len() > 0 {
            return Err(format!(
                "参数数量不正确: 此命令不需要参数, 实际接收到了 {} 个参数: {:?}",
                s.len(),
                s,
            ));
        } else {
            return Ok(Box::new(Empty {}));
        }
    }
}

// ------- String -------

/// 1 个 String
impl SubcommandArgsType for String {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        if s.len() == 1 {
            if let Some(str) = s.first() {
                return ParseResult::Ok(Box::new(str.clone()));
            }
        }
        return Err(format!(
            "参数数量不正确: 需要 1 个参数, 实际接收到了 {} 个参数: {:?}",
            s.len(),
            s,
        ));
    }
}

/// 0 个 或者 多个 String
impl SubcommandArgsType for Vec<String> {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        return Ok(Box::new(s));
    }
}

// ------- Number -------

/// 1 个 number
impl SubcommandArgsType for Number {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re: usize = str.parse().unwrap();
                return Ok(Box::new(re));
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
}

/// 0 个 或者 多个 Number
impl SubcommandArgsType for Vec<Number> {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
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

        return Ok(Box::new(re));
    }
}

// ------- PathBuf -------

/// 1 个 PathBuf
impl SubcommandArgsType for PathBuf {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        if s.len() == 1 {
            if let Some(str) = s.first() {
                let re = Path::new(str).to_owned();

                return Ok(Box::new(re));
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
}

/// 0 个 或者 多个 pathBuf

impl SubcommandArgsType for Vec<PathBuf> {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        let mut re: Vec<PathBuf> = vec![];

        for x in s {
            let u = Path::new(&x).to_owned();
            re.push(u);
        }

        return Ok(Box::new(re));
    }
}

// ------- bool -------

/// 1 个 bool
impl SubcommandArgsType for bool {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        if s.len() == 1 {
            if let Some(str) = s.first() {
                let lovwercase = str.to_lowercase();
                {
                    if lovwercase == "true" {
                        return Ok(Box::new(true));
                    }
                }

                {
                    if lovwercase == "false" {
                        return Ok(Box::new(false));
                    }
                }

                return Err(format!(
                    "参数不正确: 参数的类型是 bool, bool 类型的值可以是: true t false f",
                ));
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
}

/// 0 个 或者 多个 bool 
impl SubcommandArgsType for Vec<bool> {
    fn parse_from_args(s: Vec<String>) -> ParseResult<Box<Self>> {
        let mut re: Vec<bool> = vec![];

        for x in s {
            let lovwercase = x.to_lowercase();
            {
                if lovwercase == "true" {
                    re.push(true);
                }
            }

            {
                if lovwercase == "false" {
                    re.push(false);
                }
            }

            return Err(format!(
                "参数不正确: 参数的类型是 bool.\nbool 类型的值可以是: true , false ",
            ));
        }

        return Ok(Box::new(re));
    }
}
