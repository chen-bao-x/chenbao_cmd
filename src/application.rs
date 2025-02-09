use owo_colors::OwoColorize;
use prettytable::{row, table};

use super::*;
use std::{collections::HashSet, rc::Rc};

#[derive(Clone)]
pub enum AppDefaultAction {
    /// 打印 app 的帮助文档
    PrintHelpMessage,

    /// 如果想读取命令行参数, 请使用:   `let env_arg: Vec<String> = env::args().collect();`
    RunAction(Rc<dyn Fn() -> ()>),
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
    /// 此程序的名称
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
    pub commands: Vec<SubCommand>,

    /// env::args().collect()
    pub env_arg: Vec<String>,

    /// 是用此程序的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub examples: Option<Examples>,

    /// 子命令的 “参数”
    pub sub_command_arg: Vec<String>,

    /// 只输入了程序名称没有子命令也没有任何 flag 时之行的 action.
    pub _app_default_action: AppDefaultAction,
}

impl App {
    // -------- Public Part --------

    pub fn new(app_name: &str) -> App {
        return Self {
            app_name: app_name.to_string(),
            ..Default::default()
        };
    }

    pub fn add_about(self, about: &'static str) -> Self {
        let mut re = self;
        re.about = about;
        return re;
    }

    pub fn add_app_example(self, example: Option<Examples>) -> Self {
        let mut re = self;
        re.examples = example;
        return re;
    }

    /// 当用户查询此程序的 version 信息时显示的信息;
    pub fn app_version_message(self, version: String) -> Self {
        let mut re = self;
        re.app_version_message = version;
        return re;
    }

    pub fn add_author(self, author: &'static str) -> Self {
        let mut re = self;
        re.author = author;
        return re;
    }

    /// 设置只有 程序名, 没有任何子命令也没有任何参数时执行的 action.
    /// 这个函数只会生效 1 次.
    pub fn add_app_default_action<F>(self, action: F) -> Self
    where
        F: Fn() -> () + 'static,
    {
        let mut re = self;
        re._app_default_action = AppDefaultAction::RunAction(Rc::new(action));
        return re;
    }

    pub fn add_subcommand(self, command: SubCommand) -> Self {
        let mut re = self;

        re.commands.push(command);

        return re;
    }

    /// 自定义帮助信息.  
    /// 此方法会替换掉由 chenbao_cmd 提供的帮助文档.
    pub fn add_help_message(self, message: String) -> Self {
        let mut re = self;
        re.help_message = message;

        return re;
    }

