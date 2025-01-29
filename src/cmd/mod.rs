use std::{env, vec};

mod application;
pub use application::*;

mod command;
pub use command::*;

pub fn hello() {
    println!("--------hello--------");
    let re = App::new("cmd")
        .about("这个程序主要是为了测试我写的 cmd crate")
        .author("chen bao")
        .app_version_message("0.0.1".to_string())
        .add_command(
            Command::new("run")
                .about("运行程序")
                .action(ArgCount::Zero, |_x| {
                    print!("command \"run\"\n");
                }),
        )
        .add_command(
            Command::new("build")
                .short_name("b")
                .about("编译项目")
                .action(ArgCount::Zero, |x| {
                    print!("command \"build\"{:?}\n", x);
                }),
        )
        .add_command(Command::new("update").short_name("u"))
        .run();

    match re {
        DidHandled::Handled => {
            return; // runs perfact.
        }
        DidHandled::Failed(err_message) => {
            print!("{}\n", err_message);
            return;
        }
    }
}

pub fn test_app_default_action() {
    println!("--------test_app_default_action--------");
    let re = App::new("cmd")
        .app_default_action(|| {
            print!("{:?}", "_x");
        })
        .run();
    match re {
        DidHandled::Handled => {
            return; // runs perfact.
        }
        DidHandled::Failed(err_message) => {
            print!("{}\n", err_message);
            return;
        }
    }
}

#[derive(Clone, Debug)]
pub enum DidHandled {
    /// 表示匹配到了相关命令并正确执行了相关 action.
    Handled,

    /// 没匹配到相关命令或者其他错误.
    Failed(String),
}

#[derive(Clone, Copy, Debug)]
pub enum ArgCount {
    /// 没有参数
    Zero,

    /// 能且只能填写一个参数.
    One,

    /// 参数可以不填写
    ZeroOrOne,

    /// 参数可以不填写, 也可以填入一个或者很多个.
    ZoreOrMany,

    /// 可以填入一个或者很多个.
    OneOrMany,

    /// 填入指定数量的参数.
    Count(u8), // 严格限定数量这种做法太少见, 先不提供这个功能.
}

impl ArgCount {
    fn check(&self, cmd_args: &Vec<String>) -> DidHandled {
        let need_arg_count = self;
        let real_atgs_count = cmd_args.len();
        match need_arg_count {
            ArgCount::Zero => {
                if real_atgs_count != 0 {
                    return DidHandled::Failed(format!(
                        "需要的参数数量是 0 个, 实际是 {} 个: {:?}\n",
                        real_atgs_count, cmd_args,
                    ));
                }
            }
            ArgCount::One => {
                if real_atgs_count != 1 {
                    return DidHandled::Failed(format!(
                        "需要的参数数量是 1 个, 实际是 {} 个\n",
                        real_atgs_count
                    ));
                }
            }
            ArgCount::ZeroOrOne => {
                if real_atgs_count == 0 || real_atgs_count == 1 {
                    return DidHandled::Failed(format!(
                        "需要的参数数量是 0 个 或者 1 个参数, 实际是 {} 个\n",
                        real_atgs_count
                    ));
                }
            }
            ArgCount::ZoreOrMany => {}
            ArgCount::OneOrMany => {
                if real_atgs_count == 0 {
                    return DidHandled::Failed(format!(
                        "至少需要一个参数, 实际是 {} 个\n",
                        real_atgs_count
                    ));
                }
            }
            ArgCount::Count(count) => {
                if real_atgs_count == *count as usize {
                    return DidHandled::Failed(format!(
                        "至少需要 {} 个参数, 实际是 {} 个\n",
                        count, real_atgs_count,
                    ));
                }
            }
        };

        return DidHandled::Handled;
    }
}

fn is_debug_mode() -> bool {
    return cfg!(debug_assertions);
}

fn debug_run<F>(f: F)
where
    F: Fn() -> (),
{
    if cfg!(debug_assertions) {
        f()
    }
}
