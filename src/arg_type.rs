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
/// 用来表示这个 subcommand 不需要参数.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

// -------

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)] // 使用 转换成 toml 后不会有  NumberMultiple = [324] 这样的 enum key.
pub(crate) enum ReplArg {
    Number(i64),
    NumberMultiple(Vec<i64>),
    String(String),
    StringMultiple(Vec<String>),
    // Path(String),
    // PathMultiple(Vec<String>),
    Bool(bool),
    BoolMultiple(Vec<bool>),
}

impl ReplArg {
    // pub fn get_number(&self) -> i64 {
    pub fn get_number(&self) -> Result<i64, String> {
        if let ReplArg::Number(val) = self {
            return Ok(*val);
        }

        Err(format!("类型不正确: {:?}", self))
    }
    // pub fn get_number_multiple(&self) -> Vec<i64> {
    // pub fn get_number_multiple(&self) -> &Vec<i64> {
    pub fn get_number_multiple(&self) -> Result<&Vec<i64>, String> {
        if let ReplArg::NumberMultiple(val) = self {
            return Ok(val);
        }

        // panic!("{:?}", self);
        Err(format!("类型不正确: {:?}", self))
    }

    // pub fn get_string(&self) -> &str {
    pub fn get_string(&self) -> Result<String, String> {
        if let ReplArg::String(val) = self {
            // return val.to_string();
            return Ok(val.to_string());
        }
        // panic!("{:?}", self);
        Err(format!("类型不正确: {:?}", self))
    }
    // pub fn get_string_multiple(&self) -> &Vec<String> {
    pub fn get_string_multiple(&self) -> Result<&Vec<String>, String> {
        if let ReplArg::StringMultiple(val) = self {
            return Ok(val);
        }
        // panic!("{:?}", self);
        Err(format!("类型不正确: {:?}", self))
    }
    // pub fn get_path(&self) -> std::path::PathBuf {
    pub fn get_path(&self) -> Result<std::path::PathBuf, String> {
        if let ReplArg::String(val) = self {
            let path = std::path::Path::new(val).to_path_buf();
            return Ok(path);
            // return val.to_path_buf();
        }
        // panic!("{:?}", self);
        Err(format!("类型不正确: {:?}", self))
    }
    // pub fn get_path_multiple(&self) -> Vec<std::path::PathBuf> {
    pub fn get_path_multiple(&self) -> Result<Vec<std::path::PathBuf>, String> {
        if let ReplArg::StringMultiple(val) = self {
            // return
            let paths: Vec<std::path::PathBuf> = val
                .iter()
                .map(|x| std::path::Path::new(x).to_path_buf())
                .collect();

            return Ok(paths);
        }
        Err(format!("类型不正确: {:?}", self))
    }
    // pub fn get_bool(&self) -> bool {
    pub fn get_bool(&self) -> Result<bool, String> {
        if let ReplArg::Bool(val) = self {
            return Ok(*val);
        }
        Err(format!("类型不正确: {:?}", self))
    }

    // pub fn get_bool_multiple(&self) -> &Vec<bool> {
    //     if let ReplArg::BoolMultiple(val) = self {
    //         return val;
    //     }
    //     panic!("{:?}", self);
    // }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub(crate) struct ReplArgStore(std::collections::BTreeMap<String, ReplArg>); // BTreeMap 是按照 key 来排序的,  HashMap 是无序的.
impl ReplArgStore {
    pub fn new() -> Self {
        // Self(std::collections::HashMap::new())
        Self(std::collections::BTreeMap::new())
    }

    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    pub fn add(&mut self, index: usize, prompt: &str, v: ReplArg) -> Option<ReplArg> {
        self.insert(key_gen(index, prompt), v)
    }

    pub fn insert(&mut self, k: String, v: ReplArg) -> Option<ReplArg> {
        self.0.insert(k, v)
    }

    // pub fn remove(&mut self, index: usize, prompt: &str) -> Option<ReplArg> {
    //     self.0.remove(&key_gen(index, prompt))
    // }

    pub fn get(&self, index: usize, prompt: &str) -> Option<&ReplArg> {
        let key = key_gen(index, prompt);

        self.0.get(&key)
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        for (key, val) in &self.0 {
            println!("key: {}, val: {:?}", key, val);
        }

        toml::to_string_pretty(&self)
    }
}

/// ReplArgStore key generator.
pub(crate) fn key_gen(index: usize, prompt: &str) -> String {
    // 加数字是为了避免有相同的 key. 毕竟是存储在 BTreeMap 中的
    format!("{:02}_{}", index, prompt)
}

impl Default for ReplArgStore {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn toml_vec_numbers() {
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
        ReplArg::String(("./hello.txt").to_string()),
        ReplArg::StringMultiple(vec![
            ("./hello.txt").to_string(),
            ("./hello.txt").to_string(),
            ("./hello.txt").to_string(),
            ("./hello.txt").to_string(),
            ("./hello.txt").to_string(),
        ]),
    ];

    let mut toml_store: ReplArgStore = ReplArgStore::new();
    let mut index = 0_usize;
    val.iter().for_each(|x| {
        let key = "key".to_string();
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
