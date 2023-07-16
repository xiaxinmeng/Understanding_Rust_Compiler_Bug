
unsafe { &mut *(self.f() as *const Self as *mut Self) }
         ^
