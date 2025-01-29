use super::*;

pub enum AppDefaultAction {
    PrintHelpMessage,
    RunAction(Box<dyn Fn() -> ()>),
}

pub struct App {
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

impl App {
    // -------- Public Part --------

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
        if let Some(command_name) = self.env_arg.get(1) {
            {
                let re = self._heldle_app_version();
                match re {
                    DidHandled::Handled => return re,
                    DidHandled::Failed(_x) => {
                        /*  do nothing and continue. */
                        // if is_debug_mode() {
                        //     println!("{}", _x)
                        // }
                    }
                }
            }

            {
                let re = self._handle_app_help();
                match re {
                    DidHandled::Handled => return re,
                    DidHandled::Failed(_x) => { /*  do nothing and continue. */ }
                }
            }

            {
                let re = self._handle_commands(command_name);
                match re {
                    DidHandled::Handled => return re,
                    DidHandled::Failed(_x) => { /*  do nothing and continue. */ }
                }
            }
            // for x in &self._commands {
            //     if command_name == x.command_name || command_name == x.short_name {
            //         let cmd_args = self.command_arg;

            //         println!("self.env_arg.len() {}", self.env_arg.len());
            //         if let Some(flag) = cmd_args.first() {
            //             let first_arg = flag;

            //             // println!("first_arg {}",first_arg);

            //             // 有必要提供默认实现么?
            //             // 先不提供默认实现.

            //             if first_arg == "--help" || first_arg == "-h" {
            //                 // 打印 command 的帮助信息.
            //                 x.print_command_help(self.app_name);
            //                 println!("    x.print_help_message(self.app_name);");
            //                 // println!("{}", x.help_document);
            //                 return DidHandled::Handled;
            //             }
            //         }

            //         // 检查参数的数量是否是需要的参数数量.
            //         if let Some((arg_count, f)) = &x.action {
            //             match arg_count.check(&cmd_args) {
            //                 DidHandled::Handled => {
            //                     // 参数的数量符合要求.
            //                     f(cmd_args);

            //                     return DidHandled::Handled;
            //                 }
            //                 DidHandled::Failed(message) => return DidHandled::Failed(message),
            //             };
            //         } else {
            //             return DidHandled::Failed("还没有为此命令设置 action".to_string());
            //         }
            //     }
            // }
        } else {
            //只输入了程序名称没有子命令也没有任何 flag

            let re = self._handle_app_default_acton();
            match re {
                DidHandled::Handled => return re,
                DidHandled::Failed(_x) => { /*  do nothing and continue. */ }
            }
        }

        // 错误处理

        // 一个命令都没匹配到.
        return DidHandled::Failed(format!("未知命令: {:?}", self.env_arg));
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

    pub fn print_app_examples() {}

    // -------- Private Part --------

    /// app help 的默认实现;  
    /// // -h --help -v -version
    fn _handle_app_help(&self) -> DidHandled {
        let command_name = self.env_arg[1].clone();

        if !(command_name == "help"
            || command_name == "h"
            || command_name == "-h"
            || command_name == "--help")
        {
            return DidHandled::Failed("不是程序的 help 命令".to_string());
        }

        //  "help" 命令 的默认实现, 这里处理的是: 是用 help 命令查询其他命令.
        // 比如 `app help run` 查询 run 命令的帮助文档. 效果等同于 `app run --help`
        if let Some(需要查询的命令名称) = self.command_arg.first() {
            if 需要查询的命令名称 == "help"
                || 需要查询的命令名称 == "h"
                || 需要查询的命令名称 == "-h"
                || 需要查询的命令名称 == "--help"
            {
                // 命令 ‘help' 的帮助文档
                //TODO:
                println!("命令 ‘help' 的帮助文档");

                return DidHandled::Handled;
            }

            for x in &self._commands {
                if 需要查询的命令名称 == x.command_name || 需要查询的命令名称 == x.short_name
                {
                    x.print_command_help(self.app_name.clone());
                    return DidHandled::Handled;
                }
            }
            return DidHandled::Failed("查询的命令不存在".to_string());
        } else {
            // 打印 App 的帮助信息.
            self.print_app_help();
            return DidHandled::Handled;
        }
    }

    /// app version 命令的默认实现
    fn _heldle_app_version(&self) -> DidHandled {
        // 处理 App 的flags.
        // -h --help -v -version
        let command_name = self.env_arg[1].clone();

        if command_name == "v"
            || command_name == "version"
            || command_name == "-v"
            || command_name == "--version"
        {
            println!("{}", self.app_version_message);

            return DidHandled::Handled;
        } else {
            return DidHandled::Failed("不是 version 命令".to_string());
        }
    }

    /// 处理只输入了程序名称没有子命令也没有任何 flag 的情况.
    fn _handle_app_default_acton(&self) -> DidHandled {
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
        };
        return DidHandled::Failed("()".to_string());
    }

    fn _handle_commands(&self, command_name: &String) -> DidHandled {
        for x in &self._commands {
            if command_name == x.command_name || command_name == x.short_name {
                let cmd_args = &self.command_arg;

                // println!("self.env_arg.len() {}", self.env_arg.len());
                if let Some(first_arg) = cmd_args.first() {
                    // 有必要提供默认实现么?
                    // 先不提供默认实现.

                    if first_arg == "--help" || first_arg == "-h" {
                        // 打印 command 的帮助信息.
                        x.print_command_help(self.app_name.clone());
                        if is_debug_mode() {
                            println!("    x.print_help_message(self.app_name);");
                        }

                        return DidHandled::Handled;
                    }
                }

                // 检查参数的数量是否是需要的参数数量.
                if let Some((arg_count, f)) = &x.action {
                    match arg_count.check(&cmd_args) {
                        DidHandled::Handled => {
                            // 参数的数量符合要求.
                            f(cmd_args.clone());

                            return DidHandled::Handled;
                        }
                        DidHandled::Failed(message) => return DidHandled::Failed(message),
                    };
                } else {
                    return DidHandled::Failed("还没有为此命令设置 action".to_string());
                }
            }
        }

        return DidHandled::Handled;
    }

    fn _handle_app_example(&self) -> DidHandled {
        let command_name = self.env_arg[1].clone();

        if !(command_name == "help"
            || command_name == "h"
            || command_name == "-h"
            || command_name == "--help")
        {
            return DidHandled::Failed("不是程序的 help 命令".to_string());
        }

        //  "help" 命令 的默认实现, 这里处理的是: 是用 help 命令查询其他命令.
        // 比如 `app help run` 查询 run 命令的帮助文档. 效果等同于 `app run --help`
        if let Some(需要示例的命令名称) = self.command_arg.first() {
            if 需要示例的命令名称 == "example"
                || 需要示例的命令名称 == "e"
                || 需要示例的命令名称 == "-e"
                || 需要示例的命令名称 == "--example"
            {
                // 命令 ‘help' 的帮助文档
                //TODO:
                println!("命令 example' 的帮助文档");

                return DidHandled::Handled;
            }

            for x in &self._commands {
                if 需要示例的命令名称 == x.command_name || 需要示例的命令名称 == x.short_name
                {
                    x.print_command_help(self.app_name.clone());
                    return DidHandled::Handled;
                }
            }
            return DidHandled::Failed("查询的命令不存在".to_string());
        } else {
            // 打印 App 的帮助信息.
            self.print_app_help();
            return DidHandled::Handled;
        }
    }
}
