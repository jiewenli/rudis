pub struct List<T> {
    head: Link<T>,
}

type Link = Option<Node<T>>;

pub struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl List {
    pub fn new(&self) -> self {
        List { head: None }
    }
}



