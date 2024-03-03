use std::cell::{Ref, RefCell, RefMut};
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

    fn push_right(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
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
                }
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

    fn pop_right(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }


    fn peek_left(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            let node = node.borrow();

            // 这里是没法返回 &(node.elem) 作为 Option<&T> 的
            // 因为node是个局部变量, 没法返回局部变量的引用
            // 所以退而求其次改为了返回 Option<Ref<T>>
            Ref::map(node, |n| &n.elem)
        })
    }

    fn peek_left_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    fn peek_right(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    fn peek_right_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
}

struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_left()
    }
}

// 从后向前迭代
// DoubleEndedIterator继承自Iterator
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_right()
    }
}

// 没有实现 Iter 和 IterMut, 因为作者就放弃了...

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_left(1);
        list.push_left(2);
        list.push_left(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_left().is_none());
        assert!(list.peek_right().is_none());
        assert!(list.peek_left_mut().is_none());
        assert!(list.peek_right_mut().is_none());

        list.push_left(1);
        list.push_left(2);
        list.push_left(3);

        assert_eq!(*list.peek_left().unwrap(), 3);
        assert_eq!(*list.peek_left_mut().unwrap(), 3);
        assert_eq!(*list.peek_right().unwrap(), 1);
        assert_eq!(*list.peek_right_mut().unwrap(), 1);
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

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_right(), None);

        // Populate list
        list.push_right(1);
        list.push_right(2);
        list.push_right(3);

        // Check normal removal
        assert_eq!(list.pop_right(), Some(3));
        assert_eq!(list.pop_right(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_right(4);
        list.push_right(5);

        // Check normal removal
        assert_eq!(list.pop_right(), Some(5));
        assert_eq!(list.pop_right(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_right(), Some(1));
        assert_eq!(list.pop_right(), None);
    }
}


fn main() {}