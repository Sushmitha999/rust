fn main() {
  let a = vec![1i32,3,5,7i32,8];
  let len = a.len();
  println!("{:?}", &a[len-2]);
}