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

// 实际上我们无需为List实现Drop trait
// 原因是 Rust 自动为几乎所有类型都实现了 Drop
// 例如我们自定义的结构体, 只要结构体的所有字段都实现了 Drop, 那结构体也会自动实现 Drop
//
// 但是, 有的时候这种自动实现可能不够优秀, 例如考虑以下链表:
// list -> A -> B -> C
// list被drop时, 要先drop A, 要drop A时 需要先drop B ..., 这是一个递归的过程
// 所有如果链表太长, drop就会出问题
//
// 而我们可以手动为 List实现drop 来规避这个问题
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会发生递归drop
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem;
    use crate::{Link, Node};
    use super::List;

    #[test]
    fn test_drop() {
        let node2 = Node {
            elem: 2,
            next: Link::Empty,
        };

        let node1 = Node {
            elem: 1,
            next: Link::More(Box::new(node2)),
        };

        let mut list = List {
            head: Link::More(Box::new(node1)),
        };

        println!("{:?}", list);

        // cur_link一直位于栈上
        let mut cur_link = mem::replace(&mut list.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            // boxed_node是栈上的指针, 指向堆上的Node结构
            // 把boxed_node下一个元素的地址 赋值给了cur_link
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
        // 此时cur_link是一个栈上的Link::Empty
        println!("{:?}", cur_link);
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_left(), None);

        // Populate list
        list.push_left(1);
        list.push_left(2);
        list.push_left(3);

        // Check normal removal
        assert_eq!(list.pop_left(), Some(3));
        assert_eq!(list.pop_left(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_left(4);
        list.push_left(5);

        // Check normal removal
        assert_eq!(list.pop_left(), Some(5));
        assert_eq!(list.pop_left(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_left(), Some(1));
        assert_eq!(list.pop_left(), None);
    }

    // 如果是默认的Drop实现, 这个测试是无法通过的
    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push_left(i);
        }
        drop(list);
    }
}

fn main() {}