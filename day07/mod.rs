pub fn part_one(input: &str) -> usize {
    let root: Dir = Dir { name: "root".to_string(), subdirs: vec![], files: vec![] };
    let mut path: Vec<Dir> = vec![root];
    let mut depth: usize = 0;
    for line in input.trim().split('\n') {
        let seg = line.split(' ').collect::<Vec<&str>>();
        match (seg[0], seg[1]) {
            ("$", "cd")  => {
                match seg[2] {
                    ".." => {
                        path.pop();
                        depth -= 1;
                    }
                    _ => {
                        let new_dir: Dir = Dir { name: seg[2].to_string(), subdirs: vec![], files: vec![] };
                        path[depth].subdirs.push(new_dir);
                        // path.push(new_dir)
                        // depth += 1;
                    }
                }
            }
            ("$", "ls") => {}, // ignore
            ("dir", ..) => {}, // ignore
            _    => {
                println!("{}", line);
                let new_file: File = File { name: seg[1].to_string(), size: seg[0].parse::<usize>().unwrap()};
                path[depth].add_file(new_file);
            }
        }
    }
    println!("{}", path[0].get_total_size());
    return 1;
}

pub fn part_two(input: &str) -> usize {
    return 1;
}

#[derive(Clone)]
struct Dir {
    name: String,
    subdirs: Vec<Dir>,
    files: Vec<File>
}

#[derive(Clone)]
struct File {
    name: String,
    size: usize
}

impl Dir {
    fn get_total_filesize(&self) -> usize {
        return self.files.iter().map(|x| x.size).sum();
    }
    fn get_total_size(&self) -> usize {
        let filesize: usize = self.get_total_filesize();
        let subdir_size: usize = self.subdirs.iter().map(|x| x.get_total_size()).sum();
        return filesize + subdir_size;
    }
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
    fn add_dir(&mut self, dir: Dir) {
        self.subdirs.push(dir);
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 95437);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 0);
    }
    use super::Dir;
    use super::File;
    #[test]
    fn dir_implementation() {
        let mut root: Dir = Dir { name: "root".to_string(), subdirs: vec![], files: vec![] };
        let mut subdir: Dir = Dir { name: "child".to_string(), subdirs: vec![], files: vec![] };

        assert_eq!(root.get_total_filesize(), 0);
        assert_eq!(root.get_total_size(), 0);

        root.files.push(File { name: "f1".to_string(), size: 100 });
        root.files.push(File { name: "f2".to_string(), size: 200 });

        subdir.files.push(File { name: "s1".to_string(), size: 50 });
        subdir.files.push(File { name: "s2".to_string(), size: 150 });

        assert_eq!(root.get_total_filesize(), 300);
        assert_eq!(subdir.get_total_filesize(), 200);

        root.subdirs.push(subdir);
        assert_eq!(root.get_total_size(), 500);
    }
}
