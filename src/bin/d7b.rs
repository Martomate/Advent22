use std::{
    collections::{HashMap, VecDeque},
    fmt::{self},
    io::{self, BufRead},
};

enum Command {
    Cd(String),
    Ls,
}

enum ParsedLine {
    Cmd(Command),
    Output(Vec<String>),
}

#[derive(PartialEq, Eq)]
enum LsOutput {
    Dir(String),
    File(i32, String),
}

impl fmt::Display for LsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LsOutput::Dir(name) => write!(f, "Dir({})", name),
            LsOutput::File(size, name) => {
                write!(f, "File({}, {})", name, size.to_string())
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum ExecutedCommand {
    Cd(String),
    Ls(Vec<LsOutput>),
}

impl fmt::Display for ExecutedCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutedCommand::Cd(path) => write!(f, "cd {}", path),
            ExecutedCommand::Ls(output) => {
                let output_lines: Vec<String> =
                    output.iter().map(|line| line.to_string()).collect();
                write!(f, "ls -> {}", output_lines.join(", "))
            }
        }
    }
}

/*
enum FileSystem {
    Dir(String, Option<Vec<Box<FileSystem>>>),
    File(String, i32),
}
*/

#[derive(Debug)]
enum FileSystem {
    Dir(DirectoryData),
    File(FileData),
}

#[derive(Debug)]
struct DirectoryData {
    contents: HashMap<String, Box<FileSystem>>,
}

impl DirectoryData {
    fn update_contents(&mut self, path: &[String], contents: HashMap<String, Box<FileSystem>>) {
        if path.len() == 0 {
            self.contents = contents;
        } else {
            let dirname = &path[0];
            let rest = &path[1..];
            match self.contents.get_mut(dirname) {
                Some(entry) => match entry.as_mut() {
                    FileSystem::Dir(dir) => dir.update_contents(rest, contents),
                    _ => panic!("Not a directory: {}", dirname),
                },
                None => panic!("Could not find: {}", dirname),
            }
        }
    }

    fn all_dir_sizes(&self, result: &mut Vec<i32>) -> i32 {
        let mut dir_size: i32 = 0;

        for (_, c) in self.contents.iter() {
            match &**c {
                FileSystem::Dir(dir) => {
                    let c_dir_size = dir.all_dir_sizes(result);
                    dir_size += c_dir_size;
                }
                FileSystem::File(file) => {
                    dir_size += file.size;
                }
            }
        }

        result.push(dir_size);

        dir_size
    }
}

#[derive(Debug)]
struct FileData {
    size: i32,
}

fn infer_file_system(commands: Vec<ExecutedCommand>) -> FileSystem {
    if commands[0] != ExecutedCommand::Cd("/".to_string()) {
        panic!("you must start with 'cd /'");
    } else {
        let mut root = DirectoryData {
            contents: HashMap::new(),
        };

        let mut current_path: VecDeque<String> = VecDeque::new();

        for command in commands {
            match command {
                ExecutedCommand::Cd(name) => {
                    if name == "/".to_string() {
                        current_path.clear();
                    } else if name == "..".to_string() {
                        current_path.pop_back();
                    } else {
                        current_path.push_back(name);
                    }
                }
                ExecutedCommand::Ls(listing) => {
                    let mut contents: HashMap<String, Box<FileSystem>> = HashMap::new();
                    for c in listing.iter() {
                        match c {
                            LsOutput::Dir(name) => contents.insert(
                                name.clone(),
                                Box::from(FileSystem::Dir(DirectoryData {
                                    contents: HashMap::new(),
                                })),
                            ),
                            LsOutput::File(size, name) => contents.insert(
                                name.clone(),
                                Box::from(FileSystem::File(FileData { size: *size })),
                            ),
                        };
                    }
                    root.update_contents(&current_path.as_slices().0, contents)
                }
            }
        }
        FileSystem::Dir(root)
    }
}

fn parse_line(l: &String) -> Option<ParsedLine> {
    let parts: Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
    if parts.len() == 0 {
        return None;
    }
    if parts[0] == "$" {
        match parts[1].as_str() {
            "cd" => Some(ParsedLine::Cmd(Command::Cd(parts[2].clone()))),
            "ls" => Some(ParsedLine::Cmd(Command::Ls)),
            _ => None,
        }
    } else {
        return Some(ParsedLine::Output(parts));
    }
}

fn parse_ls_output_line(words: &Vec<String>) -> LsOutput {
    if words[0] == "dir" {
        LsOutput::Dir(words[1].clone())
    } else {
        let file_size = words[0].parse::<i32>().unwrap();
        LsOutput::File(file_size, words[1].clone())
    }
}

fn make_executed_command(cmd: &Command, output: &Vec<Vec<String>>) -> ExecutedCommand {
    match cmd {
        Command::Cd(path) => ExecutedCommand::Cd(path.clone()),
        Command::Ls => {
            let ls_output: Vec<LsOutput> = output
                .iter()
                .map(|words| parse_ls_output_line(words))
                .collect();
            ExecutedCommand::Ls(ls_output)
        }
    }
}

fn parse_input() -> Vec<ExecutedCommand> {
    let mut commands: Vec<ExecutedCommand> = Vec::new();
    let mut last_cmd: Option<Command> = None;
    let mut last_output: Vec<Vec<String>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();

        if l.len() == 0 {
            break;
        }

        match parse_line(&l) {
            Some(p) => match p {
                ParsedLine::Cmd(new_cmd) => {
                    match &last_cmd {
                        Some(cmd) => commands.push(make_executed_command(cmd, &last_output)),
                        None => (),
                    }
                    last_cmd = Some(new_cmd);
                    last_output.clear();
                }
                ParsedLine::Output(words) => last_output.push(words),
            },
            None => panic!("Invalid input: {}", l),
        }
    }
    match &last_cmd {
        Some(cmd) => commands.push(make_executed_command(cmd, &last_output)),
        None => (),
    }
    commands
}

fn main() {
    println!("Hello, world!");

    let commands = parse_input();

    let file_system = infer_file_system(commands);
    let root = DirectoryData {
        contents: HashMap::from([("/".to_string(), Box::from(file_system))]),
    };

    println!("{:?}", root.contents.get("/").unwrap());
    let mut dir_sizes: Vec<i32> = Vec::new();
    let root_dir_size = root.all_dir_sizes(&mut dir_sizes);

    let overflow = root_dir_size - 40000000;
    dir_sizes.sort();
    let result = dir_sizes.iter().find(|size| **size >= overflow).unwrap();

    println!("{}", *result);
}
