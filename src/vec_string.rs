use owo_colors::OwoColorize;

pub struct VecString(Vec<String>);
impl VecString {
    pub fn new_from_vec(vec: Vec<String>) -> Self {
        Self { 0: vec }
    }

    pub fn new_from_json_str(json_string: &str) -> Result<Self, serde_json::Error> {
        // 将 JSON 字符串反序列化为 Vec<String>
        let result: Result<Vec<String>, serde_json::Error> = serde_json::from_str(json_string);

        match result {
            Ok(my_vec) => {
                println!("{:?}", my_vec.clone()); // 输出: ["hello", "world"]

                let asdf = VecString::new_from_vec(my_vec);
                return Ok(asdf);
            }
            Err(err) => {
                println!("Error: {}", err);
                return Err(err);
            }
        };
    }

    pub fn to_json_str(&self) -> String {
        let json_value = serde_json::json!(self.0);
        return json_value.to_string();
    }
}

#[test]
fn vec_string_json() {
    let my_vec = vec!["hello".to_string(), "world".to_string()];

    let v1 = VecString::new_from_vec(my_vec);
    let json_str = v1.to_json_str();

    let v2 = VecString::new_from_json_str(&json_str).expect("json to VecString 失败");

    println!("json_str: {:?}", json_str.cyan());
    println!("v1: {:?}\nv2: {:?}", v1.0.cyan(), v2.0.cyan());
    println!("v1.0 == v2.0  -> {}  ", v1.0 == v2.0); // true 可以还原
}
