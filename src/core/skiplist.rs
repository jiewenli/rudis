pub struct skiplist<T> {
    head: SkipLink<T>,
    tail: SkipLink<T>,
    level: i32,
    length: i32,
}

struct SkipLink<T> {
    elem: SkipNode<T>,
}

struct SkipNode<T> {
    levels: [SkipLevel],
    backward: SkipNode,
    score: i64,
    val: T,
}

struct SkipLevel {
    forward: SkipNode,
    span: u32,
}