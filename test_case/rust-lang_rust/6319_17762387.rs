
<ssbr> rusti: match Some((1u, 'a')) { (x, _) => 1u, _ => 2u }
<rusti> error: internal compiler error: Asked to compute contents of fictitious type
<ssbr> rusti: match Some((1u, 'a')) { (_, _) => 1u, _ => 2u }
<rusti> error: internal compiler error: adt::represent_type called on non-ADT type
