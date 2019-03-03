fn compress<T: Eq>(a: Vec<T>) -> Vec<T> {
  let mut r = vec![];
  for x in a.into_iter() {
    if r.is_empty() || r.last().unwrap() != &x {
      r.push(x)
    }
  }
  r
}


fn main() {
  let a = vec![1i32,1,2,3,3,4,4,4,5,5,6,6,6,6];
  println!("{:?}", compress(a.clone()));
}