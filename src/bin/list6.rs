#[derive(Copy, Clone, Debug)]
struct Person<T> {
    ele: T,
}

fn main() {
    let p = Some(Person {ele: 123});
    let a = p;
    println!("{:?}", p);
}

