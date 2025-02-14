use crate::{
    action::{ArgAction, ParseResult, SubcommandArgsValue},
    examples_types::{Examples, SingleExample},
    helper::*,
};

use super::*;
use application::DidHandled;
use owo_colors::OwoColorize;
use prettytable::{row, Row};

/// 子命令
#[derive(Clone, Debug)]
pub struct SubCommand<'a> {
    /// 子命令名   
    /// 命令的名称长度最好不要超过 20 个字符.
    // pub _name: String,
    pub _name: &'a str,

    /// 命令名的简写形式, 通常是一个字符  
    // pub _short_name: String,
    pub _short_name: &'a str,

    /// 一句话介绍此命令
    // pub _about: String,
    pub _about: &'a str,

    /// 是用此命令的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub _exaples: Examples<'a>,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    // pub _help_message: Option<String>,
    pub _help_message: Option<&'a str>,

    /// 子命令需要的参数的类型以及该子命令的 action.
    /// 在打印子命令的帮助文档时需要用到此属性.
    pub _arg_type_with_action: ArgAction,
}

impl<'a> SubCommand<'a> {
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
    pub fn create_an_sub_command(name: &'a str) -> Self {
        #[cfg(debug_assertions)]
        {
            let re = SubCommand::command_name_check(name);
            if let Err(s) = re {
                panic!("{}", s);
            }
        }

        SubCommand {
            _name: name,

            _about: "",
            _help_message: None,
            _short_name: "",
            _exaples: Examples::new(),
            _arg_type_with_action: ArgAction::default(),
        }
    }
}

impl<'a> SubCommand<'a> {
    /// set `Command.short_name`
    pub fn short_name(self, short_name: &'a str) -> Self {
        let mut re = self;
        re._short_name = short_name;
        re
    }

    /// set `SubCommand.about`
    pub fn about(self, about: &'a str) -> Self {
        let mut re = self;
        re._about = about;
        re
    }

    pub fn add_example(self, command: &'a str, description: &'a str) -> Self {
        // TODO: 检查 `command: &'static str` 是否是可执行的 command.

        let mut re = self;
        re._exaples.add_single_example(command, description);

        re
    }

    /// set `Command.example`
    pub fn help_document(self, str: &'a str) -> Self {
        let mut re = self;
        re._help_message = Some(str);

        re
    }

    /// set `Command.action`
    pub fn action(self, need_arg_type: ArgAction) -> Self {
        let mut re = self;
        re._arg_type_with_action = need_arg_type;

        re
    }

    // pub fn suc_command_run(&self, app_name: &String, cmd_args: &[String]) -> DidHandled {
    pub fn suc_command_run(&self, app_name: &str, cmd_args: SharedVecString) -> DidHandled {
        self.suc_command_try_run(app_name, cmd_args, true)
    }
}

