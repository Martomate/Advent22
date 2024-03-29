use std::{
    collections::{HashMap, VecDeque},
    fmt::{self},
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
                write!(f, "File({}, {})", name, size)
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
        if path.is_empty() {
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

    fn acc_size(&self, max_dir_size: i32) -> (i32, i32) {
        let mut dir_size: i32 = 0;
        let mut total_acc_size: i32 = 0;

        for (_, c) in self.contents.iter() {
            match &**c {
                FileSystem::Dir(dir) => {
                    let (c_dir_size, c_acc_size) = dir.acc_size(max_dir_size);
                    dir_size += c_dir_size;
                    total_acc_size += c_acc_size;
                }
                FileSystem::File(file) => {
                    dir_size += file.size;
                }
            };
        }

        if dir_size <= max_dir_size {
            total_acc_size += dir_size;
        }

        (dir_size, total_acc_size)
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
                    if name == *"/" {
                        current_path.clear();
                    } else if name == *".." {
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
                    root.update_contents(current_path.as_slices().0, contents)
                }
            }
        }
        FileSystem::Dir(root)
    }
}

/*
fn infer_file_system_old2(commands: Vec<ExecutedCommand>) -> FileSystem {
    if commands[0] != ExecutedCommand::Cd("/".to_string()) {
        panic!("you must start with 'cd /'");
    } else {
        let mut fs: HashMap<String, FileSystem> = HashMap::new();
        let mut current_path: String = "/".to_string();

        for command in commands {
            match command {
                ExecutedCommand::Cd(name) => {
                    if name == "/".to_string() {
                        current_path = name;
                    } else if name == "..".to_string() {
                        let last_slash_idx = current_path.rfind('/').unwrap();
                        current_path = current_path[..last_slash_idx].to_string();
                    } else {
                        current_path.push('/');
                        current_path.push_str(name.as_str());
                    }
                }
                ExecutedCommand::Ls(ls_output) => {}
            }
        }

        let root = DirectoryData {
            contents: HashMap::new(),
        };
    }
}
*/
/*
fn infer_file_system_old(commands: Vec<ExecutedCommand>) -> FileSystem {
    if commands[0] != ExecutedCommand::Cd("/".to_string()) {
        panic!("you must start with 'cd /'");
    } else {
        let mut dirs: VecDeque<FileSystem> = VecDeque::new();
        dirs.push_back(FileSystem::Dir("/".to_string(), None));

        for command in commands {
            match command {
                ExecutedCommand::Cd(name) => {
                    if name == "/".to_string() {
                    } else if name == "..".to_string() {
                        match dirs.pop_back() {
                            Some(FileSystem::Dir(dir_name, Some(dir_contents))) => {
                                match dirs.pop_back() {
                                    Some(FileSystem::Dir(parent_name, parent_contents)) => {
                                        let new_parent_contents = match parent_contents {
                                            Some(pc) => fun_name(dir_name, &dir_contents, pc),
                                            None => Some(dir_contents),
                                        };
                                        dirs.push_back(FileSystem::Dir(
                                            parent_name,
                                            new_parent_contents,
                                        ))
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    } else {
                    }
                }
                ExecutedCommand::Ls(ls_output) => match dirs.pop_back() {
                    Some(FileSystem::Dir(dirname, _)) => {
                        let dir_content: Vec<Box<FileSystem>> = ls_output
                            .iter()
                            .map(|c| match c {
                                LsOutput::Dir(name) => {
                                    Box::from(FileSystem::Dir(name.clone(), None))
                                }
                                LsOutput::File(size, name) => {
                                    Box::from(FileSystem::File(name.clone(), *size))
                                }
                            })
                            .collect();
                        dirs.push_back(FileSystem::Dir(dirname, Some(dir_content)))
                    }
                    _ => panic!("invalid input!"),
                },
            }
        }

        dirs.pop_front().unwrap()
    }
}

fn fun_name(
    dir_name: String,
    dir_contents: &Vec<Box<FileSystem>>,
    pc: Vec<Box<FileSystem>>,
) -> Option<Vec<Box<FileSystem>>> {
    let d = FileSystem::Dir(dir_name.clone(), Some(*dir_contents));
    Some(
        pc.iter()
            .map(|&c| match *c {
                FileSystem::Dir(dname, _) if *dname == dir_name => Box::from(d),
                c => Box::from(c),
            })
            .collect(),
    )
}
*/

fn parse_line(l: &str) -> Option<ParsedLine> {
    let parts: Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
    if parts.is_empty() {
        return None;
    }
    if parts[0] == "$" {
        match parts[1].as_str() {
            "cd" => Some(ParsedLine::Cmd(Command::Cd(parts[2].clone()))),
            "ls" => Some(ParsedLine::Cmd(Command::Ls)),
            _ => None,
        }
    } else {
        Some(ParsedLine::Output(parts))
    }
}

fn parse_ls_output_line(words: &[String]) -> LsOutput {
    if words[0] == "dir" {
        LsOutput::Dir(words[1].clone())
    } else {
        let file_size = words[0].parse::<i32>().unwrap();
        LsOutput::File(file_size, words[1].clone())
    }
}

fn make_executed_command(cmd: &Command, output: &[Vec<String>]) -> ExecutedCommand {
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

fn parse_input(input: &str) -> Vec<ExecutedCommand> {
    let mut commands: Vec<ExecutedCommand> = Vec::new();
    let mut last_cmd: Option<Command> = None;
    let mut last_output: Vec<Vec<String>> = Vec::new();
    
    for l in input.lines() {
        if l.is_empty() {
            break;
        }

        match parse_line(l) {
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

pub fn main(input: &str) -> i32{
    println!("Hello, world!");

    let commands = parse_input(input);

    let file_system = infer_file_system(commands);
    let root = DirectoryData {
        contents: HashMap::from([("/".to_string(), Box::from(file_system))]),
    };

    root.acc_size(100000).1
}
