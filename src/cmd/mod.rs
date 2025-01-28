use std::{env, vec};

pub fn hello() {
    println!("--------hello--------");
    let re = App::new()
        .about("这个程序主要是为了测试我写的 cmd crate")
        .author("chen bao")
        .version("0.0.1".to_string())
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
        Sdafadsf::Success => {
            return; // runs perfact.
        }
        Sdafadsf::Failed(err_message) => {
            print!("{}\n", err_message);
            return;
        }
    }
}

pub fn test_app_default_action() {
    println!("--------test_app_default_action--------");
    let re = App::new()
        .app_default_action(|| {
            print!("{:?}", "_x");
        })
        .run();
    match re {
        Sdafadsf::Success => {
            return; // runs perfact.
        }
        Sdafadsf::Failed(err_message) => {
            print!("{}\n", err_message);
            return;
        }
    }
}

struct App {
    pub about: &'static str,
    pub author: &'static str,
    pub version: String,

    /// 此程序的帮助文档,
    pub help_message: String,

    _commands: Vec<Command>,

    /// env::args().collect()
    pub env_arg: Vec<String>,

    /// 子命令的 “参数”
    pub command_arg: Vec<String>,

    /// 只输入了程序名称没有子命令也没有任何 flag 时之行的 action.
    /// 如果 _app_default_action is None, 则会调用: self.print_help_message()
    _app_default_action: AppDefaultAction,
    // _app_default_action: Option<(ArgCount, CommandAction)>,
}

enum AppDefaultAction {
    PrintHelpMessage,
    RunAction(Box<dyn Fn() -> ()>),
}

impl App {
    pub fn new() -> App {
        let env_arg: Vec<String> = env::args().collect();

        // 第 2 个一级后面的所有.
        let cmd_arg: Vec<String> = if env_arg.len() > 2 {
            env_arg[2..].to_vec()
        } else {
            vec![]
        };

        return App {
            about: "",
            author: "",
            version: "0.0.1".to_string(),
            help_message: "".to_string(),

            env_arg: env::args().collect(),
            command_arg: cmd_arg,
            _commands: vec![],
            _app_default_action: AppDefaultAction::PrintHelpMessage, // _app_default_action == None 时会调用: self.print_help_message();
        };
    }

    pub fn about(self, about: &'static str) -> Self {
        let mut re = self;
        re.about = about;
        return re;
    }

    pub fn version(self, version: String) -> Self {
        let mut re = self;
        re.version = version;
        return re;
    }

    pub fn author(self, author: &'static str) -> Self {
        let mut re = self;
        re.author = author;
        return re;
    }

    /// 设置只有 程序名, 没有任何子命令也没有任何参数时执行的 action.
    pub fn app_default_action<F>(self, action: F) -> Self
    where
        F: Fn() -> () + 'static,
    {
        let mut re = self;
        re._app_default_action = AppDefaultAction::RunAction(Box::new(action));
        return re;
    }

    pub fn add_command(self, command: Command) -> Self {
        let mut re = self;

        re._commands.push(command);

        return re;
    }

    /// 自定义帮助信息.
    /// 如果不自定义钢珠信息, 则会使用自动生成的帮助信息.
    pub fn help_message(self, message: String) -> Self {
        let mut re = self;
        re.help_message = message;

        return re;
    }

    pub fn run(self) -> Sdafadsf {
        // 处理只输入了程序名称没有子命令也没有任何 flag 的情况.
        if self.env_arg.len() == 1 {
            match &self._app_default_action {
                AppDefaultAction::PrintHelpMessage => {
                    (&self).print_help_message();
                    return Sdafadsf::Success;
                }
                AppDefaultAction::RunAction(f) => {
                    f();

                    return Sdafadsf::Success;
                }
            }
        }

        // if self.env_arg.len() == 2 {
        //     let first_arg = self.env_arg[1].clone();
        // 处理 App 的flags.
        // -h --help -v -version
        //
        // 有必要提供默认实现么?
        // 先不提供默认实现.
        // if first_arg.starts_with('-') {
        //     if first_arg == "-h" || first_arg == "--help" {}

        //     if first_arg == "-v" || first_arg == "--version" {}
        // }
        // }

        // let cmd = self.env_arg[1].clone();
        // 处理子命令
        // self._commands.iter().for_each(|x| {
        //     if cmd == x.name || cmd == x.short_name {
        //         if let Some(f) = &x.action {
        //             f(&self.command_arg)
        //         } else if is_debug_mode() {
        //             print!(
        //                 "DEBUG MESSAGE: x.action 是 None. \n{}:{} \n",
        //                 file!(),
        //                 line!()
        //             );
        //         }
        //     }
        // });

        let cmd = self.env_arg[1].clone();

        // 处理子命令
        for x in &self._commands {
            if cmd == x.name || cmd == x.short_name {
                if let Some((arg_count, f)) = &x.action {
                    let cmd_args = self.command_arg;
                    match arg_count.check(&cmd_args) {
                        Sdafadsf::Success => {
                            f(cmd_args);

                            return Sdafadsf::Success;
                        }
                        Sdafadsf::Failed(message) => return Sdafadsf::Failed(message),
                    };
                } else {
                    return Sdafadsf::Failed("还没有为此命令设置 action".to_string());
                }
            }
        }

        // 错误处理

        // 一个命令都没匹配到.

        return Sdafadsf::Failed(format!("未知命令: {:?}", self.env_arg));
    }

