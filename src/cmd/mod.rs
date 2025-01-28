use std::{env, iter::repeat, vec};

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

struct App {
    /// 此程序的名称
    pub app_name: String,

    /// 一句话介绍此程序.
    pub about: &'static str,

    pub examples: Vec<String>,

    /// 此程序的作者
    pub author: &'static str,

    /// 当用户查询此程序的 version 信息时显示的信息;
    pub app_version_message: String,

    /// 此程序的帮助文档,
    ///
    pub help_message: String,

    _commands: Vec<Command>,

    /// env::args().collect()
    pub env_arg: Vec<String>,

    /// 子命令的 “参数”
    pub command_arg: Vec<String>,

    /// 只输入了程序名称没有子命令也没有任何 flag 时之行的 action.
    _app_default_action: AppDefaultAction,
}

enum AppDefaultAction {
    PrintHelpMessage,
    RunAction(Box<dyn Fn() -> ()>),
}

impl App {
    pub fn new(app_name: &str) -> App {
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
            app_version_message: "0.0.1".to_string(),
            help_message: "".to_string(),

            env_arg: env::args().collect(),
            command_arg: cmd_arg,
            _commands: vec![],
            _app_default_action: AppDefaultAction::PrintHelpMessage,
            app_name: app_name.to_string(),
            examples: vec![], // _app_default_action == None 时会调用: self.print_help_message();
        };
    }
    pub fn app_name(self, app_name: String) -> Self {
        let mut re = self;
        re.app_name = app_name;
        return re;
    }
    pub fn about(self, about: &'static str) -> Self {
        let mut re = self;
        re.about = about;
        return re;
    }

    /// 当用户查询此程序的 version 信息时显示的信息;
    pub fn app_version_message(self, version: String) -> Self {
        let mut re = self;
        re.app_version_message = version;
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

    pub fn examples(self, arr: Vec<String>) -> Self {
        let mut re = self;

        re.examples = arr;
        return re;
    }
    pub fn run(self) -> DidHandled {
        // 处理只输入了程序名称没有子命令也没有任何 flag 的情况.
        if self.env_arg.len() == 1 {
            match &self._app_default_action {
                AppDefaultAction::PrintHelpMessage => {
                    (&self).print_app_help();
                    return DidHandled::Handled;
                }
                AppDefaultAction::RunAction(f) => {
                    f();

                    return DidHandled::Handled;
                }
            }
        }

        // 处理 App 的flags.
        // -h --help -v -version
        // if self.env_arg.len() == 2 {
        //     if let Some(flag) = self.command_arg.first() {
        //         let first_arg = flag;

        //         if first_arg == "v"
        //             || first_arg == "version"
        //             || first_arg == "-v"
        //             || first_arg == "--version"
        //         {
        //             println!("{}", self.app_version_message);
        //             return Sdafadsf::Handled;
        //         }
        //     }
        // }

        let re = heldle_app_version(&self);
        match re {
            DidHandled::Handled => return re,
            DidHandled::Failed(_x) => {
                // do nothing and continue.
                if is_debug_mode() {
                    println!("{}", _x);
                }
            }
        }
        // 处理子命令
        let command_name = self.env_arg[1].clone();

        let re = handle_app_help(&self);
        match re {
            DidHandled::Handled => return re,
            DidHandled::Failed(_x) => {
                // do nothing and continue.
            }
        }

        for x in &self._commands {
            if command_name == x.command_name || command_name == x.short_name {
                let cmd_args = self.command_arg;

                println!("self.env_arg.len() {}", self.env_arg.len());
                if let Some(flag) = cmd_args.first() {
                    let first_arg = flag;

                    // println!("first_arg {}",first_arg);

                    // 有必要提供默认实现么?
                    // 先不提供默认实现.

                    if first_arg == "--help" || first_arg == "-h" {
                        // 打印 command 的帮助信息.
                        x.print_command_help(self.app_name);
                        println!("    x.print_help_message(self.app_name);");
                        // println!("{}", x.help_document);
                        return DidHandled::Handled;
                    }
                }

                // 检查参数的数量是否是需要的参数数量.
                if let Some((arg_count, f)) = &x.action {
                    match arg_count.check(&cmd_args) {
                        DidHandled::Handled => {
                            // 参数的数量符合要求.
                            f(cmd_args);

                            return DidHandled::Handled;
                        }
                        DidHandled::Failed(message) => return DidHandled::Failed(message),
                    };
                } else {
                    return DidHandled::Failed("还没有为此命令设置 action".to_string());
                }
            }
        }

        // 错误处理

        // 一个命令都没匹配到.
        return DidHandled::Failed(format!("未知命令: {:?}", self.env_arg));

        fn heldle_app_version(app: &App) -> DidHandled {
            // 处理 App 的flags.
            // -h --help -v -version
            let command_name = app.env_arg[1].clone();

            if command_name == "v"
                || command_name == "version"
                || command_name == "-v"
                || command_name == "--version"
            {
                println!("{}", app.app_version_message);

                return DidHandled::Handled;
            } else {
                return DidHandled::Failed("不是 version 命令".to_string());
            }
        }

        /// 处理 app help 的默认实现;  
        /// // -h --help -v -version
        fn handle_app_help(app: &App) -> DidHandled {
            let command_name = app.env_arg[1].clone();

            if !(command_name == "help"
                || command_name == "h"
                || command_name == "-h"
                || command_name == "--help")
            {
                return DidHandled::Failed("不是程序的 help 命令".to_string());
            }

            //  "help" 命令 的默认实现, 这里处理的是: 是用 help 命令查询其他命令.
            // 比如 `app help run` 查询 run 命令的帮助文档. 效果等同于 `app run --help`
            if let Some(需要查心的命令名称) = app.command_arg.first() {
                if 需要查心的命令名称 == "help"
                    || 需要查心的命令名称 == "h"
                    || 需要查心的命令名称 == "-h"
                    || 需要查心的命令名称 == "--help"
                {
                    // 命令 ‘help' 的帮助文档
                    //TODO:
                    println!("命令 ‘help' 的帮助文档");

                    return DidHandled::Handled;
                }

                for x in &app._commands {
                    if 需要查心的命令名称 == x.command_name || 需要查心的命令名称 == x.short_name
                    {
                        x.print_command_help(app.app_name.clone());
                        return DidHandled::Handled;
                    }
                }
                return DidHandled::Failed("查询的命令不存在".to_string());
            } else {
                // 打印 App 的帮助信息.
                app.print_app_help();
                return DidHandled::Handled;
            }
        }
    }

    pub fn print_app_help(&self) {
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
                let command_name = x.command_name;

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
            version = self.app_version_message,
        );
        print!("{}", message);
    }
}

