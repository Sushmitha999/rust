use Elem::{Unique, Several};

#[derive(Debug)]
enum Elem<T> {
  Unique(T),
  Several(usize, T)
}

fn decode<T: Clone>(a: Vec<Elem<T>>) -> Vec<T> {
  a.into_iter().flat_map(|e|
    match e {
      Unique(x) => vec![x],
      Several(n, x) => std::iter::repeat(x).take(n).collect()
    }.into_iter()
  ).collect()
}
//decoding
fn main() {
  let a: Vec<Elem<isize>> = vec![Several(2usize, 1isize), Unique(2isize), Several(2usize, 3isize), Unique(4isize), Several(5usize,5isize)];
  println!("{:?}", decode(a));
}