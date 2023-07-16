
use core::vec::{any};

extern mod std;
use std::rope;

fn each_statement(text: rope::Rope) -> bool {
    enum CharClass { Space, Printable, BadChar }

    enum State {
        Start,
        StLabel,
        StKeyword,
        StExpr,
        TSpace,

        BadSyntax, // line, col?
    }

    let mut state = Start;
    let mut ch1 = '\u0000'; // LL(2)

    do rope::loop_chars(text) |ch| {
        let cclass =
            if(" \t\r\n\x0A".contains_char(ch)) { Space }
            else if (ch > '\u007F' || ch < ' ') { BadChar }
            else { Printable };

        let is_label = any(&[('A', 'Z'), ('a', 'z'), ('0', '9')],
                           |r| match r { &(lo, hi) => ch >= lo && ch <= hi })
            || "'-_.".contains_char(ch);

        state = match (state, cclass, is_label, ch1, ch) {
          (_, BadChar, _, _, _) => BadSyntax,

          (Start, Space, _, _, _) => Start,
          (Start, Printable, true, _, _) => StLabel,
          (Start, _, _, _, _) => BadSyntax,  // not implemented

          (StLabel, Printable, true, _, _) => StLabel,
          (StLabel, Space, _, _, _) => StKeyword,
          (StLabel, _, _, _, _) => BadSyntax,

          (StKeyword, Space, _, _, _) => StKeyword,
          (StKeyword, Printable, _, '$', 'a') => StExpr,
          (StKeyword, _, _, _, _) => BadSyntax,

          (StExpr, _, _, '$', '.') => TSpace,
          (StExpr, _, _, '$', _)  => BadSyntax,
          (StExpr, _, _, _, _) => StExpr,

          (TSpace, Space, _, _, _) => TSpace,
          (TSpace, Printable, _, _, _) => BadSyntax,  // not implemented

          (BadSyntax, _, _, _, _) => fail fmt!("unexpected state: %?", state)
        };

        ch1 = ch;
        debug!("hi: %c", ch);
        match state {
            BadSyntax => false,
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
