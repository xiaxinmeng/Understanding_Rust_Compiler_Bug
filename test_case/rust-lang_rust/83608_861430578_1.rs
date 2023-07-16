rust
  let components: [&mut dyn Component; 3] 
      = foo.get_components([TypeId::of::<T>(), TypeId::of::<U>(), TypeId::of::<V>()]) 
  
  fn get_components<const N usize>(&mut self, which: [TypeId; 3]) -> [&mut dyn Component; 3] {
      let indices: [usize; N] = self.find_indices(which);
      self.components.get_many_mut(indices).unwrap().map(|r| r as &mut dyn Component)
  }
  