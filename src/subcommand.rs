use std::{default, vec};

use crate::{
    action::{Arg, ParseResult, SubcommandArgsValue},
    application::NeedTo,
    examples_types::{Examples, SingleExample},
    helper::*,
};

use super::*;
use application::DidHandled;
use owo_colors::OwoColorize;
use prettytable::{
    format::{LinePosition, LineSeparator, TableFormat},
    row, table, Row, Table,
};

/// 子命令
#[derive(Clone, Debug)]
pub struct SubCommand {
    /// 子命令名
    /// 命令的名称长度最好不要超过 20 个字符.
    // pub _cmd_name: &'a str,
    pub _cmd_name: String,

    /// 命令名的简写形式, 通常是一个字符
    // pub _short_name: &'a str,
    pub _short_name: String,

    /// 一句话介绍此命令
    // pub _about: &'a str,
    pub _about: String,

    /// 是用此命令的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub _exaples: Examples,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    // pub _help_message: Option<&'a str>,
    pub _help_message: Option<String>,

    /// 子命令需要的参数的类型以及该子命令的 action.
    /// 在打印子命令的帮助文档时需要用到此属性.
    pub _arg_type_with_action: Arg,
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
    ///     cmd!("build")
    ///         .short_name("b")
    ///         .about("编译项目")
    ///         .action(Arg::Bool(&|_x| {
    ///             print!("command \"run\"{:?}\n", _x);
    ///         }));
    /// ```
    pub fn create_an_sub_command(name: &str) -> Self {
        SubCommand {
            _cmd_name: name.to_owned(),

            _about: String::new(),
            _help_message: None,
            _short_name: "".to_owned(),
            _exaples: Examples::new(),
            _arg_type_with_action: Arg::default(),
        }
    }
}

impl<'a> SubCommand {
    /// set `Command.short_name`
    // pub fn short_name(self, short_name: &'a str) -> Self {
    pub fn short_name(mut self, short_name: &'a str) -> Self {
        self._short_name = short_name.to_string();
        self
    }

    /// set `SubCommand.about`
    pub fn about(self, about: &'a str) -> Self {
        let mut re = self;
        re._about = about.to_owned();
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
        re._help_message = Some(str.to_owned());

        re
    }

    /// set `Command.action`
    pub fn action(self, need_arg_type: Arg) -> Self {
        let mut re = self;
        re._arg_type_with_action = need_arg_type;

        re
    }

    pub fn sub_command_run(&self, app_name: &str, cmd_args: SharedVecString) -> DidHandled {
        self.sub_command_try_run(app_name, cmd_args, NeedTo::Run)
    }
}

impl SubCommand {
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
            self._cmd_name.styled_sub_command()
        );
        println!();
        println!("{}", table);
    }
}

