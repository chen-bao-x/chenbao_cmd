use super::*;

pub struct Command {
    /// 命令名   
    /// 命令的名称长度最好不要超过 20 个字符.
    pub command_name: &'static str,

    /// 命令名的简写形式, 通常是一个字符  
    pub short_name: &'static str,

    /// 一句话介绍此命令
    pub about: &'static str,

    /// 自定义的帮助文档.
    /// 当用户使用 help 命令查询此命令时显示的帮助文档.
    pub help_document: &'static str,

    /// command action with command_arg
    pub action: Option<(ArgCount, CommandAction)>,
}

pub type CommandAction = Box<dyn Fn(Vec<String>) -> ()>;

impl Command {
    /// Creates a new [`Command`].
    /// name:
    pub fn new(name: &'static str) -> Self {
        if is_debug_mode() && name == "" {
            eprintln!("WARNING: name 不能是空字符串 \"\", name 的值至少需要一个字符.");
        }

        return Command {
            command_name: name,
            about: "",
            help_document: "",
            action: None,
            // arg_count: ArgCount::Zero,
            short_name: "",
        };
    }

    /// set `Command.short_name`
    pub fn short_name(self, short_name: &'static str) -> Self {
        let mut re = self;
        re.short_name = short_name;
        return re;
    }

    /// set `Command. about`
    pub fn about(self, about: &'static str) -> Self {
        let mut re = self;
        re.about = about;
        return re;
    }

    /// set `Command.example`
    pub fn help_document(self, str: &'static str) -> Self {
        let mut re = self;
        re.help_document = str;
        return re;
    }

    /// set `Command.action`
    pub fn action<F>(self, arg_count: ArgCount, action: F) -> Self
    where
        F: Fn(Vec<String>) -> () + 'static,
    {
        let mut re = self;
        re.action = Some((arg_count, Box::new(action)));

        return re;
    }

    pub fn print_command_help(&self, app_name: String) {
        println!("command  print_help_message");
        if self.help_document != "" {
            // 自定义了帮助文档的情况;
            println!("{}", self.help_document);
        } else {
            // 自动生成这个 Command 的帮助文档

            let arg: String = if let Some((arg_count, _)) = &self.action {
                match arg_count {
                    ArgCount::Zero => "".to_string(),
                    ArgCount::One => "argument -- one argument".to_string(),
                    ArgCount::ZeroOrOne => "[argument] -- zore or one argument".to_string(),
                    ArgCount::ZoreOrMany => "[arguments...] -- zore or many argument".to_string(),
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

            let message = format!(
                r#"
{about}
Usage:
    {app_name} {command_name} {arg}
version: {command_name}

commands:
{short_name}
"#,
                about = self.about,
                command_name = self.command_name,
                short_name = self.short_name,
            );
            print!("{}", message);
        }
    }
    // /// set `Command.arg_count`
    // pub fn arg_count(self, arg_count: ArgCount) -> Self {
    //     let mut re = self;
    //     re.arg_count = arg_count;
    //     return re;
    // }
}
