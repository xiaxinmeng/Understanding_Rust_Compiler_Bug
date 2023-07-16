
export F="stm32f401,stm32f405"; rm -rf target; cargo +local-bare check; time cargo +local-bare check --features "$F"; rm -rf target/; cargo +local check; time cargo +local check --features "$F"
