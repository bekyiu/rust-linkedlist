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
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_iter_mut() {}

    #[test]
    fn test_iter() {}

    #[test]
    fn test_into_iter() {}

    #[test]
    fn test_peek() {}

    // 如果是默认的Drop实现, 这个测试是无法通过的
    #[test]
    fn long_list() {}
}

fn main() {}