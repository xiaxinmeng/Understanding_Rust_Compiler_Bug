
do receive |value| {
      match value {
              Left(x) => x,
              Right(void) => void.absurd () // Extra feature of sometimes receiving alternative value is unused
       }
}
