use std::{collections::HashMap, iter::Peekable, path::PathBuf};

use itertools::Itertools;

struct File {
    size: usize,
    _name: String,
}

#[derive(Default)]
struct Directory {
    files: Vec<File>,
    dirs: Vec<String>,
}

enum Command {
    CD(String),
    LS(Directory),
}

struct VM {
    cwd: PathBuf,
    filesystem: HashMap<PathBuf, Directory>,
}

impl VM {
    fn new() -> Self {
        VM {
            cwd: "/".into(),
            filesystem: HashMap::new(),
        }
    }

    fn execute(&mut self, cmd: Command) {
        match cmd {
            Command::CD(dir) if dir == "/" => {
                self.cwd = PathBuf::from(dir);
            }
            Command::CD(dir) if dir == ".." => {
                self.cwd = self.cwd.parent().unwrap().into();
            }
            Command::CD(dir) => {
                let path = self.cwd.join(dir);
                self.filesystem.entry(path.clone()).or_default();
                self.cwd = path;
            }
            Command::LS(directory) => {
                self.filesystem.insert(self.cwd.clone(), directory);
            }
        }
    }

    fn calculate_sizes(&self) -> HashMap<PathBuf, usize> {
        enum DFS {
            Up,
            Down,
        }
        let mut result = HashMap::new();
        let mut stack = vec![(PathBuf::from("/"), DFS::Down)];
        while let Some((path, dfs)) = stack.pop() {
            let children = self
                .filesystem
                .get(&path)
                .unwrap()
                .dirs
                .iter()
                .map(|d| path.join(d));
            match dfs {
                DFS::Down => {
                    stack.push((path.clone(), DFS::Up));
                    stack.extend(children.map(|p| (p, DFS::Down)));
                }
                DFS::Up => {
                    let dirsizes: usize = children.map(|p| result.get(&p).unwrap()).sum();
                    let filesizes: usize = self
                        .filesystem
                        .get(&path)
                        .unwrap()
                        .files
                        .iter()
                        .map(|f| f.size)
                        .sum();
                    result.insert(path.clone(), dirsizes + filesizes);
                }
            }
        }
        result
    }
}

struct CommandIterator<T: Iterator> {
    tokens: Peekable<T>,
}

trait IntoCommandIterator<T: Iterator = Self> {
    fn commands(self) -> CommandIterator<T>;
}

impl<T: Iterator<Item = Token>> IntoCommandIterator for T {
    fn commands(self) -> CommandIterator<T> {
        CommandIterator {
            tokens: self.peekable(),
        }
    }
}

impl<T: Iterator<Item = Token>> Iterator for CommandIterator<T> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.tokens.next() {
            Some(token) => token,
            None => return None,
        };
        if let Token::CD = token {
            return Some(Command::CD(match self.tokens.next() {
                Some(Token::Text(text)) => text,
                _ => panic!("Bad tokens"),
            }));
        }
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        while let Some(peek) = self.tokens.peek() {
            if let Token::CD | Token::LS = peek {
                break;
            }
            match (self.tokens.next().unwrap(), self.tokens.next().unwrap()) {
                (Token::Dir, Token::Text(name)) => dirs.push(name),
                (Token::Number(size), Token::Text(name)) => files.push(File { size, _name: name }),
                _ => panic!("Bad tokens"),
            }
        }
        Some(Command::LS(Directory { files, dirs }))
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    CD,
    LS,
    Text(String),
    Number(usize),
    Dir,
}

fn tokenise(input: &str) -> impl Iterator<Item = Token> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| {
            let mut it = l
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .peekable();
            let mut tokens = Vec::new();
            if let Some(&"$") = it.peek() {
                it.next();
                if let Some("ls") = it.next() {
                    tokens.push(Token::LS);
                } else {
                    tokens.push(Token::CD);
                    tokens.push(Token::Text(it.next().unwrap().into()));
                    assert!(it.next().is_none());
                }
            } else {
                if let Some(&"dir") = it.peek() {
                    it.next();
                    tokens.push(Token::Dir);
                } else {
                    tokens.push(Token::Number(it.next().unwrap().parse::<usize>().unwrap()));
                }
                tokens.push(Token::Text(it.next().unwrap().into()));
            }
            tokens.into_iter()
        })
}

pub(crate) fn solve(input: &str) -> usize {
    tokenise(input)
        .commands()
        .fold(VM::new(), |mut vm, cmd| {
            vm.execute(cmd);
            vm
        })
        .calculate_sizes()
        .values()
        .filter(|s| **s <= 100000)
        .sum()
}

pub(crate) fn solve_2(input: &str) -> usize {
    let sizes = tokenise(input)
        .commands()
        .fold(VM::new(), |mut vm, cmd| {
            vm.execute(cmd);
            vm
        })
        .calculate_sizes();
    let used_space = *sizes.get(&PathBuf::from("/")).unwrap();
    let total_space = 70000000;
    let ideal_free_space = 30000000;
    let ideal_used_space = total_space - ideal_free_space;
    let excess_usage = used_space - ideal_used_space;
    sizes
        .values()
        .copied()
        .sorted()
        .find(|s| *s >= excess_usage)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenise() {
        let input = "
            $ cd slwwgc
            $ ls
            118185 jjhc.tzr
            291916 jwnw.wqv
            dir abcx
            $ cd slwwgc
        ";
        let tokens = tokenise(input).collect_vec();
        assert_eq!(
            tokens,
            vec![
                Token::CD,
                Token::Text("slwwgc".into()),
                Token::LS,
                Token::Number(118185),
                Token::Text("jjhc.tzr".into()),
                Token::Number(291916),
                Token::Text("jwnw.wqv".into()),
                Token::Dir,
                Token::Text("abcx".into()),
                Token::CD,
                Token::Text("slwwgc".into()),
            ]
        );
    }
}
