use std::env;

#[derive(Debug, Clone, Copy)]
enum Atom {
  Void,
  Num(i64),
  Plus,
  Minus,
}

fn parse(atom: &str) -> Atom {
  match atom {
    "+" => return Atom::Plus,
    "-" => return Atom::Minus,
    _   => (),
  }
  match atom.parse::<i64>() {
    Ok(x) => return Atom::Num(x),
    _     => return Atom::Void,
  }
}

fn push_num(s: &mut [i64], c: &mut usize, a: i64) {
  *c -= 1;
  s[*c] = a;
}

fn pop_num(s: &mut [i64], c: &mut usize) -> i64 {
  let a = s[*c];
  *c += 1;
  return a;
}

fn reduce(s: &mut [i64], c: &mut usize, a: Atom) {
  match a {
    Atom::Void    => {},
    Atom::Num(x)  => push_num(s, c, x),
    Atom::Plus    => {
      let y = pop_num(s, c);
      let x = pop_num(s, c);
      println!("after double pop, just before push, stack = {}:{:?}", c, s);
      let z = x + y;
      push_num(s, c, z);
    }
    Atom::Minus   => {
      let y = pop_num(s, c);
      let x = pop_num(s, c);
      let z = x - y;
      push_num(s, c, z);
    }
  }
}

fn main() {

  let mut stack : [i64; 10] = [0; 10];
  let mut cursor : usize = 10;

  for a in env::args() {
    reduce(&mut stack, &mut cursor, parse(&a));

    println!("stack = {}:{:?}", cursor, stack);
  }

  for a in &stack[cursor..] {
    println!("{}", a);
  }
}
