// ------- Public -------

pub use action::ArgAction;
pub use application::App;
pub use application::DidHandled;
pub use subcommand::SubCommand;
pub mod arg_type;

// ------- Private -------

use helper::*;
use vec_string::*;
mod action;
mod application;
mod examples_types;
mod helper;
mod question_and_anser;
mod subcommand;
mod vec_string;

#[cfg(test)]
mod tests {

    use std::{rc::Rc, vec};

    use super::*;

    #[test]
    fn it_works() {
        // ------- 基础 API 测试 -------
        println!("--------hello--------");

        let app = App::new("cmd")
            .about("这个程序主要是为了测试我写的 cmd crate")
            .author("chen bao")
            .version_message("0.0.1".to_string())
            .add_subcommand(
                SubCommand::new("run")
                    .about("运行程序")
                    .action(ArgAction::Empty(Rc::new(|| {
                        print!(r#"runing commmand: "run""#);
                    }))),
            )
            .add_subcommand(
                SubCommand::new("help")
                    .about("运行程序")
                    .action(ArgAction::Empty(Rc::new(|| {}))),
            )
            .add_subcommand(
                SubCommand::new("build")
                    .short_name("b")
                    .about("编译项目")
                    .action(ArgAction::Bool(Rc::new(|_x| {
                        print!("command \"run\"{:?}\n", _x);
                    }))),
            )
            .add_subcommand(
                SubCommand::new("empty")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::Empty(Rc::new(|| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("number")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::Number(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("vecnumber")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::NumberMutiple(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("vecbool")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::BoolMutiple(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("vecstring")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::StringMutiple(Rc::new(|_x| {
                        print!("testing arg_zero");
                    }))),
            )
            .add_subcommand(
                SubCommand::new("repl")
                    .about("用来测试 ArgCount::Repl(_) ")
                    .action(ArgAction::Dialog(Rc::new(|r| {
                        let mut 你要吃几个汉堡包: arg_type::Number = 0;
                        let mut 多个_number: arg_type::NumberMutiple = vec![];
                        let mut string: String = String::new();
                        let mut string_multiple: Vec<String> = vec![];
                        let mut req_bool: arg_type::Bool = false;
                        // let mut req_bool_multiple: arg_type::BoolMutiple = vec![];

                        r._number(&mut 你要吃几个汉堡包, "你要吃几个汉堡包?")
                            ._number_multiple(&mut 多个_number, "多个 number")
                            ._string(&mut string, "string")
                            ._string_multiple(&mut string_multiple, "string_multiple")
                            ._yes_or_no(&mut req_bool, "bool")
                            // .yes_or_no_multiple(&mut req_bool_multiple, "bool mutiple")
                            ;
                    }))),
            );

        // let re = app.debug_duplicate_names_check();
        // match re {
        //     Ok(_) => {}
        //     Err(s) => println!("这些命令名称重复了: ··{:?}", s),
        // }

        let _ = app
            // .deubg_run(vec!["cmd", "-e"])
            // .deubg_run(vec!["cmd", "help"])
            // .deubg_run(vec!["cmd", "-h"])
            // .deubg_run(vec!["cmd", "b"])
            .deubg_run(vec!["cmd", "build", "true"])
            // .deubg_run(vec!["cmd", "build", "-h"])
            // .deubg_run(vec!["cmd", "build", "-e"])
            // .deubg_run(vec!["cmd", "run"])
            // .deubg_run(vec!["cmd", "run", "3"])
            // .deubg_run(vec!["cmd", "run", "3", "32"]) // 类型正确, 数量不正确
            // .deubg_run(vec!["cmd", "run", "-h"])
            // .deubg_run(vec!["cmd", "-h"])
            // .deubg_run(vec!["cmd"])
            // .deubg_run(vec!["cmd", "arg_one", "-h"])

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
