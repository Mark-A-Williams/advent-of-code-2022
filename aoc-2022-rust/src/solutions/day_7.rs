extern crate regex;

use std::thread::current;

use regex::Regex;

use super::file_helpers::get_lines_from_file;

pub fn part_1() {
    // Types of line
    // - cd ..
    // - cd <dirname
    // - ls
    // - dir <dirname>
    // - <filesize> <filename>

    let filesystem = parse_input_to_directory_structure("../inputs/7.example.txt");

    println!("{}", filesystem.get_size())
}

pub fn part_2() {
    todo!()
}

fn parse_input_to_directory_structure(input_file: &str) -> Directory {
    let mut current_location = "/".to_string();
    // cd <dir> changes current_location by appending /<dir>
    // cd .. removes the final / and anything after, i.e. "/a/b" goes to "/a"
    // Hack: ignore "cd /" as it's a bit different and occurs on and only on the first line

    let mut filesystem = Directory {
        name: "/".to_string(),
        files: vec![],
        subdirectories: vec![],
    };

    let lines = get_lines_from_file(input_file);

    for line in lines {
        // Only care about `cd ..`, `cd <name>` and `<filesize> <filename>`
        if line.starts_with("dir") || line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let argument = line.split_ascii_whitespace().last().unwrap();

            if argument == ".." {
                let current_relative_path = current_location.split("/").last().unwrap();
                current_location = current_location
                    .strip_suffix(current_relative_path)
                    .unwrap()
                    .to_string();
                current_location = current_location.strip_suffix("/").unwrap().to_string();
            } else {
                current_location = format!("{}/{}", current_location, argument);
            }
        } else {
            let mut correct_subdirectory =
                get_subdirectory_with_name(&mut filesystem, &current_location).unwrap();

            let file_info: Vec<&str> = line.split_ascii_whitespace().collect();

            correct_subdirectory.add_file(File {
                name: file_info[1].to_string(),
                size: file_info[0].parse().unwrap(),
            });
        }
    }

    filesystem
}

fn get_subdirectory_with_name<'a>(
    directory: &'a mut Directory,
    name: &'a String,
) -> Option<&'a mut Directory> {
    for mut subdirectory in directory.subdirectories.iter_mut() {
        if subdirectory.name.to_owned() == name.to_owned() {
            return Some(subdirectory);
        }

        match get_subdirectory_with_name(subdirectory, name) {
            Some(result) => return Some(result),
            None => continue,
        }
    }

    None
}

struct Directory {
    /// The full absolute path of the directory, e.g. "/", "/a", "/a/e" etc.
    name: String,
    subdirectories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn get_size(&self) -> i32 {
        let size_of_subdirectories: i32 = self.subdirectories.iter().map(|o| o.get_size()).sum();
        let size_of_files: i32 = self.files.iter().map(|f| f.size).sum();

        size_of_files + size_of_subdirectories
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
}

struct File {
    name: String,
    size: i32,
}

enum TerminalLineType {}
