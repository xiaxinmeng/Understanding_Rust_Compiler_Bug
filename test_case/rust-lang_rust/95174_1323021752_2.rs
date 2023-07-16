
   |
14 |     let _: Card<{Facing::Down}> = card;
   |            --------------------   ^^^^ expected `Facing::Down`, found `Facing::Up`
   |            |
   |            expected due to this
   |
   = note: expected struct `Card<Facing::Down>`
              found struct `Card<Facing::Up>`
