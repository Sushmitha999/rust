fn dupli<T: Clone>(a: Vec<T>) -> Vec<T> {
  a.into_iter().flat_map(|x| vec![x.clone(),x].into_iter()).collect()
}
//duplicating elements 
fn main() {
  let a = vec![1isize, 1, 2, 3, 3, 4, 5, 5, 5];
  println!("{:?}" , dupli(a));
}