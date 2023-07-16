
; <std::collections::hash::map::InternalEntry<K, V, &'a mut std::collections::hash::table::RawTable<K, V>>>::into_entry

  %40 = getelementptr inbounds %"std::collections::hash::map::VacantEntry<(i32, i32), ()>", %"std::collections::hash::map::VacantEntry<(i32, i32), ()>"* %_13, i32 0, i32 5
  %41 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %40, i32 0, i32 0
  store i32 %key.0, i32* %41, align 8
  %42 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %40, i32 0, i32 1
  store i32 %key.1, i32* %42, align 8