enum DidHandled {
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

// #[derive(Debug)]
pub struct Command {
    /// 命令名   
    /// 命令的名称长度最好不要超过 20 个字符.
    command_name: &'static str,

    /// 命令名的简写形式, 通常是一个字符  
    short_name: &'static str,

    /// 一句话介绍此命令
    about: &'static str,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    help_document: &'static str,

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
            command_name: name,
            about: "",
            help_document: "",
            action: None,
            // arg_count: ArgCount::Zero,
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
    pub fn help_document(self, str: &'static str) -> Self {
        let mut re = self;
        re.help_document = str;
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

    pub fn print_command_help(&self, app_name: String) {
        println!("command  print_help_message");
        if self.help_document != "" {
            // 自定义了帮助文档的情况;
            println!("{}", self.help_document);
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg: String = if let Some((arg_count, _)) = &self.action {
                match arg_count {
                    ArgCount::Zero => "".to_string(),
                    ArgCount::One => "argument -- one argument".to_string(),
                    ArgCount::ZeroOrOne => "[argument] -- zore or one argument".to_string(),
                    ArgCount::ZoreOrMany => "[arguments...] -- zore or many argument".to_string(),
                    ArgCount::OneOrMany => "<arguments...> -- zore or many argument".to_string(),
                    ArgCount::Count(count) => {
                        let mut i = 0;
                        let mut re: String = "".to_string();
                        while i < *count {
                            re.push_str(" argument");
                            i += 1;
                        }

                        re
                    }
                }
            } else {
                "".to_string()
            };

            let message = format!(
                r#"
{about}
Usage:
    {app_name} {command_name} {arg}
version: {command_name}

commands:
{short_name}
"#,
                about = self.about,
                command_name = self.command_name,
                short_name = self.short_name,
            );
            print!("{}", message);
        }
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
