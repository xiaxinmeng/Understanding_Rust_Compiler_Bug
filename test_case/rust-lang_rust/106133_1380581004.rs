 rust
fn countlines()->usize{ 
      let mut cnt:usize = 0;
      let mut stdin = io::stdin().lock();
      loop{
          let used = {
              let available = match stdin.fill_buf() {
              Ok(n) => n,          
              Err(e) => {panic!("AAA")},
              };
              if available.len() == 0 {
                  break;
              }
 
              for b in available{
                  cnt += (*b == ('\n' as u8)) as usize;
              }
              
              available.len()
          };
          stdin.consume(used);
      }
      cnt
  }
