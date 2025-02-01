use super::*;

// /// 问答式命令行交互
// #[derive(Debug)]
// pub struct ReplQA {
//     pub tips: &'static str,

//     /// need ArgType
//     pub need_arg_type: ArgType,

//     pub value: Option<SubcommandArgsValue>,

//     pub when_failed: WhenFailed,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum WhenFailed {
//     Terminate,
//     Continue,
// }

// impl ReplQA {
//     pub fn run(&mut self) {
//         println!("{}\n{}", self.tips, self.need_arg_type.arg_type_tips());

//         let mut input = String::new();
//         let re = std::io::stdin().read_line(&mut input);
//         match re {
//             Ok(_) => {
//                 let input = input.trim_end_matches('\n');

//                 let args = parse_arg_string(input);
//                 println!("{:?}", args);

//                 let v = SubcommandArgsValue::new(self.need_arg_type.clone(), args);

//                 match self.need_arg_type {
//                     ArgType::Empty => {
//                         let re = v.clone().get_empty();

//                         match re {
//                             Ok(_) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);

//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::String => {
//                         let re = v.clone().get_string();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecString => {
//                         let re = v.clone().get_vec_string();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Number => {
//                         let re = v.clone().get_number();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecNumber => {
//                         let re = v.clone().get_number();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Path => {
//                         let re = v.clone().get_path();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecPath => {
//                         let re = v.clone().get_vec_path();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Bool => {
//                         let re = v.clone().get_bool();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::VecBool => {
//                         let re = v.clone().get_vec_bool();

//                         match re {
//                             Ok(a) => {
//                                 self.value = Some(v);
//                             }
//                             Err(_e) => {
//                                 match self.when_failed {
//                                     WhenFailed::Terminate => {
//                                         println!("{}", _e);
//                                         exit(0);
//                                     }
//                                     WhenFailed::Continue => {
//                                         self.run();
//                                     }
//                                 };
//                             }
//                         }
//                     }
//                     ArgType::Repl => todo!(),
//                 };
//             }
//             Err(f) => {
//                 match self.when_failed {
//                     WhenFailed::Terminate => {
//                         println!("{}", f);
//                         exit(0);
//                     }
//                     WhenFailed::Continue => {
//                         self.run();
//                     }
//                 };
//             }
//         };
//     }

//     // yes or no QA
//     // true or false QA
//     // number QA
//     // vec<number> QA
//     // string QA
//     // Vec<string> QA
//     // path QA
//     // Vec<path> QA
//     // password QA whith confirm

//     // enum single selection QA
//     // enum multi selection QA

//     // path single selection QA
//     // path multi selection QA

// }

// #[test]
// fn adsfsadf() {
//     let mut repl = ReplQA {
//         tips: "tips",
//         need_arg_type: ArgType::VecBool,
//         value: None,
//         when_failed: WhenFailed::Continue,
//     };

//     repl.run();

//     println!("{:?}", repl);
// }

fn dsafdsaf(o: Option<String>) {
    match o {
        Some(json) => {
            // 命令行程序的使用者个此命令传入了一个 JSON,
            // 需要将此 JSON 转换为需要的数据.
        }
        None => {
            // 问答式命令行交互 来获取所需要的 参数们.
        }
    };
}

fn asdfasdf() {
    let mut a = true;
    ArgType::Bool(Rc::new(|x| {}));

    asdfsdafasdff(|| {
        a = false;
    })
}

fn asdfsdafasdff<F>(mut f: F)
where
    F: FnMut() -> (),
{
    f();
}

//     // yes or no QA
//     // true or false QA
//     // number QA
//     // vec<number> QA
//     // string QA
//     // Vec<string> QA
//     // path QA
//     // Vec<path> QA
//     // password QA whith confirm

//     // enum single selection QA
//     // enum multi selection QA

//     // path single selection QA
//     // path multi selection QA

pub struct ReplQuestions {
    
    /// 从 json_str 转换过来的 Vec<String>.
    /// 也可能是通过 问答式命令行交互 获取到的 Vec<String>.
    arr: Vec<String>,

    /// 当 Self 是从 json_str 转换过来的 Vec<String> 时,
    /// 这个用户标记读取到了哪一个参数.
    index: usize,

    /// 是否是从 json_str 转换过来的?
    is_from_jsonstr: bool,
}

impl ReplQuestions {
    /* private */
    pub(crate) fn new(input: Option<String>) -> Self {
        match input {
            Some(s) => Self::new_from_jsonstr(s),
            None => Self {
                arr: vec![],
                index: 0,
                is_from_jsonstr: false,
            },
        }
    }

    fn new_from_jsonstr(str: String) -> Self {
        // str -> Vec<String>
        Self {
            arr: vec![],
            index: 0,
            is_from_jsonstr: true,
        }
    }

    fn to_json_str() -> String {
        return String::new();
    }

    fn get_string(self, mut result_value: String) -> Self {
        let mut re = self;
        let val = re.arr.get(re.index);

        match val {
            Some(str) => {
                // 成功获取到了需要的参数
                result_value = str.clone();
            }
            None => {
                // not string
            }
        }

        re.index += 1;
        return re;
    }

    fn get_number(self, mut result_value: Number, tips: &str) -> Self {
        let mut re = self;

        if re.is_from_jsonstr {
            let val = re.arr.get(re.index);

            match val {
                Some(str) => {
                    let number_from_str: Result<Number, std::num::ParseIntError> = str.parse();
                    match number_from_str {
                        Ok(x) => {
                            // 成功获取到了需要的参数
                            result_value = x;
                        }
                        Err(_e) => {
                            // 为能转换为 Number 类型.
                        }
                    }
                }
                None => {
                    // not string
                }
            }
        } else {
            // get value from REPL.
        }

        _ = result_value;

        re.index += 1;
        return re;
    }
}

#[cfg(test)]
mod test {
    use std::default;

    use crate::{Number, ReplQuestions};

    #[test]
    fn it_works() {
        let mut x: Number = Default::default();
        let mut s: String = Default::default();

        let r = ReplQuestions::new(None);
        r.get_number(x, "你想买几个汉堡?").get_string(s);
    }
}
