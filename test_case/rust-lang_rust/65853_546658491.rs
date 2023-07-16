
function inputs:   A B C
      arguments:   A B C    // Typecheck!
      arguments:   A D C    // D is incorrect!
      arguments:   A C      // Forgot B!
      arguments:   A B D C  // D is extra!
      arguments:   A C B    // B and C are swapped!
