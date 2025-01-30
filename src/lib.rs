mod application;
pub use application::*;

mod command;
pub use command::*;

mod arg_count;
pub use arg_count::*;

mod helper;
pub use helper::*;

mod arg_types;
pub use arg_types::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DidHandled {
    /// 表示匹配到了相关命令并正确执行了相关 action.
    Handled,

    /// 没匹配到相关命令或者其他错误.
    Failed(String),
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn it_works() {
        // ------- 基础 API 测试 -------
        println!("--------hello--------");

        let app = App::new("cmd")
            .about("这个程序主要是为了测试我写的 cmd crate")
            .author("chen bao")
            .app_version_message("0.0.1".to_string())
            .add_command(
                Command::new("run")
                    .about("运行程序")
                    .add_command_example("app ruh  \n运行程序")
                    .action(ArgCount::Zero, |_x| {
                        print!("command \"run\"\n");
                    }),
            )
            .add_command(
                Command::new("build")
                    .short_name("b")
                    .about("编译项目")
                    .add_command_example("app ruh  \n运行程序")
                    .action(ArgCount::Zero, |x| {
                        print!("command \"build\"{:?}\n", x);
                    }),
            )
            .add_command(
                Command::new("arg_zero")
                    .about("用来测试 ArgCount::Zero ")
                    .add_command_example("app arg_zero  ")
                    .add_command_example("app arg_zero a")
                    .add_command_example("app arg_zero \"b\"")
                    .add_command_example("app arg_zero a b c")
                    .action(ArgCount::Zero, |_x| {
                        print!("testing arg_zero");
                    }),
            )
            .add_command(
                Command::new("arg_zero_or_one")
                    .about("用来测试 ArgCount::Zero ")
                    .add_command_example("app arg_zero_or_one  ")
                    .add_command_example("app arg_zero_or_one a")
                    .add_command_example("app arg_zero_or_one \"b\"")
                    .add_command_example("app arg_zero_or_one a b c")
                    .action(ArgCount::ZeroOrOne, |_x| {
                        print!("testing arg_zero_or_one");
                    }),
            )
            .add_command(
                Command::new("arg_zero_or_many")
                    .about("用来测试 ArgCount::Zero ")
                    .add_command_example("app arg_zero_or_many  ")
                    .add_command_example("app arg_zero_or_many a")
                    .add_command_example("app arg_zero_or_many \"b\"")
                    .add_command_example("app arg_zero_or_many a b c")
                    .action(ArgCount::ZoreOrMany, |_x| {
                        print!("testing arg_zero_or_many");
                    }),
            )
            .add_command(
                Command::new("arg_one")
                    .about("用来测试 ArgCount::Zero ")
                    .add_command_example("app arg_one  ")
                    .add_command_example("app arg_one a")
                    .add_command_example("app arg_one \"b\"")
                    .add_command_example("app arg_one a b c")
                    .action(ArgCount::One, |_x| {
                        print!("testing arg_one");
                    }),
            )
            .add_command(
                Command::new("arg_one_or_many")
                    .about("用来测试 ArgCount::Zero ")
                    .add_command_example("app arg_one_or_many  ")
                    .add_command_example("app arg_one_or_many a")
                    .add_command_example("app arg_one_or_many \"b\"")
                    .add_command_example("app arg_one_or_many a b c")
                    .action(ArgCount::OneOrMany, |_x| {
                        print!("testing arg_one_or_many");
                    }),
            )
            .add_command(
                Command::new("arg_count_2")
                    .about("用来测试 ArgCount::Zero ")
                    .add_command_example("app arg_count_2  ")
                    .add_command_example("app arg_count_2 a")
                    .add_command_example("app arg_count_2 \"b\"")
                    .add_command_example("app arg_count_2 a b c")
                    .action(ArgCount::Count(2), |_x| {
                        print!("testing arg_count_2");
                    }),
            )
            
            // .run()
            ;


        let _ = app
            // .test_run(vec!["debug_env_args testing", "b"])
            // .test_run(vec!["debug_env_args testing", "build"])
            // .test_run(vec!["debug_env_args testing", "build", "-h"])
            // .test_run(vec!["debug_env_args testing", "run"])
            // .test_run(vec!["debug_env_args testing", "run","-h"])
            // .test_run(vec!["debug_env_args testing", "-h"])
            // .test_run(vec!["debug_env_args testing"])
            // .test_run(vec!["debug_env_args testing", "help"])
            .test_run(vec!["debug_env_args testing", "arg_one","-h"])
            .test_run(vec!["debug_env_args testing", "-h"])
             


            ;
    }
}