impl<'a> SubCommand {
    pub fn formated_usage(&self, app_name: &str) -> String {
        let command_name = self._cmd_name.bright_cyan();
        let short_name = self._short_name.bright_cyan();

        let arg_in_usage = match self._arg_type_with_action {
            Arg::Empty(_) => "".to_string(),
            Arg::String(_) => "String".styled_arg_type().to_string(),
            Arg::Number(_) => "Number".styled_arg_type().to_string(),
            Arg::Path(_) => "Path".styled_arg_type().to_string(),
            Arg::Bool(_) => "Bool".styled_arg_type().to_string(),

            Arg::StringMutiple(_) => format!(r#"{}..."#, "String".styled_arg_type()),
            Arg::NumberMutiple(_) => format!(r#"{}..."#, "Number".styled_arg_type()),
            Arg::PathMutiple(_) => format!(r#"{}..."#, "Path".styled_arg_type()),
            Arg::BoolMutiple(_) => format!(r#"{}..."#, "Bool".styled_arg_type()),

            Arg::Dialog(_) => "".to_string(),
        };

        // let arg_in_usage = arg_in_usage;
        let app_name = app_name.cyan();
        let a = format!(
            r#"
{usg}:
    {app_name} {command_name} {arg_in_usage}"#,
            usg = "Usage".bright_green(),
        );

        let b = format!(
            r#"
{usg}:
    {app_name} {command_name} {arg_in_usage}
    {app_name} {short_name} {arg_in_usage}"#,
            usg = "Usage".bright_green(),
        );

        if self._short_name.is_empty() {
            a
        } else {
            b
        }
    }

    /// 自动生成的 子命令帮助文档.
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

        let command_name = &x._cmd_name;

        let mut result: Vec<Row> = vec![];

        result.push(row![command_name.styled_sub_command(), x._about]);
        if !x._short_name.is_empty() {
            result.push(row![
                short_name.styled_sub_command(),
                format!("alias: {}", command_name.styled_sub_command())
            ]);
        }

        result
    }

    /// 已经格式化好了, 直接放进 Table 打印就行.
    pub fn formated_command_example(&self, app_name: &str) -> Vec<Row> {
        if self._exaples.is_empty() {
            _ = app_name;
            vec![]

            //  自动生成的示例效果不好, 先不自动生成.
            // {
            //     let mut table: Vec<Row> = vec![];
            //     {
            //         let arg = self
            //             ._arg_type_with_action
            //             .value_example()
            //             .bright_green()
            //             .to_string();

            //         table.push(row![
            //             format!(
            //                 "{app_name} {command_name} {arg}",
            //                 command_name = self._name.styled_sub_command(),
            //             ),
            //             self._about
            //         ]);
            //     }

            //     table
            // }
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

    /// 尝试执行此 子命令.
    /// 如果  need_to == NeedTo::ParseOnly, 则只检查不执行 action.
    pub(crate) fn sub_command_try_run(
        &self,
        app_name: &str,
        cmd_args: SharedVecString,
        need_to: NeedTo,
    ) -> DidHandled {
        {
            // 处理当前 子命令 的 flag.
            if let Some(first_arg) = cmd_args.first().cloned() {
                // 处理当前子命令的 help flag.
                if first_arg == "--help" || first_arg == "-h" {
                    if need_to.is_run() {
                        self.print_command_help(app_name);
                    }
                    return DidHandled::Handled;
                }

                // 处理当前子命令的 example flag.
                if first_arg == "--example" || first_arg == "-e" {
                    if need_to.is_run() {
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
                Arg::Empty(f) => run(v.get_empty(), need_to, f),
                Arg::String(f) => run(v.get_string(), need_to, f),
                Arg::StringMutiple(f) => run(v.get_vec_string(), need_to, f),
                Arg::Number(f) => run(v.get_number(), need_to, f),
                Arg::NumberMutiple(f) => run(v.get_vec_number(), need_to, f),
                Arg::Path(f) => run(v.get_path(), need_to, f),
                Arg::PathMutiple(f) => run(v.get_vec_path(), need_to, f),
                Arg::Bool(f) => run(v.get_bool(), need_to, f),
                Arg::BoolMutiple(f) => run(v.get_vec_bool(), need_to, f),
                Arg::Dialog(f) => run(v.get_repl(), need_to, &|s| {
                    match s {
                        Some(json_string) => {
                            /* 收到了参数 "stdin" */
                            let re = &mut arg_type::Dialog::new_from_toml(json_string.as_str());
                            match re {
                                Ok(repl) => {
                                    f(repl); // repl.finesh_and_print(app_name, self._name);
                                }
                                Err(err) => {
                                    eprintln!("{}", err);
                                    // json_string 解码为 Vec<String> 时发生错误.
                                    panic!(
                                        "\n参数不正确.\nsee {} {} {} for more infomation.\n",
                                        app_name.magenta(),
                                        self._cmd_name.styled_sub_command(),
                                        "-h".styled_arg()
                                    );
                                }
                            }
                        }
                        None => {
                            /* 该子命令没有收到参数, 启动问答式交互 */

                            let mut repl = arg_type::Dialog::new();
                            f(&mut repl);
                            repl.finesh_and_print(app_name, &self._cmd_name);
                        }
                    }
                }),
            };

            if let DidHandled::Failed(err) = re {
                let tips = format!(
                    "输入  {} {} {}  查看更详细信息.",
                    app_name.styled_sub_command(),
                    self._cmd_name.styled_sub_command(),
                    "-h".styled_sub_command(),
                );
                return DidHandled::Failed(format!(
                    r#"
{}

{}

{tips}
                "#,
                    err, arg_message,
                ));
            }

            return re;

            fn run<T>(
                result: ParseResult<T>,
                // need_run_action: bool,
                need_to: NeedTo,
                func: &dyn Fn(T),
            ) -> DidHandled {
                match result {
                    Ok(s) => {
                        if need_to.is_run() {
                            func(s);
                        }
                        DidHandled::Handled
                    }
                    Err(err) => DidHandled::Failed(err),
                }
            }
        }
    }

    /// 检查 是否是此子命令的命令和参数.
    pub(crate) fn sub_command_try_parse(
        &self,
        app_name: &str,
        cmd_args: SharedVecString,
    ) -> DidHandled {
        {
            // 处理当前 子命令 的 flag.
            if let Some(first_arg) = cmd_args.first().cloned() {
                // 处理当前子命令的 help flag.
                if first_arg == "--help" || first_arg == "-h" {
                    return DidHandled::Handled;
                }

                // 处理当前子命令的 example flag.
                if first_arg == "--example" || first_arg == "-e" {
                    return DidHandled::Handled;
                }
            }
        }

        {
            let arg_message = self._arg_type_with_action.arg_message();

            let v = SubcommandArgsValue::new(cmd_args);

            let re = match &self._arg_type_with_action {
                Arg::Empty(_f) => run(v.get_empty()),
                Arg::String(_f) => run(v.get_string()),
                Arg::StringMutiple(_f) => run(v.get_vec_string()),
                Arg::Number(_f) => run(v.get_number()),
                Arg::NumberMutiple(_f) => run(v.get_vec_number()),
                Arg::Path(_f) => run(v.get_path()),
                Arg::PathMutiple(_f) => run(v.get_vec_path()),
                Arg::Bool(_f) => run(v.get_bool()),
                Arg::BoolMutiple(_f) => run(v.get_vec_bool()),
                Arg::Dialog(_f) => run(v.get_repl()),
            };

            return re.map_err(|err| {
                let _ = format!(
                    "输入  {} {} {}  查看更详细信息.",
                    app_name.styled_sub_command(),
                    self._cmd_name.styled_sub_command(),
                    "-h".styled_sub_command(),
                );
                format!(
                    r#"
{}

{}
                "#,
                    // "error: ".bright_red(),
                    err,
                    arg_message,
                )
            });

            fn run<T>(result: ParseResult<T>) -> DidHandled {
                match result {
                    Ok(_s) => DidHandled::Handled,
                    Err(err) => DidHandled::Failed(err),
                }
            }
        }
    }

    /// 测试命令是否能够被匹配
    pub(crate) fn debug_cmd_example_check(&'a self, app_name: &str) -> ExampleTestResult<'a> {
        let mut bad_examples = ExampleTestResult::new(self);

        for exam in &self._exaples.val {
            let mut wait_to_putsh = Sadadsf {
                base: exam,
                err_msg: vec![],
            };

            let cmd_arg = {
                let mut virtual_env_args = helper::parse_arg_string(&exam.command);
                if !virtual_env_args.is_empty() {
                    let name = virtual_env_args.remove(0); // 移除 app name
                    if app_name != name {
                        let err_msg = format!(
                            "{}: 需要 {}; 实际收到的: {:?}",
                            "程序名称错误".bright_red(),
                            app_name.styled_sub_command(),
                            name
                        );

                        wait_to_putsh.err_msg.push(err_msg);
                    }
                }
                if !virtual_env_args.is_empty() {
                    // 子命令的名字
                    let name = virtual_env_args.remove(0); // 移除 子命令的名字

                    if self._cmd_name != name && self._short_name != name {
                        let err_msg = format!(
                            "{}: 需要 {} 实际收到的: {:?}",
                            "子命令名称错误".bright_red(),
                            self._cmd_name.styled_sub_command(),
                            name
                        );
                        wait_to_putsh.err_msg.push(err_msg);
                    }
                }

                virtual_env_args
            };

            let re = self.sub_command_try_parse(app_name, cmd_arg.into());
            match re {
                DidHandled::Handled => bad_examples.success_examples.push(exam),
                DidHandled::Failed(err_msg) => {
                    wait_to_putsh.err_msg.push(err_msg);
                }
            }
            if !wait_to_putsh.err_msg.is_empty() {
                // 如果有 错误信息, 则这个这是一个 failures_example.
                bad_examples.failures_examples.push(wait_to_putsh);
            }
        }

        bad_examples
    }
}

pub(crate) struct ExampleTestResult<'a> {
    cmd: &'a SubCommand,
    failures_examples: Vec<Sadadsf<'a>>,
    success_examples: Vec<&'a SingleExample>,
}

impl<'a> ExampleTestResult<'a> {
    pub fn new(cmd: &'a SubCommand) -> Self {
        Self {
            cmd,
            failures_examples: vec![],
            success_examples: vec![],
        }
    }
    pub fn is_success(&self) -> bool {
        self.failures_examples.is_empty()
    }

    pub fn formated_massage(&self) -> String {
        let ok = if self.is_success() {
            "ok".green().to_string()
        } else {
            "FAILED".red().to_string()
        };

        let msgs: String = self
            .failures_examples
            .iter()
            .map(|x| x.formated_whit_err_msg().to_string())
            .fold("".to_string(), |x, y| x + &y);

        let mut table = table!();
        {
            let mut f = TableFormat::new();
            f.padding(2, 0);
            f.separator(LinePosition::Bottom, LineSeparator::new('─', 'j', '└', 'r'));
            // f.separator(LinePosition::Title, LineSeparator::new('─', 'j', '├', 'r'));
            f.separator(LinePosition::Title, LineSeparator::new('━', 'j', '┝', 'r'));
            f.left_border('│');
            table.set_format(f);
        }
        table.add_row(row![msgs]);

        /* return */
        if self.is_success() {
            format!(
                r#"example test for {cmd_name} ... {ok}
"#,
                cmd_name = self.cmd._cmd_name.styled_sub_command(),
            )
        } else {
            let title = format!(
                r#"example test for {cmd_name} ... {ok}"#,
                cmd_name = self.cmd._cmd_name.styled_sub_command(),
            );
            table.set_titles(row![title]);
            format!("\n{}", table)
        }
    }
}

/// 存储一个 SingleExample 中出现的多个错误.
pub(crate) struct Sadadsf<'a> {
    base: &'a SingleExample,

    err_msg: Vec<String>,
}

impl Sadadsf<'_> {
    fn formated_whit_err_msg(&self) -> String {
        let msg = self.err_msg.join("");
        format!("{}{}", self.base.formated(), msg)
    }
}
// example test for "run" ... ok
// example test "init" ... FAILED
//      app build 2 # this can not parse.
// example test "build" ... FAILED
//      app build 2 # this can not parse.

pub(crate) struct ErrorTable {
    pub title: Row,
    pub err_messages: Vec<Row>,
    pub format: TableFormat,
}

impl default::Default for ErrorTable {
    fn default() -> Self {
        let mut f = TableFormat::new();
        {
            f.separator(LinePosition::Bottom, LineSeparator::new('─', '─', '└', 'r'));
            // f.separator(LinePosition::Title, LineSeparator::new('─', 'j', '├', 'r'));
            f.separator(LinePosition::Title, LineSeparator::new('━', '━', '┝', 'r'));
            f.left_border('│');

            f.column_separator(' ');
            f.padding(0, 4);
        }

        Self {
            title: Default::default(),
            err_messages: Default::default(),
            format: f,
        }
    }
}

impl ErrorTable {
    // pub fn new(title: String, err_messages: Vec<String>) -> Self {
    //     Self {
    //         title: row![title],
    //         err_messages: err_messages.iter().map(|s| row![s]).collect(),
    //         ..Default::default()
    //     }
    // }

    pub fn generate_table(&self) -> Table {
        let mut table = table!();
        table.set_format(self.format);
        table.set_titles(self.title.clone());

        for x in self.err_messages.clone() {
            table.add_row(x);
        }
        table
    }

    // pub fn print(&self) {
    //     println!("{}", self.generate_table())
    // }

    // pub fn set_title(&mut self, title: Row) {
    //     self.title = title
    // }
}
