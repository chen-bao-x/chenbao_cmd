pub(crate) type ReplArgType = std::collections::HashMap<usize, (String, String)>;

pub(crate) struct HashMapString;
impl HashMapString {
    pub(crate) fn json_to_vec(json_string: &str) -> Result<ReplArgType, serde_json::Error> {
        return serde_json::from_str(json_string);
    }

    pub(crate) fn vec_to_json(vec: &ReplArgType) -> String {
        let json_value = serde_json::json!(vec);
        return serde_json::to_string_pretty(&json_value).unwrap();
        // return json_value.to_string();
    }
}

#[test]
fn sdafdsaf() {
    pub(crate) type ReplArgType = std::collections::HashMap<usize, (String, String)>;
    let mut val: ReplArgType = ReplArgType::new();

    let value = "val".to_string();
    let a = 1..199;
    a.for_each(|x| {
        let key = format!("{}{}", "key", x,);
        val.insert(x, (key, value.clone()));
    });

    let sadf = HashMapString::vec_to_json(&val);

    println!("{}", sadf);
}

pub(crate) struct VecString;
impl VecString {
    pub(crate) fn json_to_vec(json_string: &str) -> Result<Vec<String>, serde_json::Error> {
        return serde_json::from_str(json_string);
    }

    pub(crate) fn vec_to_json(vec: &Vec<String>) -> String {
        let json_value = serde_json::json!(vec);
        return serde_json::to_string_pretty(&json_value).unwrap();
        // return json_value.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use owo_colors::OwoColorize;

    #[test]
    fn vec_json是否能相互逆转() {
        let v1 = vec!["hello".to_string(), "world".to_string()];

        let json_str = VecString::vec_to_json(&v1);

        let v2 = VecString::json_to_vec(&json_str).expect("json to VecString 失败");

        println!("json_str: {:?}", json_str.cyan());
        println!("v1: {:?}\nv2: {:?}", v1.cyan(), v2.cyan());
        println!("v1 == v2  -> {}  ", v1 == v2); // true 可以还原

        assert_eq!(v1, v2);
    }
}
