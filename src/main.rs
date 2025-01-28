mod cmd;

fn main() {
    cmd::hello();
    cmd::test_app_default_action();

}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn asdfdasf(){
//         _ = App::new()
//         .about("这个程序是做某某事的")
//         .authors(vec!["chen bao", "bao chen"]) // Vec<&'static str>
//         .add_command(
//             command::new("build") // "build" is the command name.
//                 .short("b") // "b" b is the command short name
//                 .about("Build project from build.zig") // b build ---- "Build project from build.zig"
//                 .example("")
//                 .action(|arg| {}),
//         )
//         .add_command(
//             command::new("run")
//                 .short("r")
//                 .about("Build project from build.zig")
//                 .example("")
//                 .action(|arg| {}),
//         )
//         .add_command(
//             command::new("init")
//                 // .short("")
//                 .about("Build project from build.zig")
//                 .example("")
//                 .usage("git init")
//                 .arg(ArgCount::Zero)
//                 .action(|arg| {}),
//         )
//         .run();
//     }
// }
