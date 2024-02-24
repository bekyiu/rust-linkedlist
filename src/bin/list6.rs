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


// 为List实现3种迭代器
struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        // 直接转移所有权
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop_left()
    }
}

// 在结构体内使用生命周期
// 代表着 被引用的这个东西 至少要和 结构体对象的实例 活的一样长
struct Iter<'a, T> {
    // 保存一个引用, 指向当前要被返回的node
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
    // 所以这个方法上不用标生命周期
    fn iter(&self) -> Iter<T> {
        Iter {
            // self.head的类型是 Option<Box<Node<T>>>
            // 把Box<Node<T>> 看成 K
            // as_deref相当于对K进行了一次 Deref trait 的操作; 即 Option<K> -> Option<&K::Target>
            // Box是实现了 Deref trait 的, 这里Box的关联类型Target 就是 Node<T>
            // 对Box<Node<T>> 执行deref方法后 就是 &Node<T>
            // 所以 Option<Box<Node<T>>> -> Option<&Node<T>>
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            // 因为生命周期已经标注了 被引用的T 至少要和 Iter的实例 活的一样长
            // 所以这里能直接返回引用
            &node.elem
        })
    }
}

struct IterMut<'a, T> {
    // 保存一个引用, 指向当前要被返回的node
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            // self.head的类型是 Option<Box<Node<T>>>
            // 把Box<Node<T>> 看成 K
            // as_deref_mut相当于对K进行了一次 DerefMut trait 的操作; 即 Option<K> -> Option<&mut K::Target>
            // Box是实现了 DerefMut trait 的, 这里Box的关联类型Target 就是 Node<T>
            // 对Box<Node<T>> 执行deref_mut方法后 就是 &mut Node<T>
            // 所以 Option<Box<Node<T>>> -> Option<&mut Node<T>>
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // 这里不能直接self.next.map(...)
        // 因为self.next的类型是Option<&'a mut Node<T>>, 可变引用没有实现copy trait, 所以self.next也是move的
        // 所以self.next会被move到map里去, 又因为self是个引用, 本身不具有next的所有权, 所以是不能move的

        // 所以改成了take, 把&mut Node<T> 交换出来
        let next = self.next.take();
        // 这里再被move到map中就没关系了
        next.map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push_left(1);
        list.push_left(2);

        let mut it = list.iter_mut();
        assert_eq!(it.next(), Some(&mut 2));
        assert_eq!(it.next(), Some(&mut 1));
        assert_eq!(it.next(), None);

        println!("{:?}", list);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push_left(1);
        list.push_left(2);

        let mut it = list.iter();
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);

        println!("{:?}", list);
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push_left(1);
        list.push_left(2);

        let mut it = list.into_iter();
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
    }

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