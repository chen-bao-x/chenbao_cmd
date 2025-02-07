mod application;
use core::num;
use std::rc::Rc;

pub use application::*;

mod subcommand;
pub use subcommand::*;

/// Vec<String> to json  
/// json to Vec<String>  
mod vec_string;
pub use vec_string::*;

mod helper;
pub use helper::*;

mod arg_types;
pub use arg_types::*;

mod examples_types;
pub use examples_types::*;

/// 问答式命令行交互
mod repl_questions;
pub use repl_questions::*;

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
            .deubg_run(vec!["cmd", "arg_one", "-h"]);
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

fn hesadfsadf() {
    eprintln!("hello")
}
