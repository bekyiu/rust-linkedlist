#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct List {
    head: Link,
}

fn main() {
    /*
    该结构实现了c语言风格链表内存布局:
    [ptr] -> (1, ptr) -> (2, null)
    拆分后:
        [ptr] -> (1, null)
        [ptr] -> (2, null)
     */

    let node2 = Node {
        elem: 2,
        next: Link::Empty,
    };

    let node1 = Node {
        elem: 1,
        next: Link::More(Box::new(node2)),
    };

    let list = List {
        head: Link::More(Box::new(node1)),
    };

    println!("{:?}", list);
}
