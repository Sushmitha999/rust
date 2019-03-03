fn phi(mut n: usize) -> usize {
  let mut r = n;
  let mut i = 2;
  while i*i <= n {
    if n%i == 0 {
      while n % i == 0 {
        n /= i
      }
      r = r/i*(i-1);
    }
    i += 1;
  }
  if n > 1 {
    r = r/n*(n-1);
  }
  r
}
////Euler's totient function:no. of integers co prime to a given number
fn main() {
  for n in 0..16 {
    println!("phi({}) = {}", n, phi(n));
  }
}