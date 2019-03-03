fn repli<T: Clone>(n: usize, a: Vec<T>) -> Vec<T> {
  a.into_iter().flat_map(|x| std::iter::repeat(x).take(n)).collect()
}
//replicating elements for a given number of times
fn main() {
  let a = vec![1isize, 2, 3, 3, 4, 5];
  println!("{:?}" , repli(4, a));
}