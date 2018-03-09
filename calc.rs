use std::env;

#[derive(Debug, Clone, Copy)]
enum Atom {
  Void,
  Num(i64),
  Plus,
  Minus,
  Mul,
  Div,
  Mod,
  Pow,
}

fn parse(atom: &str) -> Atom {
  match atom {
    "+" => return Atom::Plus,
    "-" => return Atom::Minus,
    "*" => return Atom::Mul,
    "/" => return Atom::Div,
    "%" => return Atom::Mod,
    "^" => return Atom::Pow,
    _   => (),
  }
  match atom.parse::<i64>() {
    Ok(x) => return Atom::Num(x),
    _     => return Atom::Void,
  }
}

fn apply_binary_op(s: &mut Vec<i64>, f: fn(i64, i64) -> i64) {
  let y = s.pop().unwrap();
  let x = s.pop().unwrap();
  let z = f(x, y);
    println!("after double pop, before push stack = {:?}", s);
  s.push(z);
}

fn reduce(s: &mut Vec<i64>, a: Atom) {
  match a {
    Atom::Void    => {},
    Atom::Num(x)  => s.push(x),
    Atom::Plus    => apply_binary_op(s, |x, y| x + y),
    Atom::Minus   => apply_binary_op(s, |x, y| x - y),
    Atom::Mul     => apply_binary_op(s, |x, y| x * y),
    Atom::Div     => apply_binary_op(s, |x, y| x / y),
    Atom::Mod     => apply_binary_op(s, |x, y| x - (x / y) * y), // poor man's modulo
    Atom::Pow     => apply_binary_op(s, |x, y| x.pow(y as u32)),
  }
}

fn main() {

  let mut stack = Vec::new();

  for a in env::args() {
    // TODO: split 'a' on spaces
    reduce(&mut stack, parse(&a));
  }

  println!("stack = {:?}", stack);
}
