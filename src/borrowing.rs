pub fn borrow(v: Vec<usize>) {
    println!("{:?}", v);
}

fn main() {
    let v: Vec<usize> = vec![1,2,3];
    borrow(v);
    println!("{:?}", v);
}