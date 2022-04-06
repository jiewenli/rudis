pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Node<T>>;

pub struct Node<T> {
    elem: T,
    prev: Box<Link<T>>,
    next: Box<Link<T>>,
}

impl<T> List<T> {
    pub fn new(&self) -> Self {
        List { head: None }
    }
}



