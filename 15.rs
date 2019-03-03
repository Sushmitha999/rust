fn drop<T>(a: Vec<T>, n: usize) -> Vec<T> {
  a.into_iter().enumerate().filter_map(|(i,x)|
    if i%n < n-1 { Some(x) } else { None }
    ).collect()
}
//dropping every nth element from a list
fn main() {
  let a = vec![1isize, 1, 2, 3, 3, 4, 5, 6, 7, 8];
  println!("{:?}" , drop(a, 3));
}