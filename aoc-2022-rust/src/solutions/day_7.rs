pub fn part_1() {
    /*!*
    /
        /a
            /a/e
                i
            f
            g
            h.lst
        b.txt
        c.dat
        /d
            j
            d.log
            d.ext
            k


     */

    let example_data = Directory {
        name: "/".to_string(),
        files: vec![
            File {
                name: "b.txt".to_string(),
                size: 14848514,
            },
            File {
                name: "c.dat".to_string(),
                size: 8504156,
            },
        ],
        subdirectories: vec![],
    };

    println!("{}", example_data.get_size())
}

pub fn part_2() {
    todo!()
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
}

struct File {
    name: String,
    size: i32,
}
