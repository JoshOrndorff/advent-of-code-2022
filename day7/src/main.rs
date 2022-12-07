use std::cell::RefCell;

#[derive(PartialEq, Eq)]
struct Directory<'parent> {
    name: String,
    parent: Option<&'parent Directory<'parent>>,
    files: RefCell<Vec<(u32, String)>>,
    children: RefCell<Vec<Directory<'parent>>>,
}

fn main() {
    let input = std::fs::read_to_string("./example.txt").expect("should read input file");

    let input = input.trim().split('$').map(|cmd| cmd.trim());

    // This variable owns the root and is useful in case we encounter `cd /` in the middle of the input
    // Every other directory will be owned by its parent
    let root = Directory {
        name: "/".to_string(),
        parent: None,
        files: RefCell::new(Vec::new()),
        children: RefCell::new(Vec::new()),
    };

    // Track the current working directory. Initialize it to root to satisfy the compiler
    // even though we expect inputs to start with `cd /`.
    let mut cwd = &root;

    for cmd in input {
        let parts = cmd.split_whitespace().collect::<Vec<_>>();

        match parts[0] {
            "cd" => {
                let param = parts[1];
                cwd = match param {
                    "/" => &root,
                    ".." => cwd.parent.expect("Shouldn't call `cd ..` in root"),
                    target_name => {
                        let children = cwd
                        .children
                        .borrow();

                        &*children
                        .iter()
                        .find(|ref d| d.name == target_name)
                        .expect("Shouldn't cd into non-existent child directory")

                        // let mut target_node = &root;
                        // for c in &*children {
                        //     if c.name == target_name {
                        //         target_node = c;
                        //     }
                        // }
                        // target_node
                    },
                };
            }
            "ls" => {
                for output_line in parts[1..].chunks(2) {
                    let [size_or_dir, name] = output_line else {
                        panic!("Should have been two elements per output line");
                    };
                    match size_or_dir {
                        &"dir" => {
                            cwd.children.borrow_mut().push(Directory {
                                name: name.to_string(),
                                parent: Some(&cwd),
                                children: RefCell::new(Vec::new()),
                                files: RefCell::new(Vec::new()),
                            });
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

    // println!("part 1; {:?}", part_1);
}
