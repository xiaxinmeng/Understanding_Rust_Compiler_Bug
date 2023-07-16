
let this = mem::replace(self, Default::default());
*self = this.into_iter().filter(...).collect();
