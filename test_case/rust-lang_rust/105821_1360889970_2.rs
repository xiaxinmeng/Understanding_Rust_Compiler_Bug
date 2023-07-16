rs
        const ARRA: [u8; A.len()] = str_to_arr::<A>();
        const ARRB: [u8; B.len()] = str_to_arr::<B>();
        const ARRJOIN: [u8; A.len() + B.len()] = concat_arrs(Self::ARRA, Self::ARRB);
        const STRJOIN: &str = unsafe { std::str::from_utf8_unchecked(&Self::ARRJOIN) };
