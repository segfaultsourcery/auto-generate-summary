use std::cmp::Ordering;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub type Name = String;
pub type Title = String;
pub type Children = Vec<Node>;

fn main() {
    let root = PathBuf::from("example");
    let tree = get_hierarchy(root).unwrap();

    print_hierarchy(tree);
}

fn print_hierarchy(tree: Node) {
    let mut rows = vec!["# SUMMARY".to_string(), "".to_string()];

    hierarchy_to_md(&tree, "", 0, &mut rows);

    println!("{}", rows.join("\n"));
}

fn hierarchy_to_md(tree: &Node, path: &str, depth: usize, output: &mut Vec<String>) {
    let indentation = match depth {
        0 => None,
        n => Some(String::from_utf8(vec![b' '; (n - 1) * 4]).unwrap()),
    };

    match tree {
        Node::File(name, title) => {
            if let Some(indentation) = indentation {
                if name != "SUMMARY.md" && name != "landing.md" && name.ends_with(".md") {
                    output.push(format!(
                        "{}- [{}]({}/{})",
                        &indentation, &title, &path, &name
                    ));
                }
            }
        }
        Node::Folder(name, title, children) => {
            if let Some(indentation) = indentation {
                output.push(format!(
                    "{}- [{}]({}/{}/landing.md)",
                    &indentation, &title, &path, &name
                ));
            }

            let path = match depth {
                0 => ".".to_string(),
                _ => format!("{}/{}", path, name),
            };

            for node in children {
                hierarchy_to_md(&node, &path, depth + 1, output);
            }
        }
    }
}

fn get_hierarchy(parent: PathBuf) -> Option<Node> {
    let parent_name = parent.file_name()?.to_str()?.to_string();

    let mut children = vec![];

    for entry in fs::read_dir(&parent).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();

        let metadata = fs::metadata(&path).ok()?;

        if metadata.is_dir() {
            children.push(get_hierarchy(path).unwrap());
        } else {
            let file_name = path.file_name()?.to_str()?.to_string();

            if !file_name.ends_with(".md") {
                continue;
            }

            let title = get_title(&path)
                .or_else(|| Some(path.file_name()?.to_str()?.to_string()))
                .unwrap();

            children.push(Node::File(file_name.to_string(), title))
        }
    }

    let title = get_title(&parent.join("landing.md"))
        .or_else(|| Some(parent_name.to_owned()))
        .unwrap();

    children.sort();

    let tree = Node::Folder(parent_name, title, children);
    Some(tree)
}

fn get_title(path: &PathBuf) -> Option<String> {
    let file = fs::File::open(&path).ok()?;
    let mut buffer = BufReader::new(file);

    let mut line = String::new();

    let whitelist: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,!?-+"
            .chars()
            .collect();

    while line.is_empty() {
        let result = buffer.read_line(&mut line);

        if let Ok(0) = result {
            break;
        }

        line = line
            .replace("_", " ")
            .trim()
            .chars()
            .filter(|it| whitelist.contains(it))
            .collect();

        line = line.trim().to_string();
    }

    if line.trim().is_empty() {
        None
    } else {
        Some(line)
    }
}

// region Node

#[derive(Debug, Eq)]
pub enum Node {
    Folder(Name, Title, Children),
    File(Name, Title),
}

impl Node {
    fn title(&self) -> &str {
        match self {
            Node::Folder(_, title, _) => title,
            Node::File(_, title) => title,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title().cmp(other.title())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.title().eq(other.title())
    }
}

// endregion
