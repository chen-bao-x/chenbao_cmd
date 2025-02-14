use prettytable::{row, table, Table};

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
#[derive(Clone, Debug, Hash)]
pub struct Examples {
    pub(crate) val: Vec<SingleExample>,
}

impl std::fmt::Display for Examples {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formated())
    }
}
impl Default for Examples {
    fn default() -> Self {
        Self::new()
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

       re
    }

    pub fn pretty_formated(&self) -> Table {
        let mut table = table!();
        table.set_format(helper::table_formater());

        for x in &self.val {
            table.add_row(row![x.formated()]);
        }

        table
    }

    pub fn add_single_example(&mut self, command: String, description: String) {
        let e = SingleExample {
            command,
            description,
        };

        self.val.push(e);
    }
}

// ------- SingleExample -------

#[derive(Clone, Debug, Hash)]
pub(crate) struct SingleExample {
    pub command: String,
    pub description: String,
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
        "ls | sort-by size".to_string(),
        "List the files in the current directory, sorted by size:".to_string(),
    );
    arr.add_single_example(
        "sys host | get hostname".to_string(),
        "Get the current system host name:".to_string(),
    );
    arr.add_single_example(
        "Get the processes on your system actively using CPU:".to_string(),
        "ps | where cpu > 0".to_string(),
    );

    println!("{}", arr);
}
