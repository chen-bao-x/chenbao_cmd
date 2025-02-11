use std::vec;

#[test]
fn it_works() {
    use chenbao_cmd::*;

    // ------- 基础 API 测试 -------
    println!("--------hello--------");

    let app = App::new("cmd")
        .about("这个程序主要是为了测试我写的 cmd crate")
        .author("chen bao")
        .version_message("0.0.1".to_string())
        .add_subcommand(
            SubCommand::new("run")
                .about("运行程序")
                .action(ArgAction::Empty(&|_x| {
                    print!(r#"runing commmand: "run""#);
                })),
        )
        .add_subcommand(
            SubCommand::new("help")
                .about("运行程序")
                .action(ArgAction::Empty(&(|_x| {}))),
        )
        .add_subcommand(
            SubCommand::new("build")
                .short_name("b")
                .about("编译项目")
                .action(ArgAction::Bool(&|_x| {
                    print!("command \"run\"{:?}\n", _x);
                })),
        )
        .add_subcommand(
            SubCommand::new("empty")
                .about("用来测试 ArgCount::Zero ")
                .action(ArgAction::Empty(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_subcommand(
            SubCommand::new("number")
                .about("用来测试 ArgCount::Zero ")
                .action(ArgAction::Number(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_subcommand(
            SubCommand::new("vecnumber")
                .about("用来测试 ArgCount::Zero ")
                .action(ArgAction::NumberMutiple(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_subcommand(
            SubCommand::new("vecbool")
                .about("用来测试 ArgCount::Zero ")
                .action(ArgAction::BoolMutiple(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_subcommand(
            SubCommand::new("vecstring")
                .about("用来测试 ArgCount::Zero ")
                .action(ArgAction::StringMutiple(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_subcommand(
            SubCommand::new("repl")
                .about("用来测试 ArgCount::Repl(_) ")
                .action(ArgAction::Dialog(
                    &(|r| {
                        let items = vec!["one", "two", "tree", "four"];

                        let _req_bool = r.number("你要吃几个汉堡包?");
                        let _你要吃几个汉堡包 = r.number_multiple("多个 number");
                        let _多个_number = r.string("string");
                        let _string = r.string_multiple("string_multiple");
                        let _string_multiple = r.yes_or_no("bool");
                        let _path = r.path("path");
                        let _path_multiple = r.path_multiple("path");
                        let _seleted = r.select("selete", &items);
                        let _seleted = r.select_multiple("selete", &items);
                        // .yes_or_no_multiple(&mut req_bool_multiple, "bool mutiple")
                    }),
                )),
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
