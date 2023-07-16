
error[E0658]: paths of length greater than one in macro invocations are currently unstable
    --> /Users/jcsoo/bobbin-dev/bobbin-dev/mcu/bobbin-stm32/stm32f74x/src/irq.rs:1413:1
     |
1413 | ::bobbin_mcu::irq!(::dma::Dma2Stream7, IrqDma, Irq70);
     | ^^^^^^^^^^^^^^^^^
     |
     = help: add #![feature(proc_macro_path_invoc)] to the crate attributes to enable
