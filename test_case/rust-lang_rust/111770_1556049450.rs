plain
   Compiling diff v0.1.12
   Compiling yansi v0.5.1
   Compiling pretty_assertions v1.3.0
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0425]: cannot find function `extract_beta_rev` in this scope
    |
    |
152 |     assert_eq!(extract_beta_rev("1.99.9-beta.7"), Some("7"));
    |
help: consider importing this function
    |
1   + use crate::extract_beta_rev;
1   + use crate::extract_beta_rev;
    |

error[E0425]: cannot find function `extract_beta_rev` in this scope
    |
    |
154 |     assert_eq!(extract_beta_rev("1.99.9-beta.777"), Some("777"));
    |
help: consider importing this function
    |
1   + use crate::extract_beta_rev;
1   + use crate::extract_beta_rev;
    |

error[E0425]: cannot find function `extract_beta_rev` in this scope
    |
    |
156 |     assert_eq!(extract_beta_rev("1.99.9-nightly"), None);
    |
help: consider importing this function
    |
1   + use crate::extract_beta_rev;
1   + use crate::extract_beta_rev;
    |

error[E0425]: cannot find function `extract_beta_rev` in this scope
    |
    |
158 |     assert_eq!(extract_beta_rev("1.99.9"), None);
    |
help: consider importing this function
    |
1   + use crate::extract_beta_rev;
1   + use crate::extract_beta_rev;
    |

error[E0425]: cannot find function `extract_beta_rev` in this scope
    |
    |
160 |     assert_eq!(extract_beta_rev("invalid"), None);
    |
help: consider importing this function
    |
1   + use crate::extract_beta_rev;
