use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
        }
    }

    /*
     因为我们的链表是不可变的
     所以每次都产生了一个新的List, 指向被添加的元素

     当我们push了3次之后, 内存结构如下
     list1 = None.push_left(A)
     list2 = list1.push_left(B)
     list3 = list2.push_left(C)

     [list1] --> (A) <--(B) <-- (C)
                         ^       ^
                         |       |
     [list2] ------------+       |
                                 |
                                 |
     [list3] --------------------+
     */
    fn push_left(&self, elem: T) -> List<T> {
        let node = Node {
            elem: elem,
            next: self.head.clone(),
        };

        List {
            head: Some(Rc::new(node)),
        }
    }

    // 返回一个的链表, 新链表中去掉了原来的第一个元素
    fn pop_left(&self) -> List<T> {
        let head = self.head.as_ref();

        List {
            head: head.and_then(|node| {
                let h = node.next.clone();
                h
            })
        }
    }

    fn peek_left(&self) -> Option<&T> {
        let head = self.head.as_ref();
        head.map(|node| {
            &node.elem
        })
    }
}

// 没有实现IntoIter和IterMut是因为:
// 我们用了rc, 所有权会被共享, rc指向的东西不可变
struct Iter<'a, T> {
    // 保存一个引用, 指向当前要被返回的node
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_iter() {
        let list = List::new();
        let list = list.push_left(1).push_left(2).push_left(3);
        let mut it = list.iter();
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
    }


    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.peek_left(), None);

        let list = list.push_left(1).push_left(2).push_left(3);
        assert_eq!(list.peek_left(), Some(&3));

        let list = list.pop_left();
        assert_eq!(list.peek_left(), Some(&2));

        let list = list.pop_left();
        assert_eq!(list.peek_left(), Some(&1));

        let list = list.pop_left();
        assert_eq!(list.peek_left(), None);

        // Make sure empty tail works
        let list = list.pop_left();
        assert_eq!(list.peek_left(), None);
    }
}

fn main() {}