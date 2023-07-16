 rust
ga(10, 20, 30); // 20 and 30 are automagically
                // converted to Some(20) and Some(30)
ga(10, 20);         // ga(10, 20, 99) 
ga(10);             // ga(10, 42, 99)
ga(10, None, None); // ga(10, 42, 99)
ga(10, 20, None);   // ga(10, 20, 99)
ga(10, None, 30);   // ga(10, 42, 30)
