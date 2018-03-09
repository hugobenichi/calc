use std::env;

fn main() {

  let mut ns = Vec::new();

  for a in env::args() {
    println!("{}", a);

    match a.parse::<i64>() {
      Ok(x) => ns.push(x),
      _     => (),
    }
  }

  let mut t : i64 = 0;
  for x in &ns {
    t += x;
  }
  println!("{}", t);
}
