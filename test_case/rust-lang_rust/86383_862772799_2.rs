
          StorageLive(_7); 
          _7 = _1;
          _8 = Len((*_2));
          _9 = Lt(_7, _8);
          assert(move _9, "index out of bounds: the length is {} but the index is {}", move _8, _7) -> bb4;
