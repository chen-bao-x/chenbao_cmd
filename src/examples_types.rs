use owo_colors::OwoColorize;
use prettytable::{format::TableFormat, row, table, Row};

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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

    // pub fn pretty_formated(&self) -> Table {
    pub fn pretty_formated(&self) -> Vec<Row> {
        let mut re: Vec<Row> = vec![];

        for x in &self.val {
            re.push(row![x.formated()]);
        }
        re
    }

    pub fn add_single_example(&mut self, command: &str, description: &str) {
        self.val.push(SingleExample::new(command, description));
    }

    pub fn iter(&self) -> core::slice::Iter<'_, SingleExample> {
        self.val.iter()
    }
}

// ------- SingleExample -------

#[derive(Clone, Debug, Hash)]
pub struct SingleExample {
    // pub command: &'a str,
    pub command: String,
    // pub description: &'a str,
    pub description: String,
}

impl SingleExample {
    fn new(command: &str, description: &str) -> Self {
        Self {
            command: command.to_string(),
            description: description.to_owned(),
            // exaple_check_state: ExapleCheckState::Uncheck,
        }
    }
}

impl std::fmt::Display for SingleExample {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.formated())
    }
}

impl SingleExample {
    pub fn formated(&self) -> String {
        let mut t = table!();

        let mut f = TableFormat::new();
        {
            f.column_separator(' ');
            f.padding(1, 0);
            f.left_border('│');
        }
        t.set_format(f);

        format!(
            r#"
{left_border} {des}
{left_border} 
{left_border}     {cmd_name}

"#,
            des = self.description,
            cmd_name = self.command,
            left_border = "┃".bright_yellow()
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
