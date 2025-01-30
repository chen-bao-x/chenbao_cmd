use super::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct Command {
    /// 命令名   
    /// 命令的名称长度最好不要超过 20 个字符.
    pub command_name: String,

    /// 命令名的简写形式, 通常是一个字符  
    pub short_name: String,

    /// 一句话介绍此命令
    pub about: String,

    /// 是用此命令的一些示范和例子.
    /// 自动生成帮助文档时会用的这里面的例子.
    pub exaples: Vec<String>,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    pub help_document: String,

    /// 子命令需要的参数的类型.
    /// 在打印子命令的帮助文档时需要用到此属性.
    pub need_arg_type: ArgType,

    /// command action with command_arg
    pub action: Option<CommandAction>,
}

pub type CommandAction = Rc<dyn Fn(SubcommandArgsValue) -> ()>;

impl Command {
    pub fn new(name: &str) -> Self {
        if is_debug_mode() && name == "" {
            eprintln!("WARNING: name 不能是空字符串 \"\", name 的值至少需要一个字符.");
        }

        return Command {
            command_name: name.to_string(),
            about: "".to_string(),
            help_document: "".to_string(),
            action: None,
            // arg_count: ArgCount::Zero,
            short_name: "".to_string(),
            exaples: vec![],
            need_arg_type: ArgType::Empty,
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

    pub fn add_command_example(self, example: &str) -> Self {
        let mut re = self;
        re.exaples.push(example.to_string());
        return re;
    }

    /// set `Command.example`
    pub fn help_document(self, str: &str) -> Self {
        let mut re = self;
        re.help_document = str.to_string();
        return re;
    }

    /// set `Command.action`
    pub fn action<F>(self, need_arg_type: ArgType, action: F) -> Self
    where
        F: Fn(SubcommandArgsValue) -> () + 'static,
    {
        let mut re = self;
        re.need_arg_type = need_arg_type;
        re.action = Some(Rc::new(action));

        return re;
    }

    pub fn print_command_help(&self, app_name: String) {
        if self.help_document != "" {
            // 自定义了帮助文档的情况;
            println!("{}", self.help_document);
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg_message: String = self.need_arg_type.arg_message();

            let arg_in_usage = match self.need_arg_type {
                ArgType::Empty => "",
                ArgType::String => r#""string""#,
                ArgType::VecString => r#""string...""#,
                ArgType::Number => r#"Number"#,
                ArgType::VecNumber => r#"Number..."#,
                ArgType::Path => r#""path""#,
                ArgType::VecPath => r#""path"..."#,
                ArgType::Bool => r#"bool"#,
                ArgType::VecBool => r#"bool..."#,
            };

            // let examples_message = self.print_command_example();

            // TODO: 让打印的信息更优美.
            let flag_message =
                "Flags:\n\n    -h, --help\t\t显示此命令的帮助.\n    -e, --example\t查看示例.\n";

            let message = format!(
                r#"
{about}

Usage: {app_name} {command_name} {arg_in_usage}

Arguments:

    {arg_message}

{flag_message}

"#,
                about = self.about,
                command_name = self.command_name,
            );
            print!("{}", message);
        }
    }

    pub fn print_command_example(&self) -> String {
        // TODO: 让打印的 Example 更优美.

        let example_messae = self.exaples.iter().fold(String::new(), |a, b| a + b + "\n");

        let example_messae = "".to_string()
            + &example_messae
                .lines()
                .map(|line| format!("{}{}\n    --------\n", "    ", line))
                .collect::<String>();

        println!("\n{}", example_messae);

        return example_messae;
    }
}
