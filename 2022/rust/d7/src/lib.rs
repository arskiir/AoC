use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
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
        dirs: &mut HashMap<Vec<String>, u32>,
        breadcrumb: &mut Vec<String>,
    ) {
        for output in ls_outputs {
            if let Ok(file_size) = output[0].parse::<u32>() {
                (1..=breadcrumb.len()).for_each(|window_size| {
                    let dir_name = breadcrumb.windows(window_size).next().unwrap();
                    dirs.entry(dir_name.to_vec())
                        .and_modify(|acc_size| *acc_size += file_size)
                        .or_insert(file_size);
                });
                continue;
            }
        }
    }

    let mut dirs: HashMap<Vec<String>, u32> = HashMap::new();
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
            process_ls(&ls_outputs, &mut dirs, &mut breadcrumb);
            if let Some(command) = cd_command_buf {
                process_cd(&command, &mut breadcrumb);
            }
        } else {
            // cd command
            process_cd(&line, &mut breadcrumb);
        }
    }

    dirs.iter()
        .map(
            |(_, acc_size)| {
                if *acc_size <= 100000 {
                    *acc_size
                } else {
                    0
                }
            },
        )
        .sum()
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
        let result = part1(EXAMPLE);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 1908462);
    }
}
