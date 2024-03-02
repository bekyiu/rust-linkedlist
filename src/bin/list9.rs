use std::cell::RefCell;
use std::rc::Rc;

// 双向链表
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

// 双向链表中的结点必然会被多个人持有, 所以需要Rc
// Rc<T>是指向的不可变引用, 想要改变T的值, 可以用RefCell包裹T
// RefCell<T>不会在堆上分配任何内存, 只是把T包了一层
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        let node = Node {
            elem: value,
            next: None,
            prev: None,
        };

        Rc::new(RefCell::new(node))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_left(&mut self, value: T) {
        let new_head = Node::new(value);
        let first = self.head.take();
        match first {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn pop_left(&mut self) -> Option<T> {
        let first = self.head.take();
        first.map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                },
                None => {
                    self.tail = None
                }
            };
            // 这样是不行的 相当于是(&mut Node<T>).elem, 没发通过引用来move
            // old_head.borrow_mut().elem


            // 拿到Rc里面的东西
            let node = Rc::try_unwrap(old_head).ok().unwrap();
            // into_inner消耗掉RefCell, 拿到T
            node.into_inner().elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

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
}


fn main() {
}