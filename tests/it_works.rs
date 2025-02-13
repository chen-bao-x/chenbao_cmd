use std::vec;
use chenbao_cmd::*;

#[test]
fn it_works() {
    use chenbao_cmd::*;

    // ------- 基础 API 测试 -------
    println!("--------hello--------");

    let app =
        App::new()
            .about("这个程序主要是为了测试我写的 cmd crate")
            .author("chen bao")
            .version_message("0.0.1".to_string())
            .app_name("app")
            .add_command(
                SubCommand::create_an_sub_command("run")
                    .about("运行程序")
                    .action(ArgAction::Empty(&|_x| {
                        print!(r#"runing commmand: "run""#);
                    })),
            )
            .add_command(
                SubCommand::create_an_sub_command("help")
                    .about("运行程序")
                    .action(ArgAction::Empty(&(|_x| {}))),
            )
            .add_command(SubCommand::create_an_sub_command("build").short_name("b").about("编译项目").action(
                ArgAction::Bool(&|_x| {
                    println!("command \"run\"{:?}", _x);
                }),
            ))
            .add_command(SubCommand::create_an_sub_command("empty").about("用来测试 ArgCount::Zero ").action(
                ArgAction::Empty(&|_x| {
                    print!("testing arg_zero");
                }),
            ))
            .add_command(SubCommand::create_an_sub_command("number").about("用来测试 ArgCount::Zero ").action(
                ArgAction::Number(&|_x| {
                    print!("testing arg_zero");
                }),
            ))
            .add_command(
                SubCommand::create_an_sub_command("vecnumber")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::NumberMutiple(&|_x| {
                        print!("testing arg_zero");
                    })),
            )
            .add_command(
                SubCommand::create_an_sub_command("vecbool")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::BoolMutiple(&|_x| {
                        print!("testing arg_zero");
                    })),
            )
            .add_command(
                SubCommand::create_an_sub_command("vecstring")
                    .about("用来测试 ArgCount::Zero ")
                    .action(ArgAction::StringMutiple(&|_x| {
                        print!("testing arg_zero");
                    })),
            )
            .add_command(
                SubCommand::create_an_sub_command("repl")
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
        .deubug_run(vec!["cmd", "-e"])
        .deubug_run(vec!["cmd", "help"])
        .deubug_run(vec!["cmd", "-h"])
        .deubug_run(vec!["cmd", "b"])
        .deubug_run(vec!["cmd", "build", "true"])
        .deubug_run(vec!["cmd", "build", "-h"])
        .deubug_run(vec!["cmd", "build", "-e"])
        .deubug_run(vec!["cmd", "run"])
        .deubug_run(vec!["cmd", "run", "3"])
        .deubug_run(vec!["cmd", "run", "3", "32"]) // 类型正确, 数量不正确
        .deubug_run(vec!["cmd", "run", "-h"])
        .deubug_run(vec!["cmd", "-h"])
        .deubug_run(vec!["cmd"])
        .deubug_run(vec!["cmd", "arg_one", "-h"]);
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

use std::backtrace::Backtrace;

fn foo() {
    let backtrace = Backtrace::capture();
    println!("{:?}", backtrace);
}

fn bar() {
    foo();
}

#[test]
fn msadfdsafain() {
    bar();
}

#[test]
fn adsf() {
    use chenbao_cmd::*;
    println!("{:?}", ArgAction::Empty(&|_| {}));
}

#[test]
fn dsafdsaf() {
    let app = App::new( )
    // .app_default_action(&|| { /* action */ })
    .about("这个程序主要是为了测试我写的 cmd crate")
    // .author("chen bao")
    .app_name("app")
    .version_message("0.0.1".to_string())

    .add_command(
        SubCommand::create_an_sub_command("-")
            .short_name("b")
            .about("编译项目")
            .action(ArgAction::String(&(|_x| {
                println!("command \"run\"{:?}", _x);
            }))),
    )
    .add_command(
        SubCommand::create_an_sub_command("build")
            .short_name("b")
            .about("编译项目")
            .action(ArgAction::Bool(&(|_x| {
                println!("command \"run\"{:?}", _x);
            }))),
    )
    .add_command(
        SubCommand::create_an_sub_command("empty")
            .about("用来测试 ArgCount::Zero ")
            // .add_example("app arg_zero  ", "")
            .action(ArgAction::Empty(&(|_x| {
                print!("testing arg_zero");
            }))),
    )
    .add_command(
        SubCommand::create_an_sub_command("number")
            .about("用来测试 ArgCount::Zero ")
            .action(ArgAction::Number(&(|_x| {
                print!("testing arg_zero");
            }))),
    )
    .add_command(
        SubCommand::create_an_sub_command("vecnumber")
            .about("用来测试 ArgCount::Zero ")
            .action(ArgAction::NumberMutiple(&(|_x| {
                println!("testing vec number {:?}", _x);
            }))),
    )
    .add_command(
        SubCommand::create_an_sub_command("vecbool")
            .about("用来测试 ArgCount::Zero ")

            .action(ArgAction::BoolMutiple(&(|_x| {
                println!("testing vecbool {:?}", _x);
            }))),
    )
    .add_command(
        SubCommand::create_an_sub_command("run")
            .about("运行程序")
            // .add_example("cmd run true", "description")
            // .add_example("cmd run", "description")
            // .add_example("cmd run", "description description description description description description description ")
            // .add_example("cmd run", "description")
            // .add_example("cmd run", "description")
            // .add_example("cmd run", "description")
            // .add_example("cmd run", "description")
            // .add_example("cmd run", "description")
            // .add_example("cmd run", "description")
            .action(ArgAction::Bool(&(|_x| {
                println!("command \"run\"{:?}", _x);
            }))),
    )
    .add_command(
        SubCommand::create_an_sub_command("vecstring")
            .short_name("vs")
            .about("用来测试 vecstring ")
            // .add_example("command", "description")
            .help_document("str".to_string())
            .action(ArgAction::StringMutiple(&(|_x| {
                println!("testing vecstring {:?}", _x);
            }))),
    )
     
    ;

    let _ = app
        .deubug_run(vec!["app_name", "-e"])
        // .deubug_run(vec!["app_name", "help"])
        // .deubug_run(vec!["app_name", "-h"])
        // .deubug_run(vec!["app_name", "b"])
        // .deubug_run(vec!["app_name", "build"])
        // .deubug_run(vec!["app_name", "build", "-h"])
        // .deubug_run(vec!["app_name", "build", "-e"])
        // .deubug_run(vec!["app_name", "run"])
        // .deubug_run(vec!["app_name", "run", "3"])
        // .deubug_run(vec!["app_name", "run", "3", "32"]) // 类型正确, 数量不正确
        // .deubug_run(vec!["app_name", "run", "-h"])
        // .deubug_run(vec!["app_name", "run", "-e"])
        // .deubug_run(vec!["app_name", "-h"])
        // .deubug_run(vec!["app_name"])
        // .deubug_run(vec!["app_name", "repl"])
        // .deubug_run(vec!["app_name", "run"])
        // .deubug_run(vec!["app_name", "build"])
        // .deubug_run(vec!["app_name", "empty"])
        // .deubug_run(vec!["app_name", "number"])
        // .deubug_run(vec!["app_name", "vecnumber"])
        // .deubug_run(vec!["app_name", "vecbool"])
        // .deubug_run(vec!["app_name", "vecstring"])
        // .deubug_run(vec!["app_name", "repl"])
        .deubug_run(vec!["app_name", "--list-all-commands"])
        // 
        ;
}
