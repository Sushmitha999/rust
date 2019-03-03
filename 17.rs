fn remove_at<T>(a: &mut Vec<T>, i: usize) -> &mut Vec<T> {
  a.remove(i);
  a
}

fn main() {
  let mut a = vec![1isize, 1, 2, 3, 3, 4, 4, 4, 5];
  println!("{:?}" , remove_at(&mut a, 4));
}