impl SubCommand<'_> {
    pub fn print_command_help(&self, app_name: &str) {
        println!("{}", self.formated_command_help(app_name));
    }

    /// `app cmd -e` 打印当前子命令的示例.
    pub fn print_command_example(&self, app_name: &str) {
        let arr = self.formated_command_example(app_name);

        let table = helper::vec_row_to_table(arr);

        println!(
            "子命令 {} {} 的使用示例:",
            app_name,
            self._name.styled_sub_command()
        );
        println!();
        println!("{}", table);
    }
}
impl SubCommand<'_> {
    pub fn formated_usage(&self, app_name: &str) -> String {
        let command_name = self._name.styled_sub_command();
        let short_name = self._short_name.styled_sub_command();

        let arg_in_usage = match self._arg_type_with_action {
            ArgAction::Empty(_) => "".to_string(),

            // ArgAction::String(_) => format!(r#"{}"#, "String".styled_arg_type()),
            // ArgAction::Number(_) => format!(r#"{}"#, "Number".styled_arg_type()),
            // ArgAction::Path(_) => format!(r#"{}"#, "Path".styled_arg_type()),
            // ArgAction::Bool(_) => format!(r#"{}"#, "Bool".styled_arg_type()),
            ArgAction::String(_) => "String".styled_arg_type().to_string(),
            ArgAction::Number(_) => "Number".styled_arg_type().to_string(),
            ArgAction::Path(_) => "Path".styled_arg_type().to_string(),
            ArgAction::Bool(_) => "Bool".styled_arg_type().to_string(),

            ArgAction::StringMutiple(_) => format!(r#"{}..."#, "String".styled_arg_type()),
            ArgAction::NumberMutiple(_) => format!(r#"{}..."#, "Number".styled_arg_type()),
            ArgAction::PathMutiple(_) => format!(r#"{}..."#, "Path".styled_arg_type()),
            ArgAction::BoolMutiple(_) => format!(r#"{}..."#, "Bool".styled_arg_type()),

            ArgAction::Dialog(_) => "".to_string(),
        };

        // let arg_in_usage = arg_in_usage;

        let a = format!(
            r#"
Usage: 
{app_name} {command_name} {arg_in_usage}"#
        );

        let b = format!(
            r#"
Usage: 
{app_name} {command_name} {arg_in_usage}
{app_name} {short_name} {arg_in_usage}"#
        );

        if self._short_name.is_empty() {
            a
        } else {
            b
        }
    }

    /// `app cmd -h` 时显示的帮助文档.
    pub fn formated_command_help(&self, app_name: &str) -> String {
        if let Some(s) = &self._help_message {
            // 自定义了帮助文档的情况;
            s.to_string()
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg_message: String = if self._arg_type_with_action.arg_message().is_empty() {
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
                Usage = self.formated_usage(app_name),
            );

            message
        }
    }

    pub fn formated_row_in_list_all_command(&self) -> Vec<Row> {
        let x = self;
        let short_name = if x._short_name.is_empty() {
            "".to_string()
        } else {
            format!("{}{}", &x._short_name, ", ")
        };

        let command_name = &x._name;

        let mut result: Vec<Row> = vec![];

        result.push(row![command_name.styled_sub_command(), x._about]);
        if !x._short_name.is_empty() {
            result.push(row![
                short_name.styled_sub_command(),
                format!("alias: {}", command_name)
            ]);
        }

        result
    }

    /// 已经格式化好了, 直接放进 Table 打印就行.
    pub fn formated_command_example(&self, app_name: &str) -> Vec<Row> {
        if self._exaples.is_empty() {
            SingleExample {
                command: "{app_name} {command_name} {arg}",
                description: self._about,
            }
            .formated();

            // let mut table = table! {};
            let mut table: Vec<Row> = vec![];

            {
                // table.set_format(helper::table_formater());

                let arg = self
                    ._arg_type_with_action
                    .value_example()
                    .bright_green()
                    .to_string();

                table.push(row![
                    format!(
                        "{app_name} {command_name} {arg}",
                        command_name = self._name.styled_sub_command(),
                    ),
                    self._about
                ]);
            }

            table
        } else {
            self._exaples.pretty_formated()
        }
    }

    // /// 检查 example 里面的 command 是否能够被正常解析.
    // pub fn check(&self, app_name: &String, cmd_args: &Vec<String>) -> DidHandled {
    //     for x in &self._exaples.val {
    //         let asdf = parse_arg_string(x.command);
    //     }
    //     self.try_run(app_name, cmd_args, false)
    // }

    fn suc_command_try_run(
        &self,
        app_name: &str,
        // cmd_args: &[String],
        cmd_args: SharedVecString,
        need_to_run: bool,
    ) -> DidHandled {
        {
            // 处理当前 子命令 的 flag.
            if let Some(first_arg) = cmd_args.first().cloned() {
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

            let v = SubcommandArgsValue::new(cmd_args);

            let re = match &self._arg_type_with_action {
                ArgAction::Empty(f) => run(v.get_empty(), need_to_run, f),
                ArgAction::String(f) => run(v.get_string(), need_to_run, f),
                ArgAction::StringMutiple(f) => run(v.get_vec_string(), need_to_run, f),
                ArgAction::Number(f) => run(v.get_number(), need_to_run, f),
                ArgAction::NumberMutiple(f) => run(v.get_vec_number(), need_to_run, f),
                ArgAction::Path(f) => run(v.get_path(), need_to_run, f),
                ArgAction::PathMutiple(f) => run(v.get_vec_path(), need_to_run, f),
                ArgAction::Bool(f) => run(v.get_bool(), need_to_run, f),
                ArgAction::BoolMutiple(f) => run(v.get_vec_bool(), need_to_run, f),
                ArgAction::Dialog(f) => run(v.get_repl(), need_to_run, &|s| match s {
                    Some(json_string) => {
                        /* 收到了参数 "stdin" */
                        let re = &mut arg_type::Dialog::new_from_toml(json_string.as_str());
                        match re {
                            Ok(repl) => {
                                f(repl);
                                // repl.finesh(app_name, &self._command_name);
                            }
                            Err(err) => {
                                eprintln!("{}", err);
                                // json_string 解码为 Vec<String> 时发生错误.
                                panic!(
                                    "\n参数不正确.\nsee {} {} {} for more infomation.\n",
                                    app_name.magenta(),
                                    self._name.styled_sub_command(),
                                    "-h".styled_arg()
                                );
                            }
                        }
                    }
                    None => {
                        /* 该子命令没有收到参数, 启动问答式交互 */

                        let mut repl = arg_type::Dialog::new();
                        f(&mut repl);
                        repl.finesh(app_name, self._name);
                    }
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

            fn run<T>(
                result: ParseResult<T>,
                need_run_action: bool,
                func: &dyn Fn(T),
            ) -> DidHandled {
                match result {
                    Ok(s) => {
                        if need_run_action {
                            func(s);
                        }
                        DidHandled::Handled
                    }
                    Err(err) => DidHandled::Failed(err),
                }
            }
        }
    }

    /// 检查 子命令 的名字是否符合要求.
    #[cfg(debug_assertions)] // 只在 debug 模式下使用
    fn command_name_check(name: &str) -> Result<(), String> {
        if name.is_empty() {
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

        let arr = [
            "-h",
            "--help",
            "-e",
            "--example",
            "-v",
            "--version",
            "--list-all-commands",
        ];

        if arr.contains(&name) {
            let msg = format!(
                r#"
{error}: name 不能是 "-h", "--help", "-e", "--example", "-v", "--version", 这些已经有了默认的实现.
    "#,
                error = "error".red(),
            );

            return Err(msg);
        }

        Ok(())
    }
}
