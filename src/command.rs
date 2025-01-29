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

    /// command action with command_arg
    pub action: Option<(ArgCount, CommandAction)>,
}

pub type CommandAction = Rc<dyn Fn(Vec<String>) -> ()>;

impl Command {
    /// Creates a new [`Command`].
    /// name:
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

    pub fn add_example(self, example: &str) -> Self {
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
    pub fn action<F>(self, arg_count: ArgCount, action: F) -> Self
    where
        F: Fn(Vec<String>) -> () + 'static,
    {
        let mut re = self;
        re.action = Some((arg_count, Rc::new(action)));

        return re;
    }

    pub fn print_command_help(&self, app_name: String) {
        // debug_run(|| {
        //     println!("command  print_help_message");
        // });

        if self.help_document != "" {
            // 自定义了帮助文档的情况;
            println!("{}", self.help_document);
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg: String = if let Some((arg_count, _)) = &self.action {
                match arg_count {
                    ArgCount::Zero => "".to_string(),
                    ArgCount::One => "argument # 需要 1 个参数".to_string(),
                    ArgCount::ZeroOrOne => "[argument] # 需要 [0 个 或 1 个] 参数".to_string(),
                    ArgCount::ZoreOrMany => "[arguments...] -- ".to_string(),
                    ArgCount::OneOrMany => "<arguments...> -- zore or many argument".to_string(),
                    ArgCount::Count(count) => {
                        let mut i = 0;
                        let mut re: String = "".to_string();
                        while i < *count {
                            re.push_str(" argument");
                            i += 1;
                        }

                        re
                    }
                }
            } else {
                "".to_string()
            };

            // let short_name_message = if self.short_name == "" {
            //     "".to_string()
            // } else {
            //     "\nshort_name: ".to_string() + &self.short_name + "\n"
            // };
            let examples_message = if self.exaples.is_empty() {
                "".to_string()
            } else {
                let asdf = self.exaples.iter().fold(String::new(), |a, b| a + b + "\n");
                "\nExamples: \n".to_string() + &asdf + "\n"
            };

            let message = format!(
                r#"
{about}

Usage: {app_name} {command_name} {arg}

Arguments:

Flags:
    -h, --help
    -v, --version
{examples_message}

"#,
                about = self.about,
                command_name = self.command_name,
            );
            print!("{}", message);
        }
    }

    pub fn print_command_example(&self) -> String {
        let example_messae = self.exaples.iter().fold(String::new(), |a, b| a + b);
        println!("\n{}", example_messae);

        return example_messae;
    }
}
