
rustc \
  --opt-level 2 \
  -Z no-landing-pads \
  -g \
  --cfg cfg_mcu_has_spi --cfg cfg_tft_lcd --cfg cfg_multitasking --cfg mcu_lpc17xx --cfg arch_cortex_m3 \
  --target thumbv7m-linux-eabi \
  -Ctarget-cpu=cortex-m3 \
  -Crelocation_model=static
