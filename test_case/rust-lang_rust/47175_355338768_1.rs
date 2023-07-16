rust
    fn main() {
        let pb_ident = "p.q";
        let mut ident_path = pb_ident.split('.');
        ident_path.next_back();
        let result = ident_path.collect::<Vec<_>>();
        assert_eq!(result, vec!["p"]);
    }
    