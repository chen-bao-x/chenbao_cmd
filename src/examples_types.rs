use prettytable::{row, Row};

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
pub struct Examples<'a> {
    pub(crate) val: Vec<SingleExample<'a>>,
}

impl std::fmt::Display for Examples<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formated())
    }
}
impl Default for Examples<'_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> Examples<'a> {
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

    pub fn add_single_example(&mut self, command: &'a str, description: &'a str) {
        self.val.push(SingleExample::new(command, description));
    }

    pub fn iter(&self) -> core::slice::Iter<'_, SingleExample<'a>> {
        self.val.iter()
    }
}

// ------- SingleExample -------

#[derive(Clone, Debug, Hash)]
pub struct SingleExample<'a> {
    pub command: &'a str,
    pub description: &'a str,
}

impl<'a> SingleExample<'a> {
    fn new(command: &'a str, description: &'a str) -> Self {
        Self {
            command,
            description,
            // exaple_check_state: ExapleCheckState::Uncheck,
        }
    }
}

impl std::fmt::Display for SingleExample<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formated())
    }
}

impl SingleExample<'_> {
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
