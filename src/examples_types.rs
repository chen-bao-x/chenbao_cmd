use prettytable::{row, table, Row, Table};

use crate::helper;

///
/// List the files in the current directory, sorted by size:
///     ls | sort-by size
///
/// Get the current system host name:
///     sys host | get hostname
///
/// Get the processes on your system actively using CPU:
///     ps | where cpu > 0
///

// ------- Examples -------
#[derive(Clone, Debug)]
pub struct Examples {
    val: Vec<SingleExample>,
}

impl std::fmt::Display for Examples {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formated())
    }
}

impl Examples {
    pub fn new() -> Self {
        Self { val: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.val.is_empty()
    }

    pub fn formated(&self) -> String {
        let mut re: String = String::new();

        for x in &self.val {
            re = re + &x.formated();
        }

        return re;
    }

    pub fn pretty_formated(&self) -> Table {
        let mut table = table!();
        table.set_format(helper::table_formater());

        for x in &self.val {
            table.add_row(row![x.formated()]);
        }

        return table;
    }

    pub fn add_single_example(&mut self, command: &'static str, description: &'static str) {
        let e = SingleExample {
            command,
            description,
        };

        self.val.push(e);
    }
}

// ------- SingleExample -------

#[derive(Clone, Debug)]
struct SingleExample {
    command: &'static str,
    description: &'static str,
}

impl std::fmt::Display for SingleExample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formated())
    }
}

impl SingleExample {
    pub fn formated(&self) -> String {
        format!(
            r#"
{}
    {}

"#,
            self.description, self.command
        )
    }
}

#[test]
fn sdafasdf() {
    let mut arr: Examples = Examples::new();
    arr.add_single_example(
        "ls | sort-by size",
        "List the files in the current directory, sorted by size:",
    );
    arr.add_single_example(
        "sys host | get hostname",
        "Get the current system host name:",
    );
    arr.add_single_example(
        "Get the processes on your system actively using CPU:",
        "ps | where cpu > 0",
    );

    println!("{}", arr);
}
