            .  fn unroll_place<'tcx, R>(
            .      place: &Place<'tcx>,
            .      next: Option<&PlaceComponents<'_, 'tcx>>,
            .      op: impl FnOnce(PlaceComponentsIter<'_, 'tcx>) -> R,
            .  ) -> R {
            .      match place {
8,892,046,598          Place::Projection(interior) => unroll_place(
42,799,560,024  => /home/njn/moz/rust0/src/librustc_mir/borrow_check/places_conflict.rs:rustc_mir::borrow_check::places_conflict::unroll_place'2 (485791408x)
  534,414,126              &interior.base,
1,068,828,252              Some(&PlaceComponents {
            .                  component: place,
            .                  next,
            .              }),
2,672,070,630              op,
            .          ),
            .
            .          Place::Local(_) | Place::Static(_) => {
1,943,739,512              let list = PlaceComponents {
            .                  component: place,
            .                  next,
            .              }; 
1,943,739,512              op(list.iter())
            .          }
            .      }
6,754,274,663  }
