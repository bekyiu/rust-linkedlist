use std::mem;

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

impl List {
    fn new() -> Self {
        List {
            head: Link::Empty,
        }
    }

    // 在链表头部添加节点
    fn push_left(&mut self, value: i32) {
        // 构造新节点
        let node = Node {
            elem: value,
            // replace(&dest, src)的作用是: 读取&dest的值用于返回, 让后把src写到&dest处
            // self.head 原来是 More(Box<Node>), 指向第一个节点
            // 现在被修改为了Empty
            next: mem::replace(&mut self.head, Link::Empty),
        };
        // 让链表头部指向新节点
        self.head = Link::More(Box::new(node));
    }

    // 从链表头部移出元素
    fn pop_left(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                // 这里node.next 指向的Link 所有权转移给self.head了
                // 既让self.head指向node的下一个元素, 又让node指向下一个元素的引用断掉了
                self.head = node.next;
                // node对象中有部分所有权转移, 但是不影响node.elem的访问
                Some(node.elem)
            }
        }
    }
}


fn main() {
    let mut list = List::new();
    list.push_left(1);
    list.push_left(2);
    list.push_left(3);
    println!("{:?}", list);
    list.pop_left();
    println!("{:?}", list);
}
