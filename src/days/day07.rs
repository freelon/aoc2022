use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(DayXX { input })
}

impl Day for DayXX {
    fn part1(&self) -> String {
        let filesystem = parse(&self.input);
        format!("{}", filesystem.size_of_subs_smaller_100k())
    }

    fn part2(&self) -> String {
        let filesystem = parse(&self.input);
        let used = filesystem.size();
        let max = 70000000;
        let available = max - used;
        let need_to_free = 30000000 - available;
        let smallest_fitting = filesystem
            .all_sizes()
            .into_iter()
            .filter(|size| *size >= need_to_free)
            .min()
            .unwrap();
        format!("{smallest_fitting}")
    }
}

struct DayXX {
    input: String,
}

type File = usize;

#[derive(Default)]
struct Dir {
    children: Vec<Dir>,
    files: Vec<File>,
}

impl Dir {
    fn size(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.size())
            .sum::<usize>()
            + self.files.iter().sum::<usize>()
    }

    fn size_of_subs_smaller_100k(&self) -> usize {
        let mut result = self
            .children
            .iter()
            .map(|it| it.size_of_subs_smaller_100k())
            .sum();

        if self.size() <= 100_000 {
            result += self.size();
        }

        result
    }

    fn all_sizes(&self) -> Vec<usize> {
        let mut result = vec![self.size()];
        for child in &self.children {
            result.append(&mut child.all_sizes());
        }
        result
    }
}

fn parse(input: &str) -> Dir {
    let mut remaining: &mut dyn Iterator<Item=&str> = &mut input.lines().skip(1);

    process(&mut remaining)
}

fn process(remaining: &mut dyn Iterator<Item=&str>) -> Dir {
    let mut dir = Dir::default();
    while let Some(line) = remaining.next() {
        if line.starts_with("$ ls") || line.starts_with("dir ") {
            // nothing to do here
            continue;
        } else if line.starts_with("$ cd ..") {
            return dir;
        } else if line.starts_with("$ cd ") {
            let child = process(remaining);
            dir.children.push(child);
        } else {
            let (size, _filename) = line.split_once(' ').unwrap();
            let size: usize = size.parse().expect("must start with a number!");
            dir.files.push(size);
        }
    }
    dir
}

#[cfg(test)]
mod test {
    use crate::days::day07::create;

    #[test]
    fn example1() {
        assert_eq!(create(EXAMPLE_INPUT.to_string()).part1(), "95437");
    }

    const EXAMPLE_INPUT: &str = "$ cd /
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
}
