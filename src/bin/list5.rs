// 让链表支持泛型
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self <> {
        List {
            head: None,
        }
    }

    // 在链表头部添加节点
    fn push_left(&mut self, value: T) {
        // 构造新节点
        let node = Node {
            elem: value,
            // next指向原来head指向的节点
            // head现在是None
            next: self.head.take(),
        };
        // 让链表头部指向新节点
        self.head = Some(Box::new(node));
    }

    // 从链表头部移出元素
    fn pop_left(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            // 这里node.next 指向的Link 所有权转移给self.head了
            // 既让self.head指向node的下一个元素, 又让node指向下一个元素的引用断掉了
            self.head = node.next;
            node.elem
        })
    }

    // 返回链表头部元素的引用
    fn peek_left(&self) -> Option<&T> {
        // match &self.head {
        //     None => None,
        //     Some(node) => {
        //         Some(&node.elem)
        //     }
        // }

        // 这里as_ref通过Option<T> 构造出 Option<&T>
        // 没有转移self.head的所有权
        let head = self.head.as_ref();
        head.map(|node| {
            &node.elem
        })
    }

    fn peek_left_mut(&mut self) -> Option<&mut T> {
        let head = self.head.as_mut();
        head.map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会发生递归drop
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_peek() {
        let mut list = List::new();
        list.push_left(1);
        list.push_left(2);

        assert_eq!(list.peek_left(), Some(&2));
        list.pop_left();
        assert_eq!(list.peek_left_mut(), Some(&mut 1));
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
            list.push_left(i.to_string());
        }
        drop(list);
    }
}

fn main() {}