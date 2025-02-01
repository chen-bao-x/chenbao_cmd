use super::*;
use owo_colors::OwoColorize;
use prettytable::{row, table, Row};
use std::rc::Rc;

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
    pub exaples: Option<Examples>,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    pub help_document: String,

    /// 子命令需要的参数的类型.
    /// 在打印子命令的帮助文档时需要用到此属性.
    pub need_arg_type: ArgType,
    // /// command action with command_arg
    // pub action: Option<CommandAction>,
}

pub type CommandAction = Rc<dyn Fn(SubcommandArgsValue) -> ()>;

impl SubCommand {
    pub fn new(name: &str) -> Self {
        if is_debug_mode() && name == "" {
            eprintln!("WARNING: name 不能是空字符串 \"\", name 的值至少需要一个字符.");
        }

        return SubCommand {
            command_name: name.to_string(),
            about: "".to_string(),
            help_document: "".to_string(),
            // action: None,
            // arg_count: ArgCount::Zero,
            short_name: "".to_string(),
            exaples: None,
            need_arg_type: ArgType::Empty(Rc::new(|| {})),
        };
    }

    /// set `Command.short_name`
    pub fn short_name(self, short_name: &str) -> Self {
        let mut re = self;
        re.short_name = short_name.to_string();
        return re;
    }

    /// set `Command. about`
    pub fn about(self, about: &str) -> Self {
        let mut re = self;
        re.about = about.to_string();
        return re;
    }

    pub fn add_command_example(self, example: Option<Examples>) -> Self {
        let mut re = self;
        re.exaples = example;
        return re;
    }

    /// set `Command.example`
    pub fn help_document(self, str: &str) -> Self {
        let mut re = self;
        re.help_document = str.to_string();
        return re;
    }

    // /// set `Command.action`
    // pub fn action<F>(self, need_arg_type: ArgType, action: F) -> Self
    // where
    //     F: Fn(SubcommandArgsValue) -> () + 'static,
    // {
    //     let mut re = self;
    //     re.need_arg_type = need_arg_type;
    //     re.action = Some(Rc::new(action));

    //     return re;
    // }
    /// set `Command.action`
    pub fn action(self, need_arg_type: ArgType) -> Self {
        let mut re = self;
        re.need_arg_type = need_arg_type;

        return re;
    }

    pub fn formated_command_help(&self, app_name: String) -> String {
        if self.help_document != "" {
            // 自定义了帮助文档的情况;
            return format!("{}", self.help_document);
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg_message: String = self.need_arg_type.arg_message();

            let arg_in_usage = match self.need_arg_type {
                ArgType::Empty(_) => "",
                ArgType::String(_) => r#""string""#,
                ArgType::VecString(_) => r#""string...""#,
                ArgType::Number(_) => r#"Number"#,
                ArgType::VecNumber(_) => r#"Number..."#,
                ArgType::Path(_) => r#""path""#,
                ArgType::VecPath(_) => r#""path"..."#,
                ArgType::Bool(_) => r#"bool"#,
                ArgType::VecBool(_) => r#"bool..."#,
                ArgType::Repl(_) => "",
            };
            let arg_in_usage = arg_in_usage.magenta();

            // let examples_message = self.print_command_example();

            let help = format!("{}, {}", "-h".cyan(), "--help".cyan());
            let example = format!("{}, {}", "-e".cyan(), "--example".cyan());
            let flag_message =
                format!("Flags:\n\n    {help}\t\t显示此命令的帮助.\n    {example}\t查看示例.\n");

            let message = format!(
                r#"
{about}

Usage: {app_name} {command_name} {arg_in_usage}

Arguments:

{arg_message}

{flag_message}

"#,
                about = self.about,
                command_name = self.command_name.cyan(),
            );

            return message;
        }
    }

    pub fn print_command_help(&self, app_name: String) {
        println!("{}", self.formated_command_help(app_name));
    }

    /// 已静格式化好了, 直接放进 Table 打印就行.
    pub fn formated_command_example(&self, app_name: String) -> Vec<prettytable::Row> {
        match &self.exaples {
            Some(s) => {
                println!("{}", s);

                let mut re: Vec<Row> = vec![];
                re.push(row![s]);

                return re;
            }
            None => {
                let arg = self.need_arg_type.value_example().green().to_string();

                let r = row![
                    format!(
                        "{app_name} {command_name} {arg}",
                        command_name = self.command_name.cyan(),
                    ),
                    self.about
                ];

                return vec![r];
            }
        }
    }

    pub fn print_command_example(&self, app_name: String) {
        let arr = self.formated_command_example(app_name);
        let mut table = table!();
        table.set_format(table_formater());
        for x in arr {
            table.add_row(x);
        }

        println!("{}", table);
    }
}
