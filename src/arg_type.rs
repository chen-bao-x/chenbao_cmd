//!  命令行参数的类型,

pub type String = std::string::String;
pub type StringMutiple = Vec<String>;
pub type Number = i128;
pub type NumberMutiple = Vec<i128>;
pub type Path = std::path::PathBuf;
pub type PathMutiple = Vec<std::path::PathBuf>;
pub type Bool = bool;
pub type BoolMutiple = Vec<bool>;
pub type Dialog = crate::question_and_anser::DialogGenerator;
// pub type Empty = self::Empty;

/// 用来表示这个 subcommand 不需要参数.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}
