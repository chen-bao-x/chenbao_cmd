use super::*;
use application::DidHandled;
use owo_colors::OwoColorize;
use prettytable::{row, table, Table};
use std::vec;

// subcommand
#[derive(Clone)]
pub struct SubCommand {
    /// 命令名   
    /// 命令的名称长度最好不要超过 20 个字符.
    pub command_name: String,

    /// 命令名的简写形式, 通常是一个字符  
    pub short_name: String,

    /// 一句话介绍此命令
    pub about: String,

    /// 是用此命令的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub exaples: Examples,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    pub help_document: String,

    /// 子命令需要的参数的类型以及该子命令的 action.
    /// 在打印子命令的帮助文档时需要用到此属性.
    pub arg_type_with_action: ArgAction,
}

impl SubCommand {
    pub fn new(name: &str) -> Self {
        if is_debug_mode() && name == "" {
            eprintln!("WARNING: name 不能是空字符串 \"\", name 的值至少需要一个字符.");
        }

        return SubCommand {
            command_name: name.to_string(),
            about: "".to_string(),
            help_document: "".to_string(),
            short_name: "".to_string(),
            exaples: Examples::new(),
            arg_type_with_action: ArgAction::default(),
        };
    }

    /// set `Command.short_name`
    pub fn short_name(self, short_name: &str) -> Self {
        let mut re = self;
        re.short_name = short_name.to_string();
        return re;
    }

    /// set `SubCommand.about`
    pub fn about(self, about: &str) -> Self {
        let mut re = self;
        re.about = about.to_string();
        return re;
    }

    pub fn add_sub_command_example(self, command: &'static str, description: &'static str) -> Self {
        // TODO: 检查 `command: &'static str` 是否是可执行的 command.

        let mut re = self;
        re.exaples.add_single_example(command, description);

        return re;
    }

    /// set `Command.example`
    pub fn help_document(self, str: &str) -> Self {
        let mut re = self;
        re.help_document = str.to_string();
        return re;
    }

    /// set `Command.action`
    pub fn action(self, need_arg_type: ArgAction) -> Self {
        let mut re = self;
        re.arg_type_with_action = need_arg_type;

        return re;
    }

    pub fn run(&self, app_name: &String, cmd_args: &Vec<String>) -> DidHandled {
        self.try_run(app_name, cmd_args, true)
    }

    // /// 检查 example 里面的 command 是否能够被正常解析.
    // pub fn check(&self, app_name: &String, cmd_args: &Vec<String>) -> DidHandled {
    //     // for x in &self.exaples.val {
    //     //     let sadfdsaf = helper::parse_arg_string(x.command);
    //     // }
    //     self.try_run(app_name, cmd_args, false)
    // }

    fn try_run(&self, app_name: &String, cmd_args: &Vec<String>, need_to_run: bool) -> DidHandled {
        {
            // 处理当前 子命令 的 flag.
            if let Some(first_arg) = cmd_args.first() {
                // 处理当前子命令的 help flag.
                if first_arg == "--help" || first_arg == "-h" {
                    if need_to_run {
                        self.print_command_help(app_name);
                    }
                    return DidHandled::Handled;
                }

                // 处理当前子命令的 example flag.
                if first_arg == "--example" || first_arg == "-e" {
                    if need_to_run {
                        self.print_command_example(app_name);
                    }
                    return DidHandled::Handled;
                }
            }
        }

        {
            let m = self.arg_type_with_action.arg_message();
            let v = SubcommandArgsValue::new(cmd_args.clone());

            return match &self.arg_type_with_action {
                ArgAction::Empty(func) => {
                    match_and_try_run(v.get_empty(), need_to_run, m, |_s| func())
                }
                ArgAction::String(func) => {
                    match_and_try_run(v.get_string(), need_to_run, m, |_s| func(_s))
                }
                ArgAction::StringMutiple(func) => {
                    match_and_try_run(v.get_vec_string(), need_to_run, m, |_s| func(_s))
                }
                ArgAction::Number(func) => {
                    match_and_try_run(v.get_number(), need_to_run, m, |_s| func(_s))
                }
                ArgAction::NumberMutiple(func) => {
                    match_and_try_run(v.get_vec_number(), need_to_run, m, |_s| func(_s))
                }
                ArgAction::Path(func) => {
                    match_and_try_run(v.get_path(), need_to_run, m, |_s| func(_s.into()))
                }
                ArgAction::PathMutiple(func) => {
                    match_and_try_run(v.get_vec_path(), need_to_run, m, |_s| func(_s.into()))
                }
                ArgAction::Bool(func) => {
                    match_and_try_run(v.get_bool(), need_to_run, m, |_s| func(_s))
                }
                ArgAction::BoolMutiple(func) => {
                    match_and_try_run(v.get_vec_bool(), need_to_run, m, |_s| func(_s))
                }
                ArgAction::Dialog(func) => match_and_try_run(v.get_repl(), need_to_run, m, |s| {
                    func(arg_type::Dialog::new(s.as_deref()));
                }),
            };
        }

        /// 之前匹配 ArgAction 的时候发现
        /// ArgAction::String(func) => {
        ///     let re = v.get_string();
        ///     match re {
        ///         Ok(s) => {
        ///             if need_run_action {
        ///                 func(s);
        ///             }
        ///             return DidHandled::Handled;
        ///         }
        ///         Err(e) => {
        ///             // return DidHandled::Failed(e);
        ///             return DidHandled::Failed(format!(
        ///                 "{}\n{}",
        ///                 e,
        ///                 self.arg_type_with_action.arg_message()
        ///             ));
        ///         }
        ///     }
        /// }
        ///
        fn match_and_try_run<T, F>(
            result: ParseResult<T>,
            need_run_action: bool,
            arg_message: String,
            func: F,
        ) -> DidHandled
        where
            F: Fn(T) -> (),
        {
            match result {
                Ok(s) => {
                    if need_run_action {
                        func(s);
                    }
                    return DidHandled::Handled;
                }
                Err(e) => {
                    return DidHandled::Failed(format!(
                        r#"
{}{}

{}
                    "#,
                        "error: ".bright_red(),
                        e,
                        arg_message,
                    ));
                }
            }
        }
    }
}

