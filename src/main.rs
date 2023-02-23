
fn main() {
    let mut v: Vec<u32> = vec![7, 6, 5, 4, 3, 2, 1];
    v[2..5].sort_unstable();
    println!("{:?}", v);
}
