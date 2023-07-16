 rust
/// An iterator which walks over a directory
pub struct Directories {
    stack: Vec<std::vec::MoveItems<Path>>,
}

impl Iterator<Path> for Directories {
  fn next(&mut self) -> Option<Path> {
    enum LoopResult {
        AddAndReturn(std::vec::MoveItems<Path,Path),
        Return(Path),
        Pop,
    }
    loop {
      let loop_result;
      match self.stack.last() {
        Some(ref mut path_iter.next()) => {
          Some(path) {
            if (path.is_dir() {
              match readdir(&path) {
                Ok(dirs) => {
                  loop_result = AddAndReturn(dirs.move_iter(), path)
                },
                Err(..) => {
                  loop_result = Return(path)
                },
              }
            } else {
              loop_result = Return(path)
            }
          },
          None => {
            loop_result = Pop
          },
        }
        None => {
          return None,
        }
      };
      match loop_result {
        AddAndReturn(dirs,path) => {
          self.stack.push(dirs);
          return path
        },
        Return(path) => {
          return path
        },
        Pop => {
          self.stack.pop()
        },
      };
    }
  }
}
