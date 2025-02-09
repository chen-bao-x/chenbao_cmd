//! ```rust
//! let app = App::new("cmd")
//!      .add_about("这个程序主要是为了测试我写的 cmd crate")
//!      .add_author("chen bao")
//!      .app_version_message("0.0.1".to_string())
//!      .add_subcommand(
//!          SubCommand::new("run")
//!              .about("运行程序")
//!              .action(ArgAction::Bool(Rc::new(|_x| {
//!                  print!("command \"run\"{:?}\n", _x);
//!              }))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("help")
//!              .about("运行程序")
//!              .action(ArgAction::Empty(Rc::new(|| {}))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("build")
//!              .short_name("b")
//!              .about("编译项目")
//!              .action(ArgAction::Bool(Rc::new(|_x| {
//!                  print!("command \"run\"{:?}\n", _x);
//!              }))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("empty")
//!              .about("用来测试 ArgCount::Zero ")
//!              .action(ArgAction::Empty(Rc::new(|| {
//!                  print!("testing arg_zero");
//!              }))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("number")
//!              .about("用来测试 ArgCount::Zero ")
//!              .action(ArgAction::Number(Rc::new(|_x| {
//!                  print!("testing arg_zero");
//!              }))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("vecnumber")
//!              .about("用来测试 ArgCount::Zero ")
//!              .action(ArgAction::NumberMutiple(Rc::new(|_x| {
//!                  print!("testing arg_zero");
//!              }))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("vecbool")
//!              .about("用来测试 ArgCount::Zero ")
//!              .action(ArgAction::BoolMutiple(Rc::new(|_x| {
//!                  print!("testing arg_zero");
//!              }))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("vecstring")
//!              .about("用来测试 ArgCount::Zero ")
//!              .action(ArgAction::StringMutiple(Rc::new(|_x| {
//!                  print!("testing arg_zero");
//!              }))),
//!      )
//!      .add_subcommand(
//!          SubCommand::new("repl")
//!              .about("用来测试 ArgCount::Repl(_) ")
//!              .action(ArgAction::Dialog(Rc::new(|r| {
//!                  let mut 你要吃几个汉堡包: arg_type::Number = 0;
//!                  let mut 多个_number: arg_type::NumberMutiple = vec![];
//!                  let mut string: String = String::new();
//!                  let mut string_multiple: Vec<String> = vec![];
//!                  let mut req_bool: arg_type::Bool = false;
//!                  let mut req_bool_multiple: arg_type::BoolMutiple = vec![];
//!      
//!                  r.number(&mut 你要吃几个汉堡包, "你要吃几个汉堡包?")
//!                      .req_multiple_number(&mut 多个_number, "多个 number")
//!                      .string(&mut string, "string")
//!                      .string_multiple(&mut string_multiple, "string_multiple")
//!                      .yes_or_no(&mut req_bool, "bool")
//!                      .yes_or_no_multiple(&mut req_bool_multiple, "bool mutiple");
//!                  }))),
//!         );
//!
//!         app.run();
//! ```

use owo_colors::OwoColorize;
use prettytable::{row, table};

use super::*;
use std::{collections::HashSet, rc::Rc};

#[derive(Clone)]
pub enum AppDefaultAction {
    /// 打印 app 的帮助文档
    PrintHelpMessage,

    /// 如果想读取命令行参数, 请使用:   `let env_arg: Vec<String> = env::args().collect();`
    CustomAction(Rc<dyn Fn() -> ()>),
}

