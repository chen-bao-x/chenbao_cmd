use crate::{examples_types::Examples, helper::*};

use super::*;
use owo_colors::OwoColorize;
use prettytable::{row, table};

#[derive(Clone)]
pub enum AppDefaultAction {
    /// 打印 app 的帮助文档
    PrintHelpMessage,

    /// 如果想读取命令行参数, 请使用:   `let env_arg: Vec<String> = env::args().collect();`
    CustomAction(&'static dyn Fn() -> ()),
}

impl Default for AppDefaultAction {
    fn default() -> Self {
        Self::PrintHelpMessage
    }
}

/// 用于表示拥护输入的子命令和参数是否被正确解析.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DidHandled {
    /// 表示匹配到了相关命令并正确执行了相关 action.
    Handled,

    /// 没匹配到相关命令或者其他错误.
    Failed(String),
}

/// 用于创建命令行程序并设置程序的 子命令.
///
/// # 示例:  
/// ```rust
///    use std::rc::Rc;
///    use chenbao_cmd::*;
///    let app = App::new("cmd")
///        .about("这个程序主要是为了测试我写的 cmd crate")
///        .author("chen bao")
///        .version_message("0.0.1".to_string())
///        .add_subcommand(
///            SubCommand::new("run")
///                .about("运行程序")
///                .action(ArgAction::Empty(Rc::new(|| {
///                    print!(r#"runing commmand: "run""#);
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("help")
///                .about("运行程序")
///                .action(ArgAction::Empty(Rc::new(|| {}))),
///        )
///        .add_subcommand(
///            SubCommand::new("build")
///                .short_name("b")
///                .about("编译项目")
///                .action(ArgAction::Bool(Rc::new(|_x| {
///                    print!("command \"run\"{:?}\n", _x);
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("empty")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::Empty(Rc::new(|| {
///                    print!("testing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("number")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::Number(Rc::new(|_x| {
///                    print!("testing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("vecnumber")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::NumberMutiple(Rc::new(|_x| {
///                    print!("testing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("vecbool")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::BoolMutiple(Rc::new(|_x| {
///                    print!("testing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("vecstring")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::StringMutiple(Rc::new(|_x| {
///                    print!("testing arg_zero");
///                }))),
///        );
///    app.run();
/// ```
#[derive(Clone)]
pub struct App {
    /// 此程序的名称;  
    /// 所有自动生成的帮助文档和示例都会使用到 {app_name}
    pub _app_name: String,

    /// 一句话介绍此程序.
    pub _about: &'static str,

    /// 此程序的作者
    pub _author: &'static str,

    /// 当用户查询此程序的 version 信息时显示的信息;
    pub _app_version_message: String,

    /// 此程序的帮助文档,
    pub _help_message: String,

    /// 此 app 的所有子命令.
    pub _commands: Vec<Cmd>,

    /// `let env_arg: Vec<String> = std::env::args().collect()`
    pub _env_arg: Vec<String>,

    /// 使用此程序的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub _examples: Examples,

    /// 子命令的 “参数”
    pub _commands_arg: Vec<String>,

    /// 只输入了程序名称没有子命令也没有任何 flag 时之行的 action.
    /// 默认是 AppDefaultAction::PrintHelpMessage;
    pub _app_default_action: AppDefaultAction,
}

impl App {
    // ============================
    // =        Public Part       =
    // ============================

    /// ```rust
    ///     let app = chenbao_cmd::App::new("cmd");
    ///     app.run();
    /// ```
    // pub fn new(app_name: &str) -> App {
    pub fn new() -> App {
        Self {
            ..Default::default()
        }
    }

    /// 在这里介绍这个程序是什么. 做什么用的
    /// ```rs
    /// let app = App::new().app_name(env!("CARGO_PKG_NAME"));
    /// ```
    pub fn app_name(self, app_name: &'static str) -> Self {
        let mut re = self;
        re._app_name = app_name.to_string();
        return re;
    }

    /// 在这里介绍这个程序是什么. 做什么用的
    pub fn about(self, about: &'static str) -> Self {
        let mut re = self;
        re._about = about;
        return re;
    }

    /// 使用此程序的一些示例,  
    /// 当用户使用 `app -e` 时会打印在这里添加的示例.  
    /// 此 method 可以多次调用来给此程序添加多个示例.
    pub fn add_app_example(self, command: &'static str, description: &'static str) -> Self {
        let mut re = self;

        re._examples.add_single_example(command, description);

        return re;
    }

    /// 此程序的版本信息.  
    /// 当用户使用 `app --version` 时会打印在这里添加的版本信息.  
    /// 此 method 只需要调用一次.  
    /// ```rs
    /// let app = App::new().version_message(env!("CARGO_PKG_VERSION"));
    /// ```
    pub fn version_message(self, version_message: String) -> Self {
        let mut re = self;
        re._app_version_message = version_message;
        return re;
    }

    /// 此程序的版本信息.  
    /// 当用户使用 `app --version` 时会打印在这里添加的版本信息.  
    /// 此 method 只需要调用一次.  
    pub fn author(self, author: &'static str) -> Self {
        let mut re = self;
        re._author = author;
        return re;
    }

    ///
    /// 设置只有 程序名, 没有任何子命令也没有任何参数时执行的 action.  
    /// 默认情况下是打印此程序的帮助信息.  
    /// `app_default_action` 有默认实现, 可以不用设置.
    pub fn app_default_action(self, action: &'static dyn Fn() -> ()) -> Self {
        let mut re = self;
        re._app_default_action = AppDefaultAction::CustomAction(action);
        return re;
    }

    /// add command
    /// 为此 App 添加指令
    pub fn add_command(self, command: Cmd) -> Self {
        let mut re = self;

        re._commands.push(command);

        return re;
    }

    /// 自定义帮助信息.  
    /// 此方法会替换掉由 chenbao_cmd 提供的帮助文档.
    pub fn help_message(self, message: String) -> Self {
        let mut re = self;
        re._help_message = message;

        return re;
    }

    /// ```rust
    /// let app = chenbao_cmd::App::new("cmd")
    ///     .about("在这里介绍这个程序在什么情况下能帮助用户解决什么问题.")
    ///     .author("chen bao")
    ///     .version_message("0.0.1".to_string())
    ///     .add_subcommand(
    ///         chenbao_cmd::SubCommand::new("run")
    ///          .about("运行程序")
    ///          .action(chenbao_cmd::ArgAction::Empty(std::rc::Rc::new(|| {
    ///              print!(r#"runing commmand: "run""#);
    ///          }))),
    ///     );
    ///     app.run();
    ///
    /// ```
    pub fn run(self) {
        let re = self.try_run();
        match re {
            DidHandled::Handled => {}
            DidHandled::Failed(_e) => {
                eprintln!("{}", _e)
            }
        }
    }

    /// like run(), but has result.
    pub fn try_run(self) -> DidHandled {
        #[cfg(debug_assertions)]
        {
            self.debug_check();
        }

        let option_string = self._env_arg.get(1);
        match option_string {
            None => {
                //只输入了程序名称没有子命令也没有任何 flag

                return self._handle_app_default_acton();
            }
            Some(command_name) => {
                {
                    let re = self._heldle_app_version();
                    match re {
                        DidHandled::Handled => return re,
                        DidHandled::Failed(_x) => { /* continue. */ }
                    }
                }

                {
                    let re = self._handle_app_help();
                    match re {
                        DidHandled::Handled => return re,
                        DidHandled::Failed(_x) => { /* continue. */ }
                    }
                }

                {
                    let re = self._handle_app_example();
                    match re {
                        DidHandled::Handled => return re,
                        DidHandled::Failed(_x) => { /* continue. */ }
                    }
                }

                {
                    let re = self._handle_commands(command_name);
                    match re {
                        DidHandled::Handled => return re,
                        DidHandled::Failed(_x) => {
                            /* 这是最后一个 handle 项目了, 直接返回. */
                            return DidHandled::Failed(_x);
                        }
                    }
                }
            }
        }
    }

    //  ------- Print -------

    /// 打印 App 的帮助信息.  
    /// `app -h` 时调用此函数.
    pub fn print_app_help(&self) {
        if self._help_message.trim() != "" {
            // 有自定义的帮助文档.
            print!("{}", self._help_message);
            return;
        }

        let mut table = table!();
        table.set_format(table_formater());

        for x in &self._commands {
            let short_name = if x._short_name == "" {
                "".to_string()
            } else {
                // ", ".to_string() + &x.short_name

                format!("{}{}", &x._short_name, ", ",)
            };

            let command_name = &x._name;

            // TODO: 为 cmd_name 添加颜色.
            let cmd_name = format!(
                "{}{}",
                short_name.styled_sub_command(),
                command_name.styled_sub_command(),
            );

            table.add_row(row![cmd_name, x._about]);
        }

        let all_commands_about: String = table.to_string();

        let help = format!(
            "{}, {}",
            "-h".styled_sub_command(),
            "--help".styled_sub_command()
        );
        let ver = format!(
            "{}, {}",
            "-v".styled_sub_command(),
            "--version".styled_sub_command()
        );
        let example = format!(
            "{}, {}",
            "-e".styled_sub_command(),
            "--example".styled_sub_command()
        );

        // TODO: 让打印的信息更优美.
        let flag_message = format!("{}\n    {help}\t\t显示此命令的帮助.\n    {ver}\t查看此程序的版本.\n    {example}\t查看示例.\n" , "Flags:".bright_green());

        let message = format!(
            r#"
{about}
{author}
{flag_message}
{commands}
"#,
            about = self._about,
            author = if self._author == "" {
                "".to_string()
            } else {
                format!("{} {}", "Author:", self._author)
            },
            // version = self.app_versioCn_message,
            commands = format!("{}\n{}", "Commands:".bright_green(), all_commands_about),
        );
        print!("{}", message);
    }

    /// 打印 App 的示例.  
    /// `app -e` 时调用此函数.
    pub fn print_app_examples(&self) {
        if self._examples.is_empty() {
            let mut table = table!();
            table.set_format(table_formater());

            for x in &self._commands {
                let rows = x.formated_command_example(&self._app_name);

                rows.row_iter().for_each(|a| {
                    table.add_row(a.clone());
                });
            }
            println!("{}", table);
            // table.printstd();
        } else {
            let mut table = table!();
            table.set_format(helper::table_formater());

            self._examples.pretty_formated().row_iter().for_each(|a| {
                table.add_row(a.clone());
            });

            println!("{}", table);
        }
    }
}

impl App {
    // -------- Private Part --------

    /// app help 的默认实现;  
    /// // -h --help -v -version
    fn _handle_app_help(&self) -> DidHandled {
        let command_name = &*self._env_arg[1];
        // let arr = vec!["-h", "--help", "help", "h"];
        let arr = vec!["-h", "--help"];

        if arr.contains(&command_name) {
            // 打印 App 的帮助信息.
            self.print_app_help();
            return DidHandled::Handled;
        } else {
            return DidHandled::Failed(r#"不是 "-h" or "--help""#.to_string());
        }

        // if !(command_name == "-h" || command_name == "--help") {
        //     return DidHandled::Failed("不是程序的 help 命令".to_string());
        // }

        // //  "help" 命令 的默认实现, 这里处理的是: 是用 help 命令查询其他命令.
        // // 比如 `app help run` 查询 run 命令的帮助文档. 效果等同于 `app run --help`
        // if let Some(需要查询的命令名称) = self.sub_command_arg.first() {
        //     if 需要查询的命令名称 == "-h" || 需要查询的命令名称 == "--help" {
        //         // TODO: 命令 ‘help' 的帮助文档

        //         println!("命令 ‘help' 的帮助文档");

        //         return DidHandled::Handled;
        //     }

        //     for x in &self.sub_commands {
        //         if 需要查询的命令名称 == &x.command_name || 需要查询的命令名称 == &x.short_name
        //         {
        //             x.print_command_help(&self.app_name);
        //             return DidHandled::Handled;
        //         }
        //     }
        //     return DidHandled::Failed(format!(
        //         "查询的命令 {} 不存在",
        //         需要查询的命令名称.styled_sub_command()
        //     ));
        // } else {
        //     // 打印 App 的帮助信息.
        //     self.print_app_help();
        //     return DidHandled::Handled;
        // }
    }

    /// app version 命令的默认实现
    fn _heldle_app_version(&self) -> DidHandled {
        // 处理 App 的flags.
        //  -v -version
        let command_name = self._env_arg[1].clone();

        if command_name == "-v" || command_name == "--version" {
            println!("{}", self._app_version_message);

            return DidHandled::Handled;
        } else {
            return DidHandled::Failed("不是 version 命令".to_string());
        }
    }

    // /// app version 命令的默认实现
    // fn _heldle_app_example(&self) -> DidHandled {
    //     // 处理 App 的flags.
    //     // -h --help -v -version
    //     let command_name = self.env_arg[1].clone();

    //     if command_name == "-v" || command_name == "--version" {
    //         println!("{}", self.app_version_message);

    //         return DidHandled::Handled;
    //     } else {
    //         return DidHandled::Failed(format!("不是 {} 命令", "version".styled_sub_command()));
    //     }
    // }

    /// 处理只输入了程序名称没有子命令也没有任何 flag 的情况.
    fn _handle_app_default_acton(&self) -> DidHandled {
        if self._env_arg.len() == 1 {
            match &self._app_default_action {
                AppDefaultAction::PrintHelpMessage => {
                    (&self).print_app_help();
                    return DidHandled::Handled;
                }
                AppDefaultAction::CustomAction(f) => {
                    f();

                    return DidHandled::Handled;
                }
            }
        };
        return DidHandled::Failed("有子命令或者 flag, 不是 app_default_acton".to_string());
    }

    fn _handle_commands(&self, command_name: &String) -> DidHandled {
        for x in &self._commands {
            if command_name == &x._name || command_name == &x._short_name {
                let cmd_args = &self._commands_arg;

                return x.run(&self._app_name, cmd_args);
            } else {
                continue;
            }
        }

        return DidHandled::Failed(format!(
            "未知命令: {:?}\n\n输入 {} -h 查看帮助",
            self._env_arg, self._app_name
        ));
    }

    fn _handle_app_example(&self) -> DidHandled {
        let command_name = self._env_arg[1].clone();

        if command_name == "-e" || command_name == "--example" {
            self.print_app_examples();

            return DidHandled::Handled;
        } else {
            return DidHandled::Failed("不是 version 命令".to_string());
        }
    }
}

impl App {
    //  ------- Debug Functions -------

    /// 在 debug 模式下强制使用 debug_run(env_args: Vec<&str>) 函数中的 env_args.
    /// 此函数会忽略程序真正的 env_args.
    pub fn deubg_run(self, env_args: Vec<&str>) -> Self {
        println!("------- test_run: {:?} -------", env_args);

        let mut re = self.clone();
        let env_arg: Vec<String> = env_args.iter().map(|x| x.to_string()).collect();

        // 第 2 个一级后面的所有.
        let sub_cmd_arg: Vec<String> = if env_arg.len() > 2 {
            env_arg[2..].to_vec()
        } else {
            vec![]
        };

        re._commands_arg = sub_cmd_arg;
        re._env_arg = env_arg;

        let did_handled = re.try_run();

        match did_handled {
            DidHandled::Handled => { /* runs perfact. */ }
            DidHandled::Failed(err_message) => {
                print!("{}\n", err_message);
            }
        }

        return self;
    }

    /// 检查子命令示example是否能正确的被解析
    /// 检查子命令的名字是否重复.
    /// 命令人类友好度检查
    #[cfg(debug_assertions)] // 只在 debug 模式下使用
    pub(crate) fn debug_check(&self) {
        let re = self.debug_duplicate_names_check();
        if let Err(duplicate_names) = re {
            println!("{}", "ERROR 有子命令的名称重复了:".red());

            duplicate_names.iter().for_each(|x| {
                println!("{}", x);
            });
        }
    }

    /// 检查子命令的名字是否重复.
    #[cfg(debug_assertions)] // 只在 debug 模式下使用
    pub(crate) fn debug_duplicate_names_check(
        &self,
    ) -> Result<(), std::collections::HashSet<&String>> {
        use std::collections::HashSet;

        // 重复了的子命令名称.
        let mut duplicated_names: HashSet<&String> = HashSet::new();

        // 子命令的名字合集.
        let mut set: HashSet<&String> = HashSet::new();

        let mut default_impls: HashSet<String> = HashSet::new();
        {
            // 这几个是 chenbao_cmd  自带的默认实现的 子命令和 flag, 不能被自定义.
            default_impls.insert("-h".to_string());
            default_impls.insert("--help".to_string());
            default_impls.insert("-v".to_string());
            default_impls.insert("--version".to_string());
            default_impls.insert("-e".to_string());
            default_impls.insert("--example".to_string());
        }
        for x in &self._commands {
            {
                let name = &x._name;

                if set.contains(&name) || default_impls.contains(name) {
                    duplicated_names.insert(&name);

                    println!(
                        "name:{:?}\nduplicated_names:{:?}\nset: {:?}",
                        name, duplicated_names, set
                    );
                } else {
                    set.insert(name);
                }
            }

            {
                let short_name = &x._short_name;

                if short_name == "" {
                    continue;
                }

                if (set.contains(short_name)) || default_impls.contains(short_name) {
                    duplicated_names.insert(short_name);
                } else {
                    set.insert(short_name);
                }
            }
        }

        if duplicated_names.is_empty() {
            return Ok(());
        } else {
            return Err(duplicated_names);
        }
    }

    // pub(crate) fn debug_检查子命令示example是否能正确的被解析() {}

    // pub(crate) fn debug_命令人类友好度检查(&self) {}
}
impl Default for App {
    fn default() -> Self {
        let env_args: Vec<String> = std::env::args().collect();

        let app_name = std::env::current_exe()
            .map(|x| x.file_name().map(|x| x.to_string_lossy().into_owned())) // current_exe.file_name()
            .unwrap_or(
                env_args
                    .first()
                    .cloned()
                    .map(|path| {
                        std::path::Path::new(&path)
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                    })
                    .unwrap_or(None),
            )
            .unwrap_or(String::new());

        // 第 2 个以及后面的所有.
        let sub_cmd_arg: Vec<String> = if env_args.len() > 2 {
            env_args[2..].to_vec()
        } else {
            vec![]
        };

        Self {
            _app_name: app_name,
            _about: Default::default(),
            _author: Default::default(),
            _app_version_message: "0.0.1".to_string(),
            _help_message: Default::default(),
            _commands: Default::default(),
            _env_arg: env_args,
            _examples: Examples::new(),
            _commands_arg: sub_cmd_arg,
            _app_default_action: Default::default(),
        }
    }
}
