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

fn reduce(s: &mut Vec<i64>, a: Atom) {
  match a {
    Atom::Void    => {},
    Atom::Num(x)  => s.push(x),
    Atom::Plus    => {
      let y = s.pop().unwrap();
      let x = s.pop().unwrap();
      println!("after double pop, just before push, stack = {:?}", s);
      let z = x + y;
      s.push(z);
    }
    Atom::Minus   => {
      let y = s.pop().unwrap();
      let x = s.pop().unwrap();
      let z = x - y;
      s.push(z);
    }
  }
}

fn main() {

  let mut stack = Vec::new();

  for a in env::args() {
    reduce(&mut stack, parse(&a));

    println!("stack = {:?}", stack);
  }

  for a in &stack {
    println!("{}", a);
  }
}
