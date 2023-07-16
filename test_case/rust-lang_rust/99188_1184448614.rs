console
$ curl -s https://gist.githubusercontent.com/fasterthanlime/c3f5ab67fd6b12d198ef2dd841c31115/raw/756847298cd40474e6df1c8440805c8e884a0995/depth3.txt | rg 'evaluate_predicate_recursively' | sed -E 's/([0-9].* DEBUG )?rustc_\S*::\S* //;s/obligation=Obligation\(//;s/, depth=.*//;s/[┐│├─]/  /g;s/^  //'
(): Sized
(): Sized
(): Sized
for<'x> <() as Trait<'x>>::A == ()
for<'x> <() as Trait<'x>>::B == ()
for<'x> <() as Trait<'x>>::C == ()
for<'x> <() as Trait<'x>>::D == ()
for<'x> (): Trait<'x>
(): Termination
for<'x> <&&() as Trait<'x>>::A == ()
       for<'x> <&() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::D == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       &(): Sized
       for<'x> &(): Trait<'x>
                      for<'x> <() as Trait<'x>>::A == ()
                      for<'x> <() as Trait<'x>>::B == ()
                      for<'x> <() as Trait<'x>>::C == ()
                      for<'x> <() as Trait<'x>>::D == ()
                      (): Sized
                      for<'x> (): Trait<'x>
for<'x> <&&() as Trait<'x>>::B == ()
       for<'x> <&() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::D == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       &(): Sized
       for<'x> &(): Trait<'x>
for<'x> <&&() as Trait<'x>>::C == ()
       for<'x> <&() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::D == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       &(): Sized
       for<'x> &(): Trait<'x>
for<'x> <&&() as Trait<'x>>::D == ()
       for<'x> <&() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       for<'x> <&() as Trait<'x>>::D == ()
             for<'x> <() as Trait<'x>>::A == ()
             for<'x> <() as Trait<'x>>::B == ()
             for<'x> <() as Trait<'x>>::C == ()
             for<'x> <() as Trait<'x>>::D == ()
             (): Sized
             for<'x> (): Trait<'x>
       &(): Sized
       for<'x> &(): Trait<'x>
&&(): Sized
for<'x> &&(): Trait<'x>
                for<'x> <&() as Trait<'x>>::A == ()
                      for<'x> <() as Trait<'x>>::A == ()
                      for<'x> <() as Trait<'x>>::B == ()
                      for<'x> <() as Trait<'x>>::C == ()
                      for<'x> <() as Trait<'x>>::D == ()
                      (): Sized
                      for<'x> (): Trait<'x>
                for<'x> <&() as Trait<'x>>::B == ()
                      for<'x> <() as Trait<'x>>::A == ()
                      for<'x> <() as Trait<'x>>::B == ()
                      for<'x> <() as Trait<'x>>::C == ()
                      for<'x> <() as Trait<'x>>::D == ()
                      (): Sized
                      for<'x> (): Trait<'x>
                for<'x> <&() as Trait<'x>>::C == ()
                      for<'x> <() as Trait<'x>>::A == ()
                      for<'x> <() as Trait<'x>>::B == ()
                      for<'x> <() as Trait<'x>>::C == ()
                      for<'x> <() as Trait<'x>>::D == ()
                      (): Sized
                      for<'x> (): Trait<'x>
                for<'x> <&() as Trait<'x>>::D == ()
                      for<'x> <() as Trait<'x>>::A == ()
                      for<'x> <() as Trait<'x>>::B == ()
                      for<'x> <() as Trait<'x>>::C == ()
                      for<'x> <() as Trait<'x>>::D == ()
                      (): Sized
                      for<'x> (): Trait<'x>
                &(): Sized
                for<'x> &(): Trait<'x>
for<'x> <&() as Trait<'x>>::A == ()
       for<'x> <() as Trait<'x>>::A == ()
       for<'x> <() as Trait<'x>>::B == ()
       for<'x> <() as Trait<'x>>::C == ()
       for<'x> <() as Trait<'x>>::D == ()
       (): Sized
       for<'x> (): Trait<'x>
for<'x> <&() as Trait<'x>>::B == ()
       for<'x> <() as Trait<'x>>::A == ()
       for<'x> <() as Trait<'x>>::B == ()
       for<'x> <() as Trait<'x>>::C == ()
       for<'x> <() as Trait<'x>>::D == ()
       (): Sized
       for<'x> (): Trait<'x>
for<'x> <&() as Trait<'x>>::C == ()
       for<'x> <() as Trait<'x>>::A == ()
       for<'x> <() as Trait<'x>>::B == ()
       for<'x> <() as Trait<'x>>::C == ()
       for<'x> <() as Trait<'x>>::D == ()
       (): Sized
       for<'x> (): Trait<'x>
for<'x> <&() as Trait<'x>>::D == ()
       for<'x> <() as Trait<'x>>::A == ()
       for<'x> <() as Trait<'x>>::B == ()
       for<'x> <() as Trait<'x>>::C == ()
       for<'x> <() as Trait<'x>>::D == ()
       (): Sized
       for<'x> (): Trait<'x>
&(): Sized
for<'x> &(): Trait<'x>