    /// 运行 App.
    pub fn run(self) -> DidHandled {
        let asdf = self.env_arg.get(1);
        match asdf {
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

        for x in &self.commands {
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
        match &self.examples {
            Some(_arr) => {
                println!("{}", _arr);
            }
            None => {
                let mut table = table!();
                table.set_format(table_formater());

                for x in &self.commands {
                    let rows = x.formated_command_example(self.app_name.clone());

                    for r in rows {
                        table.add_row(r);
                    }
                }
                println!("{}", table);
                // table.printstd();
            }
        }
    }

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

    pub fn debug_duplicate_names_check(&self) -> Result<(), HashSet<String>> {
        let mut duplicated_names: HashSet<String> = HashSet::new();
        let mut set: HashSet<String> = HashSet::new();

        for x in &self.commands {
            {
                let name = x.command_name.clone();

                if set.contains(&name) {
                    println!(
                        "name:{:?}\nduplicated_names:{:?}\nset: {:?}",
                        name.clone(),
                        duplicated_names,
                        set
                    );
                    duplicated_names.insert(name);
                } else {
                    set.insert(name);
                }
            }
            {
                let short_name = x.short_name.clone();
                if set.contains(&short_name) && short_name != "" {
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

    pub fn debug_命令人类友好度检查(&self) {}
}

impl App {
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
        if let Some(需要查询的命令名称) = self.sub_command_arg.first() {
            if 需要查询的命令名称 == "help"
                || 需要查询的命令名称 == "h"
                || 需要查询的命令名称 == "-h"
                || 需要查询的命令名称 == "--help"
            {
                // 命令 ‘help' 的帮助文档

                println!("命令 ‘help' 的帮助文档");

                return DidHandled::Handled;
            }

            for x in &self.commands {
                if 需要查询的命令名称 == &x.command_name || 需要查询的命令名称 == &x.short_name
                {
                    x.print_command_help(self.app_name.clone());
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
        return DidHandled::Failed("有子命令或者 flag, 不是 app_default_acton".to_string());
    }

    fn _handle_commands(&self, command_name: &String) -> DidHandled {
        for x in &self.commands {
            if command_name == &x.command_name || command_name == &x.short_name {
                let cmd_args = &self.sub_command_arg;

                {
                    // 处理当前子命令的 flag.
                    if let Some(first_arg) = cmd_args.first() {
                        // 处理当前子命令的 help flag.
                        if first_arg == "--help" || first_arg == "-h" {
                            x.print_command_help(self.app_name.clone());
                            return DidHandled::Handled;
                        }

                        // 处理当前子命令的 example flag.
                        if first_arg == "--example" || first_arg == "-e" {
                            x.print_command_example(self.app_name.clone());
                            return DidHandled::Handled;
                        }
                    }
                }

                let v = SubcommandArgsValue::new(cmd_args.clone());
                match &x.need_arg_type {
                    ArgTypeWithAction::Empty(_f) => {
                        _f();
                        return DidHandled::Handled;
                    }
                    ArgTypeWithAction::String(_f) => {
                        let re = v.get_string();
                        match re {
                            Ok(s) => {
                                _f(s);
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::StringMutiple(_f) => {
                        let re = v.get_vec_string();
                        match re {
                            Ok(s) => {
                                _f(s);
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::Number(_f) => {
                        let re = v.get_number();
                        match re {
                            Ok(s) => {
                                _f(s);
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::NumberMutiple(_f) => {
                        let re = v.get_vec_number();
                        match re {
                            Ok(s) => {
                                _f(s);
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::Path(_f) => {
                        let re = v.get_path();
                        match re {
                            Ok(s) => {
                                _f(Rc::new(s));
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::PathMutiple(_f) => {
                        let re = v.get_vec_path();
                        match re {
                            Ok(s) => {
                                _f(Rc::new(s));
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::Bool(_f) => {
                        let re = v.get_bool();
                        match re {
                            Ok(s) => {
                                _f(s);
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::BoolMutiple(_f) => {
                        let re = v.get_vec_bool();
                        match re {
                            Ok(s) => {
                                _f(s);
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                    ArgTypeWithAction::Repl(_f) => {
                        let re = v.get_repl();
                        match re {
                            Ok(s) => {
                                _f(ReplQuestions::new(s));
                                return DidHandled::Handled;
                            }
                            Err(e) => {
                                return DidHandled::Failed(e);
                            }
                        }
                    }
                }

                // if let Some(f) = &x.action {
                //     f(SubcommandArgsValue::new(cmd_args.clone()));
                //     return DidHandled::Handled;
                // } else {
                //     return DidHandled::Failed("还没有为此命令设置 action".to_string());
                // }
            } else {
                continue;
            }
        }

        return DidHandled::Failed(format!(
            "_handle_commands(&self, command_name: &String) -> DidHandled \n未知命令: {:?}",
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

impl Default for App {
    fn default() -> Self {
        use std::env;
        let env_arg: Vec<String> = env::args().collect();
        // 第 2 个一级后面的所有.
        let sub_cmd_arg: Vec<String> = if env_arg.len() > 2 {
            env_arg[2..].to_vec()
        } else {
            vec![]
        };

        // return App {
        //     about: "",
        //     author: "",
        //     app_version_message: "0.0.1".to_string(),
        //     help_message: "".to_string(),

        //     // env_arg: env::args().collect(),
        //     env_arg: env_arg,
        //     sub_command_arg: sub_cmd_arg,
        //     _commands: vec![],
        //     _app_default_action: AppDefaultAction::PrintHelpMessage,
        //     app_name: app_name.to_string(),
        //     examples: vec![],
        //     exaples: vec![],
        // };

        Self {
            app_name: Default::default(),
            about: Default::default(),

            author: Default::default(),
            app_version_message: "0.0.1".to_string(),
            help_message: Default::default(),
            commands: Default::default(),
            env_arg: env_arg,
            examples: None,
            sub_command_arg: sub_cmd_arg,
            _app_default_action: Default::default(),
        }
    }
}