impl Default for AppDefaultAction {
    fn default() -> Self {
        Self::PrintHelpMessage
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DidHandled {
    /// 表示匹配到了相关命令并正确执行了相关 action.
    Handled,

    /// 没匹配到相关命令或者其他错误.
    Failed(String),
}

#[derive(Clone)]
pub struct App {
    /// 此程序的名称;  
    /// 所有自动生成的帮助文档和示例都会使用到 {app_name}
    pub app_name: String,

    /// 一句话介绍此程序.
    pub about: &'static str,

    /// 此程序的作者
    pub author: &'static str,

    /// 当用户查询此程序的 version 信息时显示的信息;
    pub app_version_message: String,

    /// 此程序的帮助文档,
    pub help_message: String,

    /// 此 app 的所有子命令.
    pub sub_commands: Vec<SubCommand>,

    /// let env_arg: Vec<String> = std::env::args().collect()
    pub env_arg: Vec<String>,

    /// 使用此程序的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub examples: Examples,

    /// 子命令的 “参数”
    pub sub_command_arg: Vec<String>,

    /// 只输入了程序名称没有子命令也没有任何 flag 时之行的 action.
    /// 默认是 AppDefaultAction::PrintHelpMessage;
    pub app_default_action: AppDefaultAction,
}

impl App {
    // ============================
    // =        Public Part       =
    // ============================

    pub fn new(app_name: &str) -> App {
        return Self {
            app_name: app_name.to_string(),
            ..Default::default()
        };
    }

    pub fn about(self, about: &'static str) -> Self {
        let mut re = self;
        re.about = about;
        return re;
    }

    pub fn add_app_example(self, command: &'static str, description: &'static str) -> Self {
        let mut re = self;

        re.examples.add_single_example(command, description);

        return re;
    }

    /// 当用户查询此程序的 --version 信息时显示的信息;
    pub fn version_message(self, version: String) -> Self {
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
        re.app_default_action = AppDefaultAction::CustomAction(Rc::new(action));
        return re;
    }

    pub fn add_subcommand(self, command: SubCommand) -> Self {
        let mut re = self;

        re.sub_commands.push(command);

        return re;
    }

    /// 自定义帮助信息.  
    /// 此方法会替换掉由 chenbao_cmd 提供的帮助文档.
    pub fn help_message(self, message: String) -> Self {
        let mut re = self;
        re.help_message = message;

        return re;
    }

    /// 运行 App.
    pub fn run(self) -> DidHandled {
        debug_run(|| {
            self.debug_check();
        });

        let option_string = self.env_arg.get(1);
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
                        DidHandled::Failed(_x) => {}
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

    pub fn print_app_help(&self) {
        if self.help_message.trim() != "" {
            // 有自定义的帮助文档.
            print!("{}", self.help_message);
            return;
        }

        let mut table = table!();
        table.set_format(table_formater());

        for x in &self.sub_commands {
            let short_name = if x.short_name == "" {
                "".to_string()
            } else {
                // ", ".to_string() + &x.short_name
                x.short_name.to_string()
                // format!("{}{}", ", ", &x.short_name )
            };

            let command_name = &x.command_name;

            // TODO: 为 cmd_name 添加颜色.
            let cmd_name = format!("{}, {}", command_name.cyan(), short_name.cyan());

            table.add_row(row![cmd_name, x.about]);
        }

        let all_commands_about: String = table.to_string();

        let help = format!("{}, {}", "-h".cyan(), "--help".cyan());
        let ver = format!("{}, {}", "-v".cyan(), "--version".cyan());
        let example = format!("{}, {}", "-e".cyan(), "--example".cyan());

        // TODO: 让打印的信息更优美.
        let flag_message = format!("Flags:\n\n    {help}\t\t显示此命令的帮助.\n    {ver}\t查看此程序的版本.\n    {example}\t查看示例.\n" );

        let message = format!(
            r#"
{about}
Author: {author}
Version: {version}

{flag_message}
Commands:

{all_commands_about}
"#,
            about = self.about,
            author = self.author,
            version = self.app_version_message,
        );
        print!("{}", message);
    }

    pub fn print_app_examples(&self) {
        if self.examples.is_empty() {
            let mut table = table!();
            table.set_format(table_formater());

            for x in &self.sub_commands {
                let rows = x.formated_command_example(&self.app_name);
                rows.row_iter().for_each(|a| {
                    table.add_row(a.clone());
                });
                // for r in rows {
                //     table.add_row(r);
                // }
            }
            println!("{}", table);
            // table.printstd();
        } else {
            let mut table = table!();
            table.set_format(helper::table_formater());

            self.examples.pretty_formated().row_iter().for_each(|a| {
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
        let command_name = self.env_arg[1].clone();

        if !(command_name == "-h" || command_name == "--help") {
            return DidHandled::Failed("不是程序的 help 命令".to_string());
        }

        //  "help" 命令 的默认实现, 这里处理的是: 是用 help 命令查询其他命令.
        // 比如 `app help run` 查询 run 命令的帮助文档. 效果等同于 `app run --help`
        if let Some(需要查询的命令名称) = self.sub_command_arg.first() {
            if 需要查询的命令名称 == "-h" || 需要查询的命令名称 == "--help" {
                // 命令 ‘help' 的帮助文档

                println!("命令 ‘help' 的帮助文档");

                return DidHandled::Handled;
            }

            for x in &self.sub_commands {
                if 需要查询的命令名称 == &x.command_name || 需要查询的命令名称 == &x.short_name
                {
                    x.print_command_help(&self.app_name);
                    return DidHandled::Handled;
                }
            }
            return DidHandled::Failed(format!("查询的命令 {} 不存在", 需要查询的命令名称.cyan()));
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

        if command_name == "-v" || command_name == "--version" {
            println!("{}", self.app_version_message);

            return DidHandled::Handled;
        } else {
            return DidHandled::Failed("不是 version 命令".to_string());
        }
    }

    /// app version 命令的默认实现
    fn _heldle_app_example(&self) -> DidHandled {
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
            return DidHandled::Failed(format!("不是 {} 命令", "version".cyan()));
        }
    }

    /// 处理只输入了程序名称没有子命令也没有任何 flag 的情况.
    fn _handle_app_default_acton(&self) -> DidHandled {
        if self.env_arg.len() == 1 {
            match &self.app_default_action {
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
        for x in &self.sub_commands {
            if command_name == &x.command_name || command_name == &x.short_name {
                let cmd_args = &self.sub_command_arg;

                return x.run(&self.app_name, cmd_args);
            } else {
                continue;
            }
        }

        return DidHandled::Failed(format!(
            "_handle_commands(_) -> DidHandled \n未知命令: {:?}",
            self.env_arg
        ));
    }

    fn _handle_app_example(&self) -> DidHandled {
        let command_name = self.env_arg[1].clone();

        if command_name == "e"
            || command_name == "example"
            || command_name == "-e"
            || command_name == "--example"
        {
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

        re.sub_command_arg = sub_cmd_arg;
        re.env_arg = env_arg;

        let did_handled = re.run();

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
    pub(crate) fn debug_check(&self) {
        let re = self.debug_duplicate_names_check();
        if let Err(duplicate_names) = re {
            println!("{}", "有子命令的名称重复了:".bold());

            duplicate_names.iter().for_each(|x| {
                println!("{}", x);
            });
        }
    }

    /// 检查子命令的名字是否重复.
    pub(crate) fn debug_duplicate_names_check(&self) -> Result<(), HashSet<&String>> {
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
        for x in &self.sub_commands {
            {
                let name = &x.command_name;

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
                let short_name = &x.short_name;

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
        let env_arg: Vec<String> = std::env::args().collect();

        // 第 2 个一级后面的所有.
        let sub_cmd_arg: Vec<String> = if env_arg.len() > 2 {
            env_arg[2..].to_vec()
        } else {
            vec![]
        };

        Self {
            app_name: Default::default(),
            about: Default::default(),
            author: Default::default(),
            app_version_message: "0.0.1".to_string(),
            help_message: Default::default(),
            sub_commands: Default::default(),
            env_arg: env_arg,
            examples: Examples::new(),
            sub_command_arg: sub_cmd_arg,
            app_default_action: Default::default(),
        }
    }
}
