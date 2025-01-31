use owo_colors::OwoColorize;

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
    // /// command action with command_arg
    // pub action: Option<CommandAction>,
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
            // action: None,
            // arg_count: ArgCount::Zero,
            short_name: "".to_string(),
            exaples: vec![],
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

    pub fn formated_command_example(&self, app_name: String) -> String {
        // TODO: 让打印的 Example 更优美.

        if self.exaples.is_empty() {
            // 自动生成一条 example
            // self.command_name;
            // self.need_arg_type;
            // app_name;
            let asdf = match self.need_arg_type {
                ArgType::Empty(_) => "",
                ArgType::String(_) => r#""thid is an string example.""#,
                ArgType::VecString(_) => r#""str 1" "str 2" "str 3"#,
                ArgType::Number(_) => r#"9"#,
                ArgType::VecNumber(_) => r#"5 9 100 12"#,
                ArgType::Path(_) => r#""./path/to/folder/or/file.txt""#,
                ArgType::VecPath(_) => r#""./path 1" "/path/2/" "./" "path3.txt""#,
                ArgType::Bool(_) => r#"true"#,
                ArgType::VecBool(_) => r#"true false"#,
                ArgType::Repl(_) => "",
            };

            let re = format!(
                "    {app_name} {command_name} {arg}\t# {about}\n",
                command_name = self.command_name.cyan(),
                arg = asdf.green(),
                about = self.about,
            );
            // println!("formated_command_example(): {}", re);
            return re;
        } else {
            // println!("formated_command_example() self.examples: {:?}", self.exaples);

            let example_messae = self.exaples.iter().fold(String::new(), |a, b| a + b + "\n");

            let example_messae = "".to_string()
                + &example_messae
                    .lines()
                    .map(|line| format!("{}{}\n", "    ", line))
                    .collect::<String>();

            // println!("\n{}", example_messae);

            return example_messae;
        }
    }

    pub fn print_command_example(&self, app_name: String) {
        println!("{}", self.formated_command_example(app_name));
    }
}

// pub struct SubcommandAction {
//     empty: Option<Rc<dyn Fn() -> ()>>,
//     string: Option<Rc<dyn Fn(String) -> ()>>,
//     vec_string: Option<Rc<dyn Fn(Vec<String>) -> ()>>,
//     number: Option<Rc<dyn Fn(Number) -> ()>>,
//     vec_number: Option<Rc<dyn Fn(Vec<Number>) -> ()>>,
//     path: Option<Rc<dyn Fn(Rc<arg_types::Path>) -> ()>>,
//     vec_path: Option<Rc<dyn Fn(Vec<Rc<arg_types::Path>>) -> ()>>,
//     bool: Option<Rc<dyn Fn(bool) -> ()>>,
//     vec_bool: Option<Rc<dyn Fn(Vec<bool>) -> ()>>,
//     repl: Option<Rc<dyn Fn(Option<String>) -> ()>>,
// }

// impl SubcommandAction {
//     pub fn new() -> Self {
//         Self {
//             empty: None,
//             string: None,
//             vec_string: None,
//             number: None,
//             vec_number: None,
//             path: None,
//             vec_path: None,
//             bool: None,
//             vec_bool: None,
//             repl: None,
//         }
//     }

//     pub fn Empty<F>(self, f: F) -> Self
//     where
//         F: Fn() -> () + 'static,
//     {
//         let mut re = self;
//         re.empty = Some(Rc::new(f));
//         return re;
//     }
//     pub fn string<F>(self, f: F) -> Self
//     where
//         F: Fn(String) -> () + 'static,
//     {
//         let mut re = self;
//         re.string = Some(Rc::new(f));
//         return re;
//     }
//     pub fn VecString<F>(self, f: F) -> Self
//     where
//         F: Fn(Vec<String>) -> () + 'static,
//     {
//         let mut re = self;
//         re.vec_string = Some(Rc::new(f));
//         return re;
//     }
//     pub fn Number<F>(self, f: F) -> Self
//     where
//         F: Fn(Number) -> () + 'static,
//     {
//         let mut re = self;
//         re.number = Some(Rc::new(f));
//         return re;
//     }
//     pub fn VecNumber<F>(self, f: F) -> Self
//     where
//         F: Fn(Vec<Number>) -> () + 'static,
//     {
//         let mut re = self;
//         re.vec_number = Some(Rc::new(f));
//         return re;
//     }
//     pub fn Path<F>(self, f: F) -> Self
//     where
//         F: Fn(Rc<arg_types::Path>) -> () + 'static,
//     {
//         let mut re = self;
//         re.path = Some(Rc::new(f));
//         return re;
//     }
//     pub fn VecPath<F>(self, f: F) -> Self
//     where
//         F: Fn(Vec<Rc<arg_types::Path>>) -> () + 'static,
//     {
//         let mut re = self;
//         re.vec_path = Some(Rc::new(f));
//         return re;
//     }
//     pub fn Bool<F>(self, f: F) -> Self
//     where
//         F: Fn(bool) -> () + 'static,
//     {
//         let mut re = self;
//         re.bool = Some(Rc::new(f));
//         return re;
//     }
//     pub fn VecBool<F>(self, f: F) -> Self
//     where
//         F: Fn(Vec<bool>) -> () + 'static,
//     {
//         let mut re = self;
//         re.vec_bool = Some(Rc::new(f));
//         return re;
//     }
//     pub fn Repl<F>(self, f: F) -> Self
//     where
//         F: Fn() -> () + 'static,
//     {
//         let mut re = self;
//         re.empty = Some(Rc::new(f));
//         return re;
//     }
// }

// enum asdfasdfads {
//     Empty(Rc<dyn Fn() -> ()>),
//     String(Rc<dyn Fn(String) -> ()>),
//     VecString(Rc<dyn Fn(Vec<String>) -> ()>),
//     Number(Rc<dyn Fn(Number) -> ()>),
//     VecNumber(Rc<dyn Fn(Vec<Number>) -> ()>),
//     Path(Rc<dyn Fn(Rc<arg_types::Path>) -> ()>),
//     VecPath(Rc<dyn Fn(Vec<Rc<arg_types::Path>>) -> ()>),
//     Bool(Rc<dyn Fn(bool) -> ()>),
//     VecBool(Rc<dyn Fn(Vec<bool>) -> ()>),
//     Repl(Rc<dyn Fn(Option<String>) -> ()>),
// }

// fn asdfasdf(t: asdfasdfads) {
//     match t {
//         asdfasdfads::Empty(_f) => _f(),
//         asdfasdfads::String(_f) => _f(),
//         asdfasdfads::VecString(_f) => _f(),
//         asdfasdfads::Number(_f) => _f(),
//         asdfasdfads::VecNumber(_f) => _f(),
//         asdfasdfads::Path(_f) => _f(),
//         asdfasdfads::VecPath(_f) => _f(),
//         asdfasdfads::Bool(_f) => _f(),
//         asdfasdfads::VecBool(_f) => _f(),
//         asdfasdfads::Repl(_f) => _f(),
//     }
// }
