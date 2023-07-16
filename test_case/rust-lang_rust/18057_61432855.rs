 rust
#![feature(unboxed_closures, overloaded_calls, unboxed_closure_sugar)]

pub trait Seq<T> {
    fn pop(self) -> Result<(T, Self, Self), Self>;
}

impl<'a> Seq<char> for &'a str {
    fn pop(self) -> Result<(char, &'a str, &'a str), &'a str> {
        if self.is_empty() {
            Err(self)
        } else {
            Ok((self.char_at(0), self, self.slice_from(1)))
        }
    }
}

type ParseResult<S,T> = Result<(T, S, S),S>;
type Parser<S,T> = Box<|&: S| -> ParseResult<S,T>>;

fn elem<T, S:Seq<T>, Res>(e:Box<|&: T| -> Option<Res>>) -> Parser<S, Res> {
  box move |&: s:S| {
    match s.pop() {
      Err(b) => Err(b),
      Ok((c, b, n)) => match e.call((c,)) {
        Some(r) => Ok((r, b, n)),
        None => Err(b)
      }
    }
  }
}

fn concat<C,S:Seq<C>,T1,T2>(p1: Parser<S, T1>, p2: Parser<S, T2>) -> Parser<S,(T1,T2)> {
  box move |&: s:S| {
    match p1.call((s,)) {
       Err(b) => Err(b),
       Ok((r1, b, n1)) => {
         match p2.call((n1,)) {
            Err(_) => Err(b),
            Ok((r2, _, n2)) => Ok(((r1,r2), b, n2))
         }
       }
    }
  }
}

fn alter<C,S:Seq<C>,T>(p1: Parser<S, T>, p2: Parser<S, T>) -> Parser<S, T> {
  box move |&: s:S| {
    match p1.call((s,)) {
       Err(b) => p2.call((b,)),
       r => r
    }
  }
}

fn digit(r:&int) -> Box<|&: char| -> Option<int>> {
  let r = *r;
  box move |&: c:char| {
   let i = match c {
     '0'...'9' => { c as int - '0' as int },
     'a'...'z' => { 10 + c as int - 'a' as int },
     'A'...'Z' => { 10 + c as int - 'A' as int },
     _ => r
   };
   if i < r {
    Some(i)
   } else {
    None
   }
  }
}

fn do_fold<S,T,R>(p:&Parser<S,T>, l:&Box<|&: T,R| -> R>, b:S, i:R, s:S) -> ParseResult<S,R> {
  match p.call((s,)) {
    Err(n) => Ok((i,b,n)),
    Ok((r,_,n)) => {
      let v = l.call((r,i));
      do_fold(p,l,b,v,n)
    }
  }
}

fn fold1<S,T,R:Clone+'static>(p:Parser<S,T>, l: Box<|&: T,R| -> R>, i:R) -> Parser<S,R> {
  box move |&: s:S| {
    match p.call((s,)) {
      Err(e) => Err(e),
      Ok((r,b,n)) => {
        let v = l.call((r,i.clone()));
        do_fold(&p,&l,b,v,n)
      }
    }
  }
}

fn main() {
    static RADIX:int = 13;
    let p1 = fold1(elem(digit(&RADIX)), box |&: c:int, v:int| -> int {v*RADIX+c}, 0i) ;
    let p2 = alter(elem(box |&: c:char| { if c == 'z' { Some('Z') } else { None } }),
                   elem(box |&: c:char| { if c == 'w' { Some('W') } else { None } })) ;
    println!("{}", concat(p1, p2).call(("1awert",)));
}