    pub fn print_help_message(&self) {
        if self.help_message.trim() != "" {
            print!("{}", self.help_message);
            return;
        }

        let all_commands_about: String = self
            ._commands
            .iter()
            .map(|x| {
                let short_name = if x.short_name == "" {
                    "".to_string()
                } else {
                    ", ".to_string() + x.short_name
                };
                let command_name = x.name;

                // TODO: 为 cmd_name 添加颜色.
                let cmd_name = command_name.to_string() + &short_name;

                // TODO:  可以考虑使用 pretty Table 来美化输出
                format!("    {cmd_name}\t\t\t{about}\n", about = x.about,)
            })
            .collect();

        let message = format!(
            r#"
{about}
author: {author}
version: {version}

commands:
{all_commands_about}
"#,
            about = self.about,
            author = self.author,
            version = self.version,
        );
        print!("{}", message);
    }
}

enum Sdafadsf {
    Success,
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
    fn check(&self, cmd_args: &Vec<String>) -> Sdafadsf {
        let need_arg_count = self;
        let real_atgs_count = cmd_args.len();
        match need_arg_count {
            ArgCount::Zero => {
                if real_atgs_count != 0 {
                    return Sdafadsf::Failed(format!(
                        "需要的参数数量是 0 个, 实际是 {} 个: {:?}\n",
                        real_atgs_count, cmd_args,
                    ));
                }
            }
            ArgCount::One => {
                if real_atgs_count != 1 {
                    return Sdafadsf::Failed(format!(
                        "需要的参数数量是 1 个, 实际是 {} 个\n",
                        real_atgs_count
                    ));
                }
            }
            ArgCount::ZeroOrOne => {
                if real_atgs_count == 0 || real_atgs_count == 1 {
                    return Sdafadsf::Failed(format!(
                        "需要的参数数量是 0 个 或者 1 个参数, 实际是 {} 个\n",
                        real_atgs_count
                    ));
                }
            }
            ArgCount::ZoreOrMany => {}
            ArgCount::OneOrMany => {
                if real_atgs_count == 0 {
                    return Sdafadsf::Failed(format!(
                        "至少需要一个参数, 实际是 {} 个\n",
                        real_atgs_count
                    ));
                }
            }
            ArgCount::Count(count) => {
                if real_atgs_count == *count as usize {
                    return Sdafadsf::Failed(format!(
                        "至少需要 {} 个参数, 实际是 {} 个\n",
                        count, real_atgs_count,
                    ));
                }
            }
        };

        return Sdafadsf::Success;
    }
}

// #[derive(Debug)]
pub struct Command {
    /// 命令的名称长度最好不要超过 20 个字符.
    name: &'static str,

    ///
    short_name: &'static str,

    /// 参数的数量,
    /// 如果没有参数, arg_count = ArgCount::Zero.
    arg_count: ArgCount,

    about: &'static str,

    /// 此命令的帮助文档.
    help_message: &'static str,

    /// command action with command_arg
    action: Option<(ArgCount, CommandAction)>,
}

type CommandAction = Box<dyn Fn(Vec<String>) -> ()>;

impl Command {
    /// Creates a new [`Command`].
    /// name:
    pub fn new(name: &'static str) -> Self {
        if is_debug_mode() && name == "" {
            eprintln!("WARNING: name 不能是空字符串 \"\", name 的值至少需要一个字符.");
        }

        return Command {
            name: name,
            about: "",
            help_message: "",
            action: None,
            arg_count: ArgCount::Zero,
            short_name: "",
        };
    }

    /// set `Command.short_name`
    pub fn short_name(self, short_name: &'static str) -> Self {
        let mut re = self;
        re.short_name = short_name;
        return re;
    }

    /// set `Command. about`
    pub fn about(self, about: &'static str) -> Self {
        let mut re = self;
        re.about = about;
        return re;
    }

    /// set `Command.example`
    pub fn example(self, str: &'static str) -> Self {
        let mut re = self;
        re.help_message = str;
        return re;
    }

    /// set `Command.action`
    pub fn action<F>(self, arg_count: ArgCount, action: F) -> Self
    where
        F: Fn(Vec<String>) -> () + 'static,
    {
        let mut re = self;
        re.action = Some((arg_count, Box::new(action)));

        return re;
    }

    // /// set `Command.arg_count`
    // pub fn arg_count(self, arg_count: ArgCount) -> Self {
    //     let mut re = self;
    //     re.arg_count = arg_count;
    //     return re;
    // }
}

fn is_debug_mode() -> bool {
    return cfg!(debug_assertions);
}

// 我有这样一个函数:    pub fn new(name: &'static str) -> Self, 如何在编译时检查 name 的长度, 如果 name 的长度是 0, 则编译错误?
