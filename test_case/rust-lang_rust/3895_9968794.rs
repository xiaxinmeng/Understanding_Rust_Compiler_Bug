
use core::vec::{any};

extern mod std;
use std::rope;

fn each_statement(text: rope::Rope) -> bool {
    enum IsSpace { Space, Printable }

    enum State {
        Start,
        StLabel,
        StKeyword,
        StExpr,
        TSpace,

        BadChar,  // TODO: line, col?
        BadSyntax, // line, col?
    }

    let mut state = Start;
    let mut ch1 = '\u0000'; // LL(2)

    do rope::loop_chars(text) |ch| {
        let cclass = if(" \t\r\n\x0A".contains_char(ch))
                        { Space } else { Printable };
        let is_label = any(&[('A', 'Z'), ('a', 'z'), ('0', '9')],
                           |r| match r { &(lo, hi) => ch >= lo && ch <= hi })
            || "'-_.".contains_char(ch);

        state = match (state, cclass) {
          (_, Printable) if ch > '\u007F' || ch < ' ' => BadChar,

          (Start, Space) => Start,
          (Start, _) if is_label => StLabel,
          (Start, _) => BadSyntax,  // not implemented

          (StLabel, _) if is_label => StLabel,
          (StLabel, Space) => StKeyword,
          (StLabel, _) => BadSyntax,

          (StKeyword, Space) => StKeyword,
          (StKeyword, _) if ch1 == '$' && ch == 'a' => StExpr,
          (StKeyword, _) => BadSyntax,

          (StExpr, _) if ch1 == '$' && ch == '.' => TSpace,
          (StExpr, _) if ch1 == '$' => BadSyntax,
          (StExpr, _) => StExpr,

          (TSpace, Space) => TSpace,
          (TSpace, Printable) => BadSyntax,  // not implemented

          (BadChar, _)
          | (BadSyntax, _) => fail fmt!("unexpected state: %?", state)
        };

        ch1 = ch;
        debug!("hi: %c", ch);
        match state {
            BadChar | BadSyntax => false,
            _ => true
        }
    };

    match state {
      TSpace => true,
      _ => false
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn each_statement1() {
        let doc = rope::of_str(@~"axiom.1 $a |- x = x $.");
        each_statement(doc);
    }
}
