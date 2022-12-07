use super::file_helpers::get_lines_from_file;

pub fn part_1() {
    // Types of line
    // - cd ..
    // - cd <dirname
    // - ls
    // - dir <dirname>
    // - <filesize> <filename>

    let filesystem = parse_input_to_directory_structure("../inputs/7.example.txt");

    println!("{}", filesystem.get_size());

    print_directory(filesystem)
}

pub fn part_2() {
    todo!()
}

fn print_directory(directory: Directory) {
    for file in directory.files.iter() {
        println!("File {}, size {}", file.name, file.size);
    }

    for subdir in directory.subdirectories {
        println!("Subdr {}", subdir.name);
        print_directory(subdir)
    }
}

fn parse_input_to_directory_structure(input_file: &str) -> Directory {
    let mut current_location = "/".to_string();

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

                if current_location != "/" {
                    current_location = current_location.strip_suffix("/").unwrap().to_string();
                }
            } else {
                let new_full_path = {
                    if current_location == "/" {
                        format!("/{}", argument)
                    } else {
                        format!("{}/{}", current_location, argument)
                    }
                };

                let mut correct_subdirectory =
                    get_subdirectory_with_name(&mut filesystem, &current_location).unwrap();

                correct_subdirectory.add_subdirectory(Directory {
                    name: new_full_path.clone(), // this is so lazy
                    files: vec![],
                    subdirectories: vec![],
                });

                current_location = new_full_path;
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
    println!("{} {}", directory.name, name);
    if directory.name.to_owned() == name.to_owned() {
        return Some(directory);
    }

    for mut subdirectory in directory.subdirectories.iter_mut() {
        // if subdirectory.name.to_owned() == name.to_owned() {
        //     return Some(subdirectory);
        // }

        println!("{} {}", subdirectory.name, name);
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

    fn add_subdirectory(&mut self, directory: Directory) {
        self.subdirectories.push(directory)
    }
}

struct File {
    name: String,
    size: i32,
}

enum TerminalLineType {}
