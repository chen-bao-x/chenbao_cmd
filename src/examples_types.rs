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

    pub fn formated(&self) -> String {
        let mut re: String = String::new();

        for x in &self.val {
            re = re + &x.formated();
        }

        return re;
    }

    pub fn add_single_example(self, command: String, description: String) -> Self {
        let mut re = self;

        let e = SingleExample {
            command,
            description,
        };

        re.val.push(e);
        return re;
    }
}

// ------- SingleExample -------

#[derive(Clone, Debug)]
pub struct SingleExample {
    command: String,
    description: String,
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
    let arr: Examples = Examples::new()
        .add_single_example(
            "ls | sort-by size".to_string(),
            "List the files in the current directory, sorted by size:".to_string(),
        )
        .add_single_example(
            "sys host | get hostname".to_string(),
            "Get the current system host name:".to_string(),
        )
        .add_single_example(
            "Get the processes on your system actively using CPU:".to_string(),
            "ps | where cpu > 0".to_string(),
        );

    println!("{}", arr);
}
