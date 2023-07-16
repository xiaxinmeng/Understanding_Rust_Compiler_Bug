rust
for _ in 2..k {
//before————————————————————————-
                        l_kth = &l_kth.as_ref().unwrap().next;
			if l_kth.is_none() {
				return head;
			}
//-—————————————————————————————-
//after——————————————————————————
			l_kth = match l_kth.as_ref() {
				None => return head,
				Some(n,) => &n.next,
			};
		}
//-—————————————————————————————-
