pub type Name<'a> = &'a str;
pub type Title<'a> = &'a str;
pub type Children<'a> = Vec<Node<'a>>;


#[derive(Debug)]
pub enum Node<'a> {
    Folder(Name<'a>, Children<'a>),
    File(Name<'a>, Title<'a>),
}
