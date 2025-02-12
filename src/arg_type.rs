//!  命令行参数的类型,

pub type String = std::string::String;
pub type Number = i64;
pub type NumberMutiple = Vec<Number>;
pub type StringMutiple = Vec<String>;
pub type Path = std::path::PathBuf;
pub type PathMutiple = Vec<std::path::PathBuf>;
pub type Bool = bool;
pub type BoolMutiple = Vec<bool>;
pub type Dialog = crate::question_and_anser::DialogGenerator;
// pub type Empty = self::EmptyArg;

/// 用来表示这个 subcommand 不需要参数.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}

// -------

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)] // 使用 untagged enum
enum ReplArg {
    Number(i64),
    NumberMultiple(Vec<i64>),
    String(String),
    StringMultiple(Vec<String>),
    Path(std::path::PathBuf),
    PathMultiple(Vec<std::path::PathBuf>),
    Bool(bool),
    BoolMultiple(Vec<bool>),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(untagged)] // 使用 untagged enum
struct ReplArgStore(std::collections::HashMap<String, ReplArg>);
impl ReplArgStore {
    fn new() -> Self {
        Self(std::collections::HashMap::new())
    }

    fn add(&mut self, index: usize, prompt: &str, v: ReplArg) -> Option<ReplArg> {
        self.insert(ReplArgStore::key_gen(index, prompt), v)
    }

    fn insert(&mut self, k: String, v: ReplArg) -> Option<ReplArg> {
        self.0.insert(k, v)
    }

    fn key_gen(index: usize, prompt: &str) -> String {
        format!("{}_{}", index.to_string(), prompt)
    }
}

impl Default for ReplArgStore {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn toml_vec_numbers() {
    use std::path::Path;
    let val: Vec<ReplArg> = vec![
        ReplArg::String("string".to_string()),
        ReplArg::StringMultiple(vec![
            "string".to_string(),
            "string".to_string(),
            "string".to_string(),
        ]),
        ReplArg::Number(1),
        ReplArg::NumberMultiple(vec![1, 2, 3]),
        ReplArg::Bool(true),
        ReplArg::Bool(false),
        ReplArg::BoolMultiple(vec![true, false, true, false]),
        ReplArg::Path(Path::new("./hello.txt").to_path_buf()),
        ReplArg::PathMultiple(vec![
            Path::new("./hello.txt").to_path_buf(),
            Path::new("./hello.txt").to_path_buf(),
            Path::new("./hello.txt").to_path_buf(),
            Path::new("./hello.txt").to_path_buf(),
            Path::new("./hello.txt").to_path_buf(),
        ]),
    ];

    let mut toml_store: ReplArgStore = ReplArgStore::new();
    let mut index = 0_usize;
    val.iter().for_each(|x| {
        let key = format!("{}", "key");
        toml_store.add(index, &key, x.clone());
        index += 1;
    });

    println!("{:?}", toml_store);
    let dsaf = toml::to_string_pretty(&toml_store);
    match dsaf {
        Ok(_s) => println!("{}", _s),
        Err(_err) => println!("{:?}", _err),
    }

    let mut h: ReplArgStore = ReplArgStore::new();
    h.insert("k".to_string(), ReplArg::Number(123432));
    let dsaf = toml::to_string_pretty(&h);

    match dsaf {
        Ok(_s) => println!("{}", _s),
        Err(_err) => println!("{:?}", _err),
    }
}
