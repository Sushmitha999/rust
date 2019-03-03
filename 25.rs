fn primes(begin: usize, end: usize) -> Vec<usize> {
  let mut ps = vec![2];
  let mut i = 3;
  loop {
    if i >= end {
      break;
    }
    let mut flag = true;
    for &p in ps.iter() {
      if p*p > i {
        break;
      }
      if i % p == 0 {
        flag = false;
        break;
      }
    }
    if flag {
      ps.push(i);
    }
    i += 2;
  }
  ps.into_iter().skip_while(|&p| p < begin).collect()
}

fn main() {
  println!("{:?}", primes(1, 10));
  println!("{:?}", primes(123, 2014));
}