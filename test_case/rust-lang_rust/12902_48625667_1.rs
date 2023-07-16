 rust
select!{
  obj1_opt from rx1 using recv_opt => {
    // Do stuff with obj1_opt
  },
  obj2_opt from rx2 using recv_opt => {
    // Do stuff with obj2_opt
  }
}
