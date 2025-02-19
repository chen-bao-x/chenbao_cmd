//!
//!
//!
//! # 命令行程序命令设计规范:
//!  ternary
//! 现在的命令更多的是人类手动在 Terminal 中输入, 此规范主要是为了让人类在手动输入命令行指令时能 更轻松 更愉快 更容易输入正确的指令.
//!
//! > 如今，尽管许多 CLI 程序主要(甚至专门)是被人类使用，而不是被程序调用，但它们的许多交互设计仍然承载着过去的包袱。 现在是时候摆脱这些历史包袱了: 如果一个命令主要是被人类所使用的，而不是程序，那么它就应该首先以人为本设计。
//! > from: <https://sunbk201.github.io/cli-guidelines-zh/>
//!
//! ## rules:  
//! 1. 三元命令风格:
//! 2. 如果是多个 “参数” 并且每个参数的类型相同:
//! 3. 每个子命令的有的 flags: -h -e
//! 4. app 自带的 flags: -h -e -v
//! 5. 如果 “参数” 数量非常多, 类型各有不同, 建议使用 “交互式问答” 的形式来让用户能够 愉快的 轻松的 正确的 完成参数的填写, Don't let your user Panic.
//!
//!   
//! ### 1. 三元命令风格:
//! 程序名 子命令 参数
//! ```sh
//! cargo new "./folder/projectName"
//! ```
//!
//! 参数可以省略:
//! ```sh
//! cargo init
//! ```
//!
//! 只有程序名的情况也是可以的:
//!
//! 1. 则可以打印帮助信息, 例如:
//! ```sh
//! cargo
//! ```
//!
//! 2. 也可以直接执行, 例如:
//! ```sh
//! ls
//! ```
//!
//!
//! ### 2. 如果是多个 “参数” 并且每个参数的类型相同:
//! 备注: 很多时候 “参数” 默认被当作 字符串, 还是建议使用 半角双引号 包裹起来.
//! ```sh
//! cargo add packageA packageB packageC
//!
//! # 效果应该等同于:
//! cargo add packageA
//! cargo add packageB
//! cargo add packageC
//!
//! ```
//!
//! ### 3. 每个子命令都有的 flags:
//! ```sh
//! app subcommand -h           # 查看帮助文档
//! app subcommand --help       # 查看帮助文档
//!
//!
//! app subcommand -e           # 查看示例
//! app subcommand --example    # 查看示例
//! ```
//!
//! ### 4. app 自带的 flags:
//! ```sh
//! app -h                      # 查看帮助文档
//! app --help                  # 查看帮助文档
//!
//! app -v                      # 查看 app 的版本信息
//! app --version               # 查看 app 的版本信息
//!
//! app -e                      # 查看示例
//! app --example               # 查看示例
//! ```
//!
//!
//! ### 5. 如果 “参数” 数量非常多, 类型各有不同, 建议使用 “交互式问答” 的形式来让用户能够 愉快的 轻松的 正确的 完成参数的填写, Don't let your user Panic.
//! 方案一:
//! ```sh
//! git commit --repl
//! > did you want commit all changed files(y/n)?
//! y
//! > input commit message:
//! 新增了某某功能.
//! > runing command: git commit "all" = "true", message = "添加了一个新功能"  # for API.
//!
//! > runing command: git.commit().all().message("新增了某某功能.")
//!
//! ```
//!
//!

// ------- Public -------

pub use action::Arg;
pub use application::App;
pub use application::DidHandled;
pub use chenbao_cmd_macro::cmd;
pub mod arg_type;
pub use subcommand::SubCommand;

// ------- Private -------

mod action;
mod application;
mod examples_types;
mod helper;
mod question_and_anser;
mod subcommand;

pub(crate) type SharedString = std::rc::Rc<String>;
pub(crate) type SharedVecString = std::rc::Rc<Vec<String>>;
