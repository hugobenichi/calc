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

fn push_atom(s: &mut [Atom], c: &mut usize, a: Atom) {
  *c -= 1;
  s[*c] = a;
}

fn pop_atom(s: &mut [Atom], c: &mut usize) -> Atom {
  let a = s[*c];
  *c += 1;
  return a;
}

fn reduce_stack(s: &mut [Atom], c: &mut usize) {
  s[0] = pop_atom(s, c);
}

fn main() {

  let mut stack : [Atom; 10] = [Atom::Void; 10];
  let mut cursor : usize = 10;

  for a in env::args() {
    push_atom(&mut stack, &mut cursor, parse(&a));

    println!("stack = {}:{:?}", cursor, stack);
  }

  reduce_stack(&mut stack, &mut cursor);

  for a in &stack[cursor-1..] {
    println!("{:?}", a);
  }
}
