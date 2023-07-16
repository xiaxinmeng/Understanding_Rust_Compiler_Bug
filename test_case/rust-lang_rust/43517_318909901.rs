

define i16 @"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$3all28_$u7b$$u7b$closure$u7d$$u7d$17h5b298b7604579d38E"(%closure* nocapture readnone, i32* noalias nocapture readonly dereferenceable(4)) unnamed_addr #0 {
start:
  %2 = load i32, i32* %1, align 4
  %not. = icmp eq i32 %2, 1
  %. = zext i1 %not. to i16
  ret i16 %.
}
