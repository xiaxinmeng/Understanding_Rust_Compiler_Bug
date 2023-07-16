rust
#![allow(unused_variables)]

type ExpInt = i16;

type Limb = u128;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Loss {
    ExactlyZero,
}

fn shift_left(dst: &mut [Limb], exp: &mut ExpInt, bits: usize) {
    loop {}
}

fn shift_right(dst: &mut [Limb], exp: &mut ExpInt, bits: usize) -> Loss {
    loop {}
}

fn sub(a: &mut [Limb], b: &[Limb], c: Limb) -> Limb {
    loop {}
}

pub fn add_or_sub(
    a_sig: &mut [Limb],
    a_exp: &mut ExpInt,
    a_sign: &mut bool,
    b_sig: &mut [Limb],
    b_exp: ExpInt,
    b_sign: bool,
) {
    let bits = *a_exp - b_exp;

    let (reverse, loss);

    if bits == 0 {
        loop {}
    } else if bits > 0 {
        loss = shift_right(b_sig, &mut 0, (bits - 1) as usize);
        shift_left(a_sig, a_exp, 1);
        reverse = false;
    } else {
        loss = shift_right(a_sig, a_exp, (-bits - 1) as usize);
        shift_left(b_sig, &mut 0, 1);
        reverse = true;
    }

    let borrow = (loss != Loss::ExactlyZero) as Limb;
    if reverse {
        assert_eq!(sub(b_sig, a_sig, borrow), 0);
    }
}
