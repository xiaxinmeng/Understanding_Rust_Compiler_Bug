rust
let trex = TyrannosaurusRex::new();
let is_a_dog = has_a_tail(trex)
  && has_bad_breath(trex)
  && is_a_carnivore(trex); // Misplaced semi-colon (perhaps due to reordering of lines)
  && is_adorable(trex);

if is_a_dog {
    give_hug(trex); // Ouch!
}
