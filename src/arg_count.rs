// use super::DidHandled;

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub enum ArgCount {
//     /// 没有参数
//     Zero,

//     /// 参数可以不填写
//     ZeroOrOne,

//     /// 参数可以不填写, 也可以填入一个或者很多个.
//     ZoreOrMany,

//     /// 能且只能填写一个参数.
//     One,

//     /// 可以填入一个或者很多个.
//     OneOrMany,

//     /// 填入指定数量的参数.
//     Count(u8), // 严格限定数量这种做法太少见, 先不提供这个功能.
// }

// impl std::fmt::Display for ArgCount {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let str = match self {
//             ArgCount::Zero => "ArgCount::Zero".to_string(),
//             ArgCount::One => "ArgCount::One".to_string(),
//             ArgCount::ZeroOrOne => "ArgCount::ZeroOrOne".to_string(),
//             ArgCount::ZoreOrMany => "ArgCount::ZoreOrMany".to_string(),
//             ArgCount::OneOrMany => "ArgCount::OneOrMany".to_string(),
//             ArgCount::Count(c) => {
//                 format!("ArgCount::Count({c})")
//             }
//         };

//         return write!(f, "{}", str);
//     }
// }

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub struct WhenFiledTips {
//     pub str: String,
// }

// impl WhenFiledTips {
//     pub fn new(app_name: &str, sub_command_name: &str) -> Self {
//         Self {
//             str: format!(
//                 r#"
// 输入: {app_name} {sub_command_name} -e 查看示例

// 输入: {app_name} {sub_command_name} -h 查看更多信息
// "#,
//             ),
//         }
//     }
// }

// impl std::fmt::Display for WhenFiledTips {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.str)
//     }
// }

// impl ArgCount {
//     pub fn check_with_tips(
//         &self,
//         args_for_subcommand: &Vec<String>,
//         when_filed_tips: WhenFiledTips,
//     ) -> DidHandled {
//         match self.check(args_for_subcommand) {
//             DidHandled::Handled => return DidHandled::Handled,
//             DidHandled::Failed(msg) => DidHandled::Failed(format!("{}\n{when_filed_tips}", msg)),
//         }
//     }
//     pub fn check(&self, args_for_subcommand: &Vec<String>) -> DidHandled {
//         let need_arg_count = self;
//         let real_args_count = args_for_subcommand.len();

//         match need_arg_count {
//             ArgCount::Zero => {
//                 if real_args_count != 0 {
//                     return DidHandled::Failed(format!(
//                         "需要的参数数量是 0 个, 传入的参数数量是: {} 个: {:?}",
//                         real_args_count, args_for_subcommand,
//                     ));
//                 }
//             }
//             ArgCount::One => {
//                 if real_args_count != 1 {
//                     return DidHandled::Failed(format!(
//                         "需要的参数数量是 1 个, 传入的参数数量是: {} 个",
//                         real_args_count
//                     ));
//                 }
//             }
//             ArgCount::ZeroOrOne => {
//                 if real_args_count >= 2 {
//                     return DidHandled::Failed(format!(
//                         "需要的参数数量是 0 个 或者 1 个, 传入的参数数量是: {} 个",
//                         real_args_count
//                     ));
//                 }
//             }
//             ArgCount::ZoreOrMany => { /* 无论 real_atgs_count 是多少都符合要求 */ }
//             ArgCount::OneOrMany => {
//                 if real_args_count < 1 {
//                     return DidHandled::Failed(format!(
//                         "至少需要一个参数, 传入的参数数量是: {} 个",
//                         real_args_count
//                     ));
//                 }
//             }
//             ArgCount::Count(count) => {
//                 if real_args_count != *count as usize {
//                     return DidHandled::Failed(format!(
//                         "需要 {} 个参数, 传入的参数数量是: {} 个",
//                         count, real_args_count,
//                     ));
//                 }
//             }
//         };

//         return DidHandled::Handled;
//     }
// }

// #[cfg(test)]
// mod test_arg_count_check {
//     use super::*;

//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn 人眼parse() {
//         let arg_count_arr: Vec<ArgCount> = vec![
//             ArgCount::Zero,
//             ArgCount::ZeroOrOne,
//             ArgCount::ZoreOrMany,
//             ArgCount::One,
//             ArgCount::OneOrMany,
//             ArgCount::Count(0),
//             ArgCount::Count(1),
//             ArgCount::Count(2),
//             ArgCount::Count(3),
//             ArgCount::Count(4),
//             ArgCount::Count(5),
//             ArgCount::Count(6),
//             ArgCount::Count(7),
//             ArgCount::Count(8),
//             ArgCount::Count(9),
//         ];

        
//         let arr: Vec<Vec<String>> = vec![
//             vec![],                                                          // 0
//             vec!["one".to_string()],                                         // 1
//             vec!["one".to_string(), "two".to_string()],                      // 2
//             vec!["one".to_string(), "two".to_string(), "three".to_string()], // 3
//             vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ], // 4
//             vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ], // 5
//             vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ], // 6
//             vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ], // 7
//             vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ], // 8
//             vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ], // 9
//         ];

//         for arg in &arg_count_arr {
//             for x in &arr {
//                 let re = arg.check(&x);
//                 println!("need: {} 实际: {} result: {:?}", arg, x.len(), re);
//                 // assert_eq!(re, DidHandled::Handled)
//             }
//             println!();
//         }
//     }

//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn   zero_or_one(){

//         // ------- ArgCount::ZeroOrOne

//         assert_eq!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec![]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec!["one".to_string()]    ));

//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::ZeroOrOne.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }
//     #[test] #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     fn zore_or_many(){

//         // ------- ArgCount::ZoreOrMany


//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec![]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec!["one".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::ZoreOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }

//     #[test] #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     fn one(){

//         // ------- ArgCount::One


//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec![]    ));

//         assert_eq!(DidHandled::Handled, ArgCount::One.check(& vec!["one".to_string()]    ));

//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::One.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }
//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn one_or_many(){

//         // ------- ArgCount::OneOrMany


//         assert_ne!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec![]    ));

//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec!["one".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::OneOrMany.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }
//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn count_0() {

//         assert_eq!(DidHandled::Handled, ArgCount::Count(0).check(& vec![]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec!["one".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }
//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn count_1() {
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec![]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::Count(1).check(& vec!["one".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(1).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }

//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn count_2() {

//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec![]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec!["one".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::Count(2).check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(2).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }
//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn count_3() {

//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec![]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec!["one".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::Count(3).check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(3).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }
//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn count_9() {
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec![]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec!["one".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(9).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_eq!(DidHandled::Handled, ArgCount::Count(9).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));

//     }

//     #[rustfmt::skip] // 让 rustfmt 不要格式化这个函数里面的代码.
//     #[test]
//     fn zero(){
//         // ------- ArgCount::Zero

//         assert_eq!(DidHandled::Handled, ArgCount::Zero.check(& vec![]    ));

//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec!["one".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Zero.check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));


//         // ------- ArgCount::Count

//         assert_eq!(DidHandled::Handled, ArgCount::Count(0).check(& vec![]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec!["one".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec!["one".to_string(), "two".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec!["one".to_string(), "two".to_string(), "three".to_string()]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), ]    ));
//         assert_ne!(DidHandled::Handled, ArgCount::Count(0).check(& vec![ "one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string(), ]    ));


//     }
// }
