use crate::examples_types::Examples;
use crate::helper::*;
use crate::*;
use core::fmt;
use owo_colors::OwoColorize;
use prettytable::{row, table, Table};
use std::rc::Rc;

#[derive(Clone)]
pub(crate) enum AppDefaultAction {
    /// 打印 app 的帮助文档
    PrintHelpMessage,

    /// 如果想读取命令行参数, 请使用:   `let env_arg: Vec<String> = env::args().collect();`
    CustomAction(&'static dyn Fn()),
}

impl Default for AppDefaultAction {
    fn default() -> Self {
        Self::PrintHelpMessage
    }
}
impl fmt::Debug for AppDefaultAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PrintHelpMessage => write!(f, "PrintHelpMessage"),
            Self::CustomAction(_) => f.debug_tuple("CustomAction(_)").finish(),
        }
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
///                    println!(r#"ning commmand: "run""#);
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
///                    println!("coand \"run\"{:?}\n", _x);
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("empty")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::Empty(Rc::new(|| {
///                    println!("teing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("number")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::Number(Rc::new(|_x| {
///                    println!("teing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("vecnumber")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::NumberMutiple(Rc::new(|_x| {
///                    println!("teing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("vecbool")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::BoolMutiple(Rc::new(|_x| {
///                    println!("teing arg_zero");
///                }))),
///        )
///        .add_subcommand(
///            SubCommand::new("vecstring")
///                .about("用来测试 ArgCount::Zero ")
///                .action(ArgAction::StringMutiple(Rc::new(|_x| {
///                    println!("teing arg_zero");
///                }))),
///        );
///    app.run();
/// ```
#[derive(Clone, Debug)]
pub struct App<'a> {
    /// 此程序的名称;  
    /// 所有自动生成的帮助文档和示例都会使用到 self._app_name
    pub _app_name: SharedString,

    /// 一句话介绍此程序.
    // pub _about: SharedString,
    pub _about: &'a str,

    /// 此程序的作者
    // pub _author: SharedString,
    pub _author: &'a str,

    /// 当用户查询此程序的 version 信息时显示的信息;
    // pub _app_version_message: SharedString,
    pub _app_version_message: &'a str,

    /// 此程序的帮助文档,
    pub _help_message: SharedString,

    /// 此 app 的所有子命令.
    pub _commands: Vec<SubCommand<'a>>,

    /// 使用此程序的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub _examples: Examples,

    /// 子命令的 “参数”
    pub _commands_arg: SharedVecString,

    /// 只输入了程序名称没有子命令也没有任何 flag 时之行的 action.
    /// 默认是 AppDefaultAction::PrintHelpMessage;
    _app_default_action: AppDefaultAction,

    /// `let env_arg: Vec<String> = std::env::args().collect()`
    _env_arg: SharedVecString,
}

impl<'a> App<'a> {
    // ============================
    // =        Public Part       =
    // ============================

    /// ```rust
    ///     let app = chenbao_cmd::App::new("cmd");
    ///     app.run();
    /// ```
    // pub fn new(app_name: &str) -> App {
    pub fn new() -> App<'a> {
        Self {
            ..Default::default()
        }
    }

    /// 如果不设置 app_name(_), 则会使用编译后可执行文件的文件名字作为 app_name.
    /// ```rs
    /// let app = App::new().app_name(env!("CARGO_PKG_NAME"));
    /// ```
    pub fn app_name(self, app_name: &str) -> Self {
        let mut re = self;
        re._app_name = Rc::new(app_name.to_string());
        re
    }

    /// 在这里介绍这个程序是什么. 做什么用的
    pub fn about(self, about: &'a str) -> Self {
        let mut re = self;
        // re._about = about.to_string().into();
        re._about = about;
        re
    }

    /// 使用此程序的一些示例,  
    /// 当用户使用 `app -e` 时会打印在这里添加的示例.  
    /// 此 method 可以多次调用来给此程序添加多个示例.
    pub fn add_app_example(self, command: &str, description: &str) -> Self {
        let mut re = self;

        re._examples
            .add_single_example(command.to_string(), description.to_string());

        re
    }

    /// 此程序的版本信息.  
    /// 当用户使用 `app --version` 时会打印在这里添加的版本信息.  
    /// 此 method 只需要调用一次.  
    /// ```rs
    /// let app = App::new().version_message(env!("CARGO_PKG_VERSION"));
    /// ```
    pub fn version_message(self, version_message: &'a str) -> Self {
        let mut re = self;
        // re._app_version_message = version_message.to_owned().into();
        re._app_version_message = version_message;
        re
    }

    /// 此程序的版本信息.  
    /// 当用户使用 `app --version` 时会打印在这里添加的版本信息.  
    /// 此 method 只需要调用一次.  
    pub fn author(self, author: &'a str) -> Self {
        let mut re = self;

        re._author = author;
        re
    }

    ///
    /// 设置只有 程序名, 没有任何子命令也没有任何参数时执行的 action.  
    /// 默认情况下是打印此程序的帮助信息.  
    /// `app_default_action` 有默认实现, 可以不用设置.
    pub fn app_default_action(self, action: &'static dyn Fn()) -> Self {
        let mut re = self;
        re._app_default_action = AppDefaultAction::CustomAction(action);
        re
    }

    /// add command
    /// 为此 App 添加指令
    pub fn add_command(self, command: SubCommand<'a>) -> Self {
        let mut re = self;

        re._commands.push(command);

        re
    }

    /// 自定义帮助信息.  
    /// 此方法会替换掉由 chenbao_cmd 提供的帮助文档.
    pub fn help_message(self, message: &str) -> Self {
        let mut re = self;
        re._help_message = message.to_string().into();

        re
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
    ///              println!(r#"ning commmand: "run""#);
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

                self._handle_app_default_acton()
            }
            Some(command_name) => {
                {
                    let re = self._handle_app_version();
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
                    let re = self._handle_list_all_command();
                    match re {
                        DidHandled::Handled => return re,
                        DidHandled::Failed(_x) => { /* continue. */ }
                    }
                }

                {
                    let re = self._handle_commands(command_name);
                    match re {
                        DidHandled::Handled => re,
                        DidHandled::Failed(_x) => {
                            /* 这是最后一个 handle 项目了, 直接返回. */
                            DidHandled::Failed(_x)
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
            println!("{}", self._help_message);
            return;
        }

        let mut table = table!();
        table.set_format(helper::table_formater());

        for x in &self._commands {
            let short_name = if x._short_name.is_empty() {
                "".to_string()
            } else {
                // ", ".to_string() + &x.short_name

                format!("{}{}", &x._short_name, ", ",)
            };

            let command_name = &x._name;

            // TODO: 为 cmd_name 添加颜色.
            let cmd_name = format!("{}{}", short_name, command_name,);

            table.add_row(row![cmd_name.styled_sub_command(), x._about]);
        }

        let all_commands_about: String = table.to_string();

        let app_usage = format!(
            r#"
Usage:

    {app_name} {command} {arguments}
"#,
            app_name = self._app_name.cyan(),
            command = "<command>".bright_cyan(),
            arguments = "[arguments]".green(),
        );
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
        let list_all_commands = "--list-all-commands".styled_sub_command().to_string();

        // TODO: 让打印的信息更优美.
        let flag_message = format!("{}\n    {help}\t\t显示此命令的帮助.\n    {ver}\t查看此程序的版本.\n    {example}\t查看示例.\n    {list_all_commands}\t查看所有 command.\n" , "Flags:".bright_green());
        let author = if self._author.is_empty() {
            "".to_string()
        } else {
            format!("{} {}", "Author:", self._author)
        };

        let commands = format!("{}\n{}", "Commands:".bright_green(), all_commands_about);

        println!(
            r#"
{about}
{app_usage}
{author}
{flag_message}
{commands}
"#,
            about = self._about,
            // version = self.app_versioCn_message,
        );
    }

    /// 打印 App 的示例.  
    /// `app -e` 时调用此函数.
    pub fn print_app_examples(&self) {
        if self._examples.is_empty() {
            let mut table = table!();
            table.set_format(table_formater());

            for x in &self._commands {
                let rows = x.formated_command_example(&self._app_name);

                for x in rows {
                    table.add_row(x);
                }

                // rows.row_iter().for_each(|a| {
                //     table.add_row(a.clone());
                // });
            }
            println!("{}", table);
            table.printstd();
        } else {
            // let mut table = table!();
            // table.set_format(helper::table_formater());

            // self._examples.pretty_formated().row_iter().for_each(|a| {
            //     table.add_row(a.clone());
            // });

            // println!("{}", table);

            println!("{}", vec_row_to_table(self._examples.pretty_formated()));
        }
    }
}

impl App<'_> {
    // -------- Private Part --------

    fn _handle_defalt_implement(&self) {}

    /// app help 的默认实现;  
    /// // -h --help -v -version
    fn _handle_app_help(&self) -> DidHandled {
        let command_name = &*self._env_arg[1];
        // let arr = vec!["-h", "--help", "help", "h"];
        let arr = ["-h", "--help"];

        if arr.contains(&command_name) {
            // 打印 App 的帮助信息.
            self.print_app_help();
            DidHandled::Handled
        } else {
            DidHandled::Failed(r#"不是 "-h" or "--help""#.to_string())
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
    fn _handle_app_version(&self) -> DidHandled {
        // 处理 App 的flags.
        //  -v -version
        // let command_name = self._env_arg[1].clone();
        let command_name = &self._env_arg[1];

        if command_name == "-v" || command_name == "--version" {
            println!("{}", self._app_version_message);

            DidHandled::Handled
        } else {
            DidHandled::Failed("不是 version 命令".to_string())
        }
    }

    fn _handle_list_all_command(&self) -> DidHandled {
        let command_name = &self._env_arg[1];

        if command_name == "--list-all-commands" {
            {
                // print all commands
                let mut table = Table::new();
                table.set_format(table_formater());

                self._commands.iter().for_each(|x| {
                    x.formated_row_in_list_all_command().iter().for_each(|x| {
                        table.add_row(x.clone());
                    });
                });

                println!("{}", table);
            }

            DidHandled::Handled
        } else {
            DidHandled::Failed(format!(
                "不是 {} 命令",
                "--list-all-commands".styled_sub_command()
            ))
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
        {
            if self._env_arg.len() == 1 {
                match &self._app_default_action {
                    AppDefaultAction::PrintHelpMessage => {
                        self.print_app_help();
                        return DidHandled::Handled;
                    }
                    AppDefaultAction::CustomAction(f) => {
                        f();

                        return DidHandled::Handled;
                    }
                }
            };
        }
        // return DidHandled::Failed("有子命令或者 flag, 不是 app_default_acton".to_string());
        DidHandled::Failed("有子命令或者 flag, 不是 app_default_acton".to_string())
    }

    // fn _handle_commands(&self, command_name: &String) -> DidHandled {
    fn _handle_commands(&self, command_name: &String) -> DidHandled {
        {
            for x in &self._commands {
                if command_name == &x._name || command_name == &x._short_name {
                    let cmd_args = self._commands_arg.clone();

                    return x.suc_command_run(&self._app_name, cmd_args);
                } else {
                    continue;
                }
            }
        }

        DidHandled::Failed(format!(
            "未知命令: {}\n\n输入 {} {} 查看所有命令",
            self._env_arg.join(" ").styled_sub_command(),
            self._app_name.styled_sub_command(),
            "--list-all-commands".styled_sub_command(),
        ))
    }

    fn _handle_app_example(&self) -> DidHandled {
        let command_name = self._env_arg[1].clone();

        if command_name == "-e" || command_name == "--example" {
            self.print_app_examples();

            DidHandled::Handled
        } else {
            DidHandled::Failed("不是 version 命令".to_string())
        }
    }
}

// enum Expect {
//     Right = 0,
//     Wrong = 1,
// }

impl App<'_> {
    //  ------- Debug Functions -------

    /// 模拟用户输入,  
    /// 用来更方便的测试程序.  
    // #[cfg(debug_assertions)]
    pub fn deubug_run<const N: usize>(self, virtual_env_args: [&str; N]) -> Self {
        println!(
            "------- command testing for: {} ",
            virtual_env_args.join(" ").styled_sub_command()
        );

        let mut re = self.clone();
        let env_arg: Vec<String> = virtual_env_args.iter().map(|x| x.to_string()).collect();

        // 第 2 个一级后面的所有.
        let sub_cmd_arg: SharedVecString = if env_arg.len() > 2 {
            env_arg[2..].to_vec().into()
        } else {
            vec![].into()
        };

        re._commands_arg = sub_cmd_arg;
        re._env_arg = env_arg.into();
        if re._app_name.is_empty() {
            re._app_name = env!("CARGO_PKG_NAME").to_string().into();
        }

        let did_handled = re.try_run();

        match did_handled {
            DidHandled::Handled => { /* runs perfact. */ }
            DidHandled::Failed(err_message) => {
                println!("{}", err_message);
            }
        }

        // 返回未修改的 self
        self
    }

    /// 检查子命令示example是否能正确的被解析
    /// 检查子命令的名字是否重复.
    /// 命令人类友好度检查
    #[cfg(debug_assertions)] // 只在 debug 模式下使用
    pub fn debug_check(&self) {
        // TODO: 打印两个名称冲突的 Cmd

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
    // ) -> Result<(), std::collections::HashSet<&String>> {
    ) -> Result<(), std::collections::HashSet<&str>> {
        use std::collections::HashSet;

        // 重复了的子命令名称.

        // let mut duplicated_names: HashSet<&String> = HashSet::new();
        let mut duplicated_names: HashSet<&str> = HashSet::new();

        // 子命令的名字合集.
        // let mut set: HashSet<&String> = HashSet::new();
        let mut set: HashSet<&str> = HashSet::new();

        // let mut default_impls: HashSet<String> = HashSet::new();
        let mut default_impls: HashSet<&str> = HashSet::new();
        {
            // 这几个是 chenbao_cmd  自带的默认实现的 子命令和 flag, 不能被自定义.
            // default_impls.insert("-h".to_string());
            // default_impls.insert("--help".to_string());
            // default_impls.insert("-v".to_string());
            // default_impls.insert("--version".to_string());
            // default_impls.insert("-e".to_string());
            // default_impls.insert("--example".to_string());
            
            default_impls.insert("-h");
            default_impls.insert("--help");
            default_impls.insert("-v");
            default_impls.insert("--version");
            default_impls.insert("-e");
            default_impls.insert("--example");
        }
        for x in &self._commands {
            {
                let name = &x._name;

                if set.contains(name) || default_impls.contains(name) {
                    duplicated_names.insert(name);

                    println!(
                        "name:{:?}\nduplicated_names:{:?}\nset: {:?}",
                        name, duplicated_names, set
                    );
                } else {
                    set.insert(name);
                }
            }

            {
                let short_name = x._short_name;

                if short_name.is_empty() {
                    continue;
                }

                if (set.contains(&short_name)) || default_impls.contains(short_name) {
                    duplicated_names.insert(short_name);
                } else {
                    set.insert(short_name);
                }
            }
        }

        if duplicated_names.is_empty() {
            Ok(())
        } else {
            Err(duplicated_names)
        }
    }

    // pub(crate) fn debug_检查子命令示example是否能正确的被解析() {}

    // pub(crate) fn debug_命令人类友好度检查(&self) {}
}

impl Default for App<'_> {
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
            .unwrap_or_default();

        // 第 2 个以及后面的所有.
        let sub_cmd_arg: Vec<String> = if env_args.len() > 2 {
            env_args[2..].to_vec()
        } else {
            vec![]
        };

        Self {
            _app_name: app_name.into(),
            _about: Default::default(),
            _author: Default::default(),
            _app_version_message: "0.0.1",
            _help_message: Default::default(),
            _commands: Default::default(),
            _env_arg: env_args.into(),
            _examples: Examples::new(),
            _commands_arg: sub_cmd_arg.into(),
            _app_default_action: Default::default(),
        }
    }
}
