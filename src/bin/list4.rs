#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

// 之前的Link枚举, 实际上就是Option了, 没必要再定义一次
type Link = Option<Box<Node>>;

#[derive(Debug)]
struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
        }
    }

    // 在链表头部添加节点
    fn push_left(&mut self, value: i32) {
        // 构造新节点
        let node = Node {
            elem: value,
            // 原来的mem::replace可以简化为take
            // next指向原来head指向的节点
            // head现在是None
            next: self.head.take(),
        };
        // 让链表头部指向新节点
        self.head = Some(Box::new(node));
    }

    // 从链表头部移出元素
    fn pop_left(&mut self) -> Option<i32> {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         // 这里node.next 指向的Link 所有权转移给self.head了
        //         // 既让self.head指向node的下一个元素, 又让node指向下一个元素的引用断掉了
        //         self.head = node.next;
        //         // node对象中有部分所有权转移, 但是不影响node.elem的访问
        //         Some(node.elem)
        //     }
        // }

        // 上述代码可以用map改写
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
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