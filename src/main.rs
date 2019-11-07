use crate::node::Node;

mod node;
mod hierarchy_finder;

fn main() {
    let tree = Node::Folder("root", vec![
        Node::Folder(
            "files",
            vec![
                Node::File("file1.md", "Title 1"),
                Node::File("file2.md", "Title 2"),
            ]
        ),
        Node::File("landing.md", "Landing page"),
    ]);

    println!("tree = {:#?}", tree);
}
