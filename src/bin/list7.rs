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

需要注意的是 Rc<T> 是指向底层数据的不可变的引用，因此你无法通过它来修改数据
这也符合 Rust 的借用规则：要么存在多个不可变借用，要么只能存在一个可变借用。

let mut r1 = Rc::new(10);
*r1 = 30; // Rc没有实现DerefMut trait, 不能这样干

所以我们这个链表是一个不可变的链表, 节点一旦创建后, elem和next都不能再改变了
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