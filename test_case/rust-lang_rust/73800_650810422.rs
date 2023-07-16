rust
         self.write(&unsafe { mem::transmute::<_, [u8; 4]>(i) }) 
