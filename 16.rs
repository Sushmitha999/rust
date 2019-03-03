fn split<'a, T>(a: &'a [T], n: usize) -> (&'a [T], &'a [T]) {
  a.split_at(std::cmp::min(n, a.len()))
}
//splitting into half
fn main() {
  let a = &[1isize, 1, 2, 3, 3, 4, 5 ,6 ,8, 8, 9, 9];
  println!("{:?}" , split(a, a.len()/2));
}