use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

/*
在我们之前的定义的链表结构中 它的内存布局是这样的

    该结构实现了c语言风格链表内存布局:
    [ptr] -> (1, ptr) -> (2, null)
    拆分后:
        [ptr] -> (1, null)
        [ptr] -> (2, null)

这样的链表有一个问题, 因为rust所有权的机制, node节点无法被共享, 也就是说无法做到下面的内存布局:
    list1 -> A ---+
                  |
                  v
    list2 ------> B -> C -> D
                  ^
                  |
    list3 -> X ---+
我们看到B节点有三个所有者, 这在之前的链表结构中是无法实现的

所以我们修改了Link<T>的定义
从Option<Box<Node<T>>> 改到 Option<Rc<Node<T>>>
 */
type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

fn main() {
    let node1 = Rc::new(Node {
        elem: 1,
        next: None,
    });

    let node2 = node1.clone();

    let list1 = List {
        head: Some(node1),
    };

    let list2 = List {
        head: Some(node2),
    };

    println!("{:?}", list1);
    println!("{:?}", list2);

}