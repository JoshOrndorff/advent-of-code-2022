use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<Directory>>,
    files: RefCell<Vec<(u32, String)>>,
    children: RefCell<Vec<Rc<Directory>>>,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("should read input file");

    let input = input.trim().split('$').map(|cmd| cmd.trim()).skip(1);

    // This variable owns the root and is useful in case we encounter `cd /` in the middle of the input
    // Every other directory will be owned by its parent
    let root = Rc::new(Directory {
        name: "/".to_string(),
        parent: None,
        files: RefCell::new(Vec::new()),
        children: RefCell::new(Vec::new()),
    });

    // Track the current working directory. Initialize it to root to satisfy the compiler
    // even though we expect inputs to start with `cd /`.
    let mut cwd = Rc::clone(&root);

    // Populate the tree from the input contents
    for cmd in input {
        let parts = cmd.split_whitespace().collect::<Vec<_>>();

        match parts[0] {
            "cd" => {
                let param = parts[1];
                cwd = match param {
                    "/" => Rc::clone(&root),
                    ".." => {
                        let p = cwd.parent.as_ref().expect("Shouldn't call `cd ..` in root");
                        Rc::clone(p)
                    }
                    target_name => {
                        let children = cwd.children.borrow();

                        Rc::clone(
                            children
                                .iter()
                                .find(|ref d| d.name == target_name)
                                .expect("Shouldn't cd into non-existent child directory"),
                        )
                    }
                };
            }
            "ls" => {
                for output_line in parts[1..].chunks(2) {
                    let [size_or_dir, name] = output_line else {
                        panic!("Should have been two elements per output line");
                    };
                    match size_or_dir {
                        &"dir" => {
                            cwd.children.borrow_mut().push(Rc::new(Directory {
                                name: name.to_string(),
                                parent: Some(Rc::clone(&cwd)),
                                children: RefCell::new(Vec::new()),
                                files: RefCell::new(Vec::new()),
                            }));
                        }
                        size => cwd.files.borrow_mut().push((
                            size.parse().expect("file sizes should parse"),
                            name.to_string(),
                        )),
                    }
                }
            }
            _ => {
                panic!("Encountered command that was neither cd nor ls");
            }
        }
    }

    // Now traverse the tree finding directories that are at most 100_000
    let mut dir_sizes = Vec::<u32>::new();
    let root_size = find_dirs(&mut dir_sizes, &root);

    let part_1 = dir_sizes
        .iter()
        .filter(|size| size <= &&100_000u32)
        .sum::<u32>();

    println!("part 1: {:?}", part_1);

    let disk_size = 70_000_000;
    let needed_unused = 30_000_000;
    let currently_available = disk_size - root_size;
    let needed_to_free = needed_unused - currently_available;

    dir_sizes.sort();

    let part_2 = dir_sizes.iter().find(|size| size >= &&needed_to_free).unwrap();

    println!("part 2: {:?}", part_2);
}

fn find_dirs(accumulator: &mut Vec<u32>, dir: &Rc<Directory>) -> u32 {
    let children = dir.children.borrow();
    let files = dir.files.borrow();

    let mut size = 0u32;

    for (file_size, _) in &*files {
        size += file_size;
    }

    for child in &*children {
        size += find_dirs(accumulator, child);
    }

    accumulator.push(size);
    size
}