impl SubCommand {
    /// `app cmd -h` 时显示的帮助文档.
    pub fn formated_command_help(&self, app_name: &String) -> String {
        if self.help_document != "" {
            // 自定义了帮助文档的情况;
            return format!("{}", self.help_document);
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg_message: String = if self.arg_type_with_action.arg_message() == "" {
                format!(
                    r#"
                    {}{}"#,
                    "Arguments:\n",
                    self.arg_type_with_action.arg_message()
                )
            } else {
                String::new()
            };

            let help = format!(
                "{}, {}",
                "-h".styled_sub_command(),
                "--help".styled_sub_command()
            );
            let example = format!(
                "{}, {}",
                "-e".styled_sub_command(),
                "--example".styled_sub_command()
            );
            let flag_message =
                format!("Flags:\n    {help}\t\t显示此命令的帮助.\n    {example}\t查看示例.\n");

            let message = format!(
                r#"
{about}
{Usage}
{arg_message}
{flag_message}

"#,
                about = self.about,
                // command_name = self.command_name.styled_sub_command(),
                Usage = self.formated_usage(&app_name),
            );

            return message;
        }
    }

    pub fn print_command_help(&self, app_name: &String) {
        println!("{}", self.formated_command_help(app_name));
    }

    /// 已经格式化好了, 直接放进 Table 打印就行.
    pub fn formated_command_example(&self, app_name: &String) -> Table {
        if self.exaples.is_empty() {
            let mut table = table!();
            table.set_format(helper::table_formater());

            let arg = self
                .arg_type_with_action
                .value_example()
                .bright_green()
                .to_string();

            table.add_row(row![
                format!(
                    "{app_name} {command_name} {arg}",
                    command_name = self.command_name.styled_sub_command(),
                ),
                self.about
            ]);

            return table;
        } else {
            return self.exaples.pretty_formated();
        }
    }

    /// `app cmd -e` 打印当前子命令的示例.
    pub fn print_command_example(&self, app_name: &String) {
        let arr = self.formated_command_example(app_name);
        let mut table = table!();
        table.set_format(table_formater());

        arr.row_iter().for_each(|x| {
            table.add_row(x.clone());
        });

        println!(
            "子命令 {} {} 的使用示例:",
            app_name,
            self.command_name.styled_sub_command()
        );
        println!();
        println!("{}", table);
    }
}
impl SubCommand {
    fn formated_usage(&self, app_name: &String) -> String {
        let command_name = self.command_name.styled_sub_command();
        let short_name = self.short_name.styled_sub_command();

        let arg_in_usage = match self.arg_type_with_action {
            ArgAction::Empty(_) => "".to_string(),

            ArgAction::String(_) => format!(r#"{}"#, "String".styled_arg_type()),
            ArgAction::Number(_) => format!(r#"{}"#, "Number".styled_arg_type()),
            ArgAction::Path(_) => format!(r#"{}"#, "Path".styled_arg_type()),
            ArgAction::Bool(_) => format!(r#"{}"#, "Bool".styled_arg_type()),

            ArgAction::StringMutiple(_) => format!(r#"{}..."#, "String".styled_arg_type()),
            ArgAction::NumberMutiple(_) => format!(r#"{}..."#, "Number".styled_arg_type()),
            ArgAction::PathMutiple(_) => format!(r#"{}..."#, "Path".styled_arg_type()),
            ArgAction::BoolMutiple(_) => format!(r#"{}..."#, "Bool".styled_arg_type()),

            ArgAction::Dialog(_) => "".to_string(),
        };

        let arg_in_usage = arg_in_usage;
        if self.short_name == "" {
            return format!(
                r#"
Usage: 
    {app_name} {command_name} {arg_in_usage}"#
            );
        } else {
            return format!(
                r#"
Usage: 
    {app_name} {command_name} {arg_in_usage}
    {app_name} {short_name} {arg_in_usage}"#
            );
        }
    }
}
