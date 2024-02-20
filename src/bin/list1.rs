#[derive(Debug)]
enum List {
    // 因为List<T>的大小不确定, 所以放在Box里
    Elem(i32, Box<List>),
    Empty,
}

fn main() {
    /*
    []表示栈, ()表示堆
    该的链表内存布局是这样的:
        [1, ptr] -> (2, ptr) -> (empty, junk)
    这样明显有2个坏处
    1. 链表拆分的时候, 堆上的数据需要拷贝到栈上
        [1, ptr] -> (empty, junk)
        [2, ptr] -> (empty, junk)
    2. 有一个没有用的junk值

    c语言风格链表是这样的:
    [ptr] -> (1, ptr) -> (2, null)
    拆分后:
        [ptr] -> (1, null)
        [ptr] -> (2, null)
     */
    let list = List::Elem(
        1,
        Box::new(
            List::Elem(
                2,
                Box::new(List::Empty),
            )
        ),
    );
    println!("{:?}", list);
}
