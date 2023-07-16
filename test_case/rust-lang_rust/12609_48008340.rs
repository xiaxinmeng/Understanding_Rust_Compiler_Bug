
type foo;;

let x: foo = Obj.magic 0 in
match x with
| _ -> print_endline "hello darkness my old friend";; (* OK *)
