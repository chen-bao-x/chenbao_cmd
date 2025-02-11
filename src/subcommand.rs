use crate::{
    action::{ArgAction, ParseResult, SubcommandArgsValue},
    examples_types::Examples,
    helper::*,
};

use super::*;
use application::DidHandled;
use owo_colors::OwoColorize;
use prettytable::{row, table, Table};
use std::vec;

/// 设置子命令的名称 功能和参数类型.  
#[derive(Clone)]
pub struct SubCommand {
    /// 命令名   
    /// 命令的名称长度最好不要超过 20 个字符.
    pub _command_name: String,

    /// 命令名的简写形式, 通常是一个字符  
    pub _short_name: String,

    /// 一句话介绍此命令
    pub _about: String,

    /// 是用此命令的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub _exaples: Examples,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    pub _help_document: Option<String>,

    /// 子命令需要的参数的类型以及该子命令的 action.
    /// 在打印子命令的帮助文档时需要用到此属性.
    pub _arg_type_with_action: ArgAction,
}

impl SubCommand {
    /// 创建新的 SubCommand.  
    ///
    /// 这几个已经又了默认实现, 不能再作为子命令的名称:  
    /// "-h" "--help"  
    /// "-e" "--help"  
    /// "-e" "--example"  
    /// ```
    /// use chenbao_cmd::*;
    /// SubCommand::new("build")
    ///     .short_name("b")
    ///     .about("编译项目")
    ///     .action(ArgAction::Bool(std::rc::Rc::new(|_x| {
    ///         print!("command \"run\"{:?}\n", _x);
    ///     })));
    /// ```
    pub fn new(name: &str) -> Self {
        #[cfg(debug_assertions)]
        {
            let re = SubCommand::command_name_check(name);
            if let Err(s) = re {
                panic!("{}", s);
            }
        }

        return SubCommand {
            _command_name: name.to_string(),
            _about: "".to_string(),
            _help_document: None,
            _short_name: "".to_string(),
            _exaples: Examples::new(),
            _arg_type_with_action: ArgAction::default(),
        };
    }
}

impl SubCommand {
    /// set `Command.short_name`
    pub fn short_name(self, short_name: &str) -> Self {
        let mut re = self;
        re._short_name = short_name.to_string();
        return re;
    }

    /// set `SubCommand.about`
    pub fn about(self, about: &str) -> Self {
        let mut re = self;
        re._about = about.to_string();
        return re;
    }

    pub fn add_example(self, command: &'static str, description: &'static str) -> Self {
        // TODO: 检查 `command: &'static str` 是否是可执行的 command.

        let mut re = self;
        re._exaples.add_single_example(command, description);

        return re;
    }

    /// set `Command.example`
    pub fn help_document(self, str: String) -> Self {
        let mut re = self;
        re._help_document = Some(str);

        return re;
    }

    /// set `Command.action`
    pub fn action(self, need_arg_type: ArgAction) -> Self {
        let mut re = self;
        re._arg_type_with_action = need_arg_type;

        return re;
    }

    pub fn run(&self, app_name: &String, cmd_args: &Vec<String>) -> DidHandled {
        self.try_run(app_name, cmd_args, true)
    }
}

