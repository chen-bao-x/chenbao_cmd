mod application;
pub use application::*;

mod subcommand;
pub use subcommand::*;

mod helper;
pub use helper::*;

mod arg_types;
pub use arg_types::*;

mod examples_types;
pub use examples_types::*;

// mod arg_count;
// pub use arg_count::*;


#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use super::*;

    #[test]
    fn it_works() {
        // ------- 基础 API 测试 -------
        println!("--------hello--------");

        let app = App::new("cmd")
            .add_about("这个程序主要是为了测试我写的 cmd crate")
            .add_author("chen bao")
            .app_version_message("0.0.1".to_string())
            //         .add_app_example(vec![
            // "app example 1".to_string(),
            // "app example 2".to_string(),
            // "app example 2".to_string(),
            // "app run 3".to_string(),
            //         ])
            .add_subcommand(
                SubCommand::new("run")
                    .about("运行程序")
                    .action(ArgType::Bool(Rc::new(|_x| {
                        print!("command \"run\"{:?}\n", _x);
                    }))),
            )
            .add_subcommand(
                SubCommand::new("build")
                    .short_name("b")
                    .about("编译项目")
                    .action(ArgType::Bool(Rc::new(|_x| {
                        print!("command \"run\"{:?}\n", _x);
                    }))),
            )
            .add_subcommand(
                SubCommand::new("empty")
                    .about("用来测试 ArgCount::Zero ")
                    // .add_command_example("app arg_zero  ")
                    // .add_command_example("app arg_zero a")
                    // .add_command_example("app arg_zero \"b\"")
                    // .add_command_example("app arg_zero a b c")
                    .action(ArgType::Empty(Rc::new(|| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("number")
                    .about("用来测试 ArgCount::Zero ")
                    // .add_command_example("app arg_zero  ")
                    // .add_command_example("app arg_zero a")
                    // .add_command_example("app arg_zero \"b\"")
                    // .add_command_example("app arg_zero a b c")
                    .action(ArgType::Number(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("vecnumber")
                    .about("用来测试 ArgCount::Zero ")
                    // .add_command_example("app arg_zero  ")
                    // .add_command_example("app arg_zero a")
                    // .add_command_example("app arg_zero \"b\"")
                    // .add_command_example("app arg_zero a b c")
                    .action(ArgType::VecNumber(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("vecbool")
                    .about("用来测试 ArgCount::Zero ")
                    // .add_command_example("app arg_zero  ")
                    // .add_command_example("app arg_zero a")
                    // .add_command_example("app arg_zero \"b\"")
                    // .add_command_example("app arg_zero a b c")
                    .action(ArgType::VecBool(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("vecstring")
                    .about("用来测试 ArgCount::Zero ")
                    // .add_command_example("app vecstring  ")
                    // .add_command_example("app arg_zero a")
                    // .add_command_example("app arg_zero \"b\"")
                    // .add_command_example("app arg_zero a b c")
                    .action(ArgType::VecString(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            );

        let re = app.debug_duplicate_names_check();
        match re {
            Ok(_) => {}
            Err(s) => panic!("这些命令名称重复了: ··{:?}", s),
        }

        let _ = app
            .deubg_run(vec!["cmd", "-e"])
            .deubg_run(vec!["cmd", "help"])
            .deubg_run(vec!["cmd", "-h"])
            .deubg_run(vec!["cmd", "b"])
            .deubg_run(vec!["cmd", "build"])
            .deubg_run(vec!["cmd", "build", "-h"])
            .deubg_run(vec!["cmd", "build", "-e"])
            .deubg_run(vec!["cmd", "run"])
            .deubg_run(vec!["cmd", "run", "3"])
            .deubg_run(vec!["cmd", "run", "3", "32"]) // 类型正确, 数量不正确
            .deubg_run(vec!["cmd", "run", "-h"])
            .deubg_run(vec!["cmd", "-h"])
            .deubg_run(vec!["cmd"])
            .deubg_run(vec!["cmd", "arg_one", "-h"])
            //
            ;
    }

    #[test]
    fn maadsfin() {
        println!("请输入一些文本：");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");

        println!("你输入了：{}", input.trim());
    }
}

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
