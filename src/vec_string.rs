pub(crate) struct VecString();
impl VecString {
    pub(crate) fn json_to_vec(json_string: &str) -> Result<Vec<String>, serde_json::Error> {
        return serde_json::from_str(json_string);
    }

    pub(crate) fn vec_to_json(vec: &Vec<String>) -> String {
        let json_value = serde_json::json!(vec);
        return json_value.to_string();
    }

    pub(crate) fn adsfadsf(a: Vec<String>) -> String {
        let mut str: String = String::new();
        let mut index: usize = 0;
        // serde_json::to_string(&a);

        str.push('[');
        for x in &a {
            str.push_str(&format!("{:?}", x));
            if index < a.len() - 1 {
                str.push_str(", ");
            }

            index += 1;
        }
        str.push(']');
        return str;
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
