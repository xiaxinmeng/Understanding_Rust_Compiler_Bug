rust

fn main() {

    #[inline]
    let _a = 4;

    #[inline(UUUU)]
    let _b = 4;

    #[repr(nothing)]
    let _x = 0;


    #[repr(something_not_real)]
    {
    	1
    };
}
