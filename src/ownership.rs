pub fn take_ownership(v: Vec<usize>) {
    println!("{:?}", v);
}

fn main() {
    let v: Vec<usize> = vec![1,2,3];
    take_ownership(v);
    println!("{:?}", v);
}