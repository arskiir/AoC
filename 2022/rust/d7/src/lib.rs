use std::collections::{BTreeSet, HashMap};

/// This overflows the call stack but is correct.
pub fn poor_over_engineered_part1(input: &str) -> u32 {
    #[derive(Debug)]
    pub struct Dir {
        size_accumulated: u32,
        dirs: BTreeSet<String>,
    }

    fn process_cd(command: &Vec<&str>, breadcrumb: &mut Vec<String>) {
        let dir_name = *command.last().unwrap();
        if dir_name == "/" {
            breadcrumb.clear();
            breadcrumb.push("/".to_string());
            return;
        }

        if dir_name == ".." {
            breadcrumb.pop();
            return;
        }

        breadcrumb.push(dir_name.to_string());
    }

    fn process_ls(
        ls_outputs: &[Vec<&str>],
        system: &mut HashMap<String, Dir>,
        breadcrumb: &mut Vec<String>,
    ) {
        let current_dir = breadcrumb.last().unwrap();
        if !system.contains_key(current_dir) {
            system.insert(
                current_dir.to_string(),
                Dir {
                    dirs: BTreeSet::new(),
                    size_accumulated: 0,
                },
            );
        }

        for output in ls_outputs {
            let dir_ref = system.get_mut(current_dir).unwrap();
            if let Ok(file_size) = output[0].parse::<u32>() {
                dir_ref.size_accumulated += file_size;
                continue;
            }
            dir_ref.dirs.insert(output[1].to_string());
        }
    }

    fn size_deep(dir: &Dir, system: &HashMap<String, Dir>) -> u32 {
        if dir.dirs.len() == 0 {
            return dir.size_accumulated;
        }
        dir.dirs
            .iter()
            .map(|dir_name| {
                let dir = system.get(dir_name).unwrap();
                size_deep(dir, system)
            })
            .sum::<u32>()
            + dir.size_accumulated
    }

    let mut system: HashMap<String, Dir> = HashMap::new();
    let mut breadcrumb: Vec<String> = vec![];

    let mut lines_it = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>());
    while let Some(line) = lines_it.next() {
        if line[0] == "$" && line[1] == "ls" {
            // ls command
            let mut ls_outputs: Vec<Vec<&str>> = vec![];
            let mut cd_command_buf: Option<Vec<&str>> = None;
            while let Some(line) = lines_it.next() {
                if line[0] == "$" {
                    // this is another cd command?
                    cd_command_buf = Some(line);
                    break;
                } else {
                    ls_outputs.push(line);
                }
            }
            process_ls(&ls_outputs, &mut system, &mut breadcrumb);
            if let Some(command) = cd_command_buf {
                process_cd(&command, &mut breadcrumb);
            }
        } else {
            // cd command
            process_cd(&line, &mut breadcrumb);
        }
    }

    system
        .iter()
        .map(|(_, dir)| {
            if dir.size_accumulated <= 100000 {
                size_deep(dir, &system)
            } else {
                0
            }
        })
        .sum()
}

pub fn part1(input: &str) -> u32 {
    32
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn ex_part1_works() {
        let result = poor_over_engineered_part1(EXAMPLE);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 95437);
    }
}
