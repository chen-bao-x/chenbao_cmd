use chenbao_cmd::*;

#[test]
fn it_works() {
    use chenbao_cmd::*;
    use chenbao_cmd::Arg;

    // ------- 基础 API 测试 -------
    println!("--------hello--------");

    let app = App::new()
        .about("这个程序主要是为了测试我写的 cmd crate")
        .author("chen bao")
        .version_message("0.0.1")
        .app_name("app")
        .add_command(
            cmd!("run")
                .about("运行程序")
                .action(Arg::Empty(&|_x| {
                    print!(r#"runing commmand: "run""#);
                })),
        )
        .add_command(
            cmd!("help")
                .about("运行程序")
                .action(Arg::Empty(&(|_x| {}))),
        )
        .add_command(
            cmd!("build")
                .short_name("b")
                .about("编译项目")
                .action(Arg::Bool(&|_x| {
                    println!("command \"run\"{:?}", _x);
                })),
        )
        .add_command(
            cmd!("empty")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::Empty(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_command(
            cmd!("number")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::Number(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_command(
            cmd!("vecnumber")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::NumberMutiple(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_command(
            cmd!("vecbool")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::BoolMutiple(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_command(
            cmd!("vecstring")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::StringMutiple(&|_x| {
                    print!("testing arg_zero");
                })),
        )
        .add_command(
            cmd!("repl")
                .about("用来测试 ArgCount::Repl(_) ")
                .action(Arg::Dialog(
                    &(|r| {
                        let items = vec!["one", "two", "tree", "four"];

                        let _req_bool = r.number("你要吃几个汉堡包?").unwrap();
                        let _你要吃几个汉堡包 = r.number_multiple("多个 number").unwrap();
                        let _多个_number = r.string("string").unwrap();
                        let _string = r.string_multiple("string_multiple").unwrap();
                        let _string_multiple = r.yes_or_no("bool").unwrap();
                        let _path = r.path("path").unwrap();
                        let _path_multiple = r.path_multiple("path").unwrap();
                        let _seleted = r.select("selete", &items).unwrap();
                        let _seleted = r.select_multiple("selete", &items).unwrap();
                        
                    }),
                )),
        )
        // asdfsdaf
        ;

    // let re = app.debug_duplicate_names_check();
    // match re {
    //     Ok(_) => {}
    //     Err(s) => println!("这些命令名称重复了: ··{:?}", s),
    // }

    let _asdf = app
        .deubug_run(["cmd", "-e"])
        .deubug_run(["cmd", "help"])
        .deubug_run(["cmd", "-h"])
        .deubug_run(["cmd", "b"])
        .deubug_run(["cmd", "build", "true"])
        .deubug_run(["cmd", "build", "-h"])
        .deubug_run(["cmd", "build", "-e"])
        .deubug_run(["cmd", "run"])
        .deubug_run(["cmd", "run", "3"])
        .deubug_run(["cmd", "run", "3", "32"]) // 类型正确, 数量不正确
        .deubug_run(["cmd", "run", "-h"])
        .deubug_run(["cmd", "-h"])
        .deubug_run(["cmd"])
        .deubug_run(["cmd", "arg_one", "-h"]);
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
    println!("{:?}", Arg::Empty(&|_| {}));
}

#[test]
fn dsafdsaf() {
    let app = App::new()
        // .app_default_action(&|| { /* action */ })
        .about("这个程序主要是为了测试我写的 cmd crate")
        // .author("chen bao")
        .app_name("app")
        .version_message("0.0.1")
        .add_command(
            cmd!("-")
                .short_name("b")
                .about("编译项目")
                .action(Arg::String(
                    &(|_x| {
                        println!("command \"run\"{:?}", _x);
                    }),
                )),
        )
        .add_command(
            cmd!("build")
                .short_name("b")
                .about("编译项目")
                .action(Arg::Bool(
                    &(|_x| {
                        println!("command \"run\"{:?}", _x);
                    }),
                )),
        )
        .add_command(
            cmd!("empty")
                .about("用来测试 ArgCount::Zero ")
                // .add_example("app arg_zero  ", "")
                .action(Arg::Empty(
                    &(|_x| {
                        print!("testing arg_zero");
                    }),
                )),
        )
        .add_command(
            cmd!("number")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::Number(
                    &(|_x| {
                        print!("testing arg_zero");
                    }),
                )),
        )
        .add_command(
            cmd!("vecnumber")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::NumberMutiple(
                    &(|_x| {
                        println!("testing vec number {:?}", _x);
                    }),
                )),
        )
        .add_command(
            SubCommand::create_an_sub_command("vecbool")
                .about("用来测试 ArgCount::Zero ")
                .action(Arg::BoolMutiple(
                    &(|_x| {
                        println!("testing vecbool {:?}", _x);
                    }),
                )),
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
                .action(Arg::Bool(
                    &(|_x| {
                        println!("command \"run\"{:?}", _x);
                    }),
                )),
        )
        .add_command(
            SubCommand::create_an_sub_command("vecstring")
                .short_name("vs")
                .about("用来测试 vecstring ")
                // .add_example("command", "description")
                .help_document("str")
                .action(Arg::StringMutiple(
                    &(|_x| {
                        println!("testing vecstring {:?}", _x);
                    }),
                )),
        )
        .deubug_run(["cmd", "-e"])
        .deubug_run([""])
        .deubug_run(["cmd", "help"])
        .deubug_run(["cmd", "-h"])
        .deubug_run(["cmd", "b"])
        .deubug_run(["cmd", "build", "true"])
        .deubug_run(["cmd", "build", "-h"])
        .deubug_run(["cmd", "build", "-e"])
        .deubug_run(["cmd", "run"])
        .deubug_run(["cmd", "run", "3"])
        .deubug_run(["cmd", "run", "3", "32"]) // 类型正确, 数量不正确
        .deubug_run(["cmd", "run", "-h"])
        .deubug_run(["cmd", "-h"])
        .deubug_run(["cmd"])
        .deubug_run(["cmd", "arg_one", "-h"]);

    let _ = app
        .deubug_run(["app_name", "-e"])
        .deubug_run( ["app_name", "help"])
        .deubug_run( ["app_name", "-h"])
        .deubug_run( ["app_name", "b"])
        .deubug_run( ["app_name", "build"])
        .deubug_run( ["app_name", "build", "-h"])
        .deubug_run( ["app_name", "build", "-e"])
        .deubug_run( ["app_name", "run"])
        .deubug_run( ["app_name", "run", "3"])
        .deubug_run( ["app_name", "run", "3", "32"]) 
        .deubug_run( ["app_name", "run", "-h"])
        .deubug_run( ["app_name", "run", "-e"])
        .deubug_run( ["app_name", "-h"])
        .deubug_run( ["app_name"])
        .deubug_run( ["app_name", "repl"])
        .deubug_run( ["app_name", "run"])
        .deubug_run( ["app_name", "build"])
        .deubug_run( ["app_name", "empty"])
        .deubug_run( ["app_name", "number"])
        .deubug_run( ["app_name", "vecnumber"])
        .deubug_run( ["app_name", "vecbool"])
        .deubug_run( ["app_name", "vecstring"])
        .deubug_run( ["app_name", "repl"])
        .deubug_run(["app_name", "--list-all-commands"]);
}
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn maasdfasfin() {
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{},{},{}", one, two, c.get());

    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);
    let _c = b.clone();
}