impl SubCommand {
    pub fn print_command_help(&self, app_name: &String) {
        println!("{}", self.formated_command_help(app_name));
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
            self._command_name.styled_sub_command()
        );
        println!();
        println!("{}", table);
    }
}
impl SubCommand {
    fn formated_usage(&self, app_name: &String) -> String {
        let command_name = self._command_name.styled_sub_command();
        let short_name = self._short_name.styled_sub_command();

        let arg_in_usage = match self._arg_type_with_action {
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
        if self._short_name == "" {
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

    /// `app cmd -h` 时显示的帮助文档.
    pub fn formated_command_help(&self, app_name: &String) -> String {
        if let Some(s) = &self._help_document {
            // 自定义了帮助文档的情况;
            return format!("{}", s);
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg_message: String = if self._arg_type_with_action.arg_message() == "" {
                format!(
                    r#"
                {}{}"#,
                    "Arguments:\n",
                    self._arg_type_with_action.arg_message()
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
                about = self._about,
                // command_name = self.command_name.styled_sub_command(),
                Usage = self.formated_usage(&app_name),
            );

            return message;
        }
    }

    /// 已经格式化好了, 直接放进 Table 打印就行.
    pub fn formated_command_example(&self, app_name: &String) -> Table {
        if self._exaples.is_empty() {
            let mut table = table!();
            table.set_format(helper::table_formater());

            let arg = self
                ._arg_type_with_action
                .value_example()
                .bright_green()
                .to_string();

            table.add_row(row![
                format!(
                    "{app_name} {command_name} {arg}",
                    command_name = self._command_name.styled_sub_command(),
                ),
                self._about
            ]);

            return table;
        } else {
            return self._exaples.pretty_formated();
        }
    }

    // /// 检查 example 里面的 command 是否能够被正常解析.
    // pub fn check(&self, app_name: &String, cmd_args: &Vec<String>) -> DidHandled {
    //     for x in &self._exaples.val {
    //         let asdf = parse_arg_string(x.command);
    //     }
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
            let arg_message = self._arg_type_with_action.arg_message();
            // let n = need_to_run;
            let v = SubcommandArgsValue::new(cmd_args.clone());

            let re = match &self._arg_type_with_action {
                ArgAction::Empty(f) => adsfdasf(v.get_empty(), need_to_run, f),
                ArgAction::String(f) => adsfdasf(v.get_string(), need_to_run, f),
                ArgAction::StringMutiple(f) => adsfdasf(v.get_vec_string(), need_to_run, f),
                ArgAction::Number(f) => adsfdasf(v.get_number(), need_to_run, f),
                ArgAction::NumberMutiple(f) => adsfdasf(v.get_vec_number(), need_to_run, f),
                ArgAction::Path(f) => adsfdasf(v.get_path(), need_to_run, f),
                ArgAction::PathMutiple(f) => adsfdasf(v.get_vec_path(), need_to_run, f),
                ArgAction::Bool(f) => adsfdasf(v.get_bool(), need_to_run, f),
                ArgAction::BoolMutiple(f) => adsfdasf(v.get_vec_bool(), need_to_run, f),
                ArgAction::Dialog(f) => adsfdasf(v.get_repl(), need_to_run, &|s| {
                    f(&mut arg_type::Dialog::new(s.as_deref()));
                }),
            };

            if let DidHandled::Failed(err) = re {
                return DidHandled::Failed(format!(
                    r#"
{}{}

{}
                "#,
                    "error: ".bright_red(),
                    err,
                    arg_message,
                ));
            }

            return re;

            fn adsfdasf<T>(
                result: ParseResult<T>,
                need_run_action: bool,
                func: &dyn Fn(T) -> (),
            ) -> DidHandled {
                match result {
                    Ok(s) => {
                        if need_run_action {
                            func(s);
                        }
                        return DidHandled::Handled;
                    }
                    Err(err) => return DidHandled::Failed(err),
                }
            }
        }

        // {
        //     let arg_message = self._arg_type_with_action.arg_message();
        //     let n = need_to_run;
        //     let v = SubcommandArgsValue::new(cmd_args.clone());

        //     return match &self._arg_type_with_action {
        //         ArgAction::Empty(f) => _try_run(v.get_empty(), n, arg_message, f),
        //         ArgAction::String(f) => _try_run(v.get_string(), n, arg_message, f),
        //         ArgAction::StringMutiple(f) => _try_run(v.get_vec_string(), n, arg_message, f),
        //         ArgAction::Number(f) => _try_run(v.get_number(), n, arg_message, f),
        //         ArgAction::NumberMutiple(f) => _try_run(v.get_vec_number(), n, arg_message, f),
        //         ArgAction::Path(f) => _try_run(v.get_path(), n, arg_message, f),
        //         ArgAction::PathMutiple(f) => _try_run(v.get_vec_path(), n, arg_message, f),
        //         ArgAction::Bool(f) => _try_run(v.get_bool(), n, arg_message, f),
        //         ArgAction::BoolMutiple(f) => _try_run(v.get_vec_bool(), n, arg_message, f),
        //         ArgAction::Dialog(f) => _try_run(v.get_repl(), n, arg_message, &|s| {
        //             f(&mut arg_type::Dialog::new(s.as_deref()));
        //         }),
        //     };
        // }

        //         // match_and_try_run
        //         fn _try_run<T>(
        //             result: ParseResult<T>,
        //             need_run_action: bool,
        //             arg_message: String,
        //             func: &dyn Fn(T) -> (),
        //         ) -> DidHandled {
        //             match result {
        //                 Ok(s) => {
        //                     if need_run_action {
        //                         func(s);
        //                     }
        //                     return DidHandled::Handled;
        //                 }
        //                 Err(err) => {
        //                     return DidHandled::Failed(format!(
        //                         r#"
        // {}{}

        // {}
        //                     "#,
        //                         "error: ".bright_red(),
        //                         err,
        //                         arg_message,
        //                     ));
        //                 }
        //             }
        //         }
    }

    /// 检查 子命令 的名字是否符合要求.
    #[cfg(debug_assertions)] // 只在 debug 模式下使用
    fn command_name_check(name: &str) -> Result<(), String> {
        if name == "" {
            let msg = format!(
                r#"
    {error}: name 不能是空字符串 "", name 的值至少需要一个字符.
    
    如果想要设置只有 程序名 时执行的 action.
    
    请使用: app_default_action(_) 来设置.
    示例:
    let app = App::new("cmd")
        .app_default_action(||{{ /* action */ }}) 
    "#,
                error = "error".red(),
            );

            return Err(msg);
        }

        let arr = vec!["-h", "--help", "-e", "--example", "-v", "--version"];

        if arr.contains(&name) {
            let msg = format!(
                r#"
{error}: name 不能是 "-h", "--help", "-e", "--example", "-v", "--version", 这些已经有了默认的实现.
    "#,
                error = "error".red(),
            );

            return Err(msg);
        }

        return Ok(());
    }
}
