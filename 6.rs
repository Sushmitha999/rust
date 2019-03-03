//palindrome
fn is_palindrome_iter<T: Eq>(a: &[T]) -> bool {
  a.iter().zip(a.iter().rev()).all(|(x,y)| x == y)
}

fn main() {
  let a = &[1i32,3,1,4];
  let b = &['a','b','a'];
  println!("{}", is_palindrome_iter(a));
  println!("{}", is_palindrome_iter(b));
}