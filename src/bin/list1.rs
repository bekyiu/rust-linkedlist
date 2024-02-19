#[derive(Debug)]
enum List<T> {
    // 因为List<T>的大小不确定, 所以放在Box里
    Elem(T, Box<List<T>>),
    Empty,
}

fn main() {
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
