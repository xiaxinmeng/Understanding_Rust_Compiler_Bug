plain
   Compiling unicase v2.6.0
error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:134:19
    |
134 |         Operator::try_from(*self).is_ok()
    |                   ^^^^^^^^ multiple `try_from` found
    |
    = note: candidate #1 is defined in an impl of the trait `TryFrom` for the type `T`
note: candidate #2 is defined in the trait `css::token::MyTryFrom`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:29:5
    |
29  |     fn try_from(value: T) -> Result<Self, Self::Error>;
help: disambiguate the associated function for candidate #1
    |
    |
134 |         TryFrom::try_from(*self).is_ok()
help: disambiguate the associated function for candidate #2
    |
    |
134 |         css::token::MyTryFrom::try_from(*self).is_ok()

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:368:38
    |
    |
368 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:96:5
    |
96  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
368 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
368 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:391:42
    |
    |
391 |             if let Ok(c) = ReservedChar::try_from(c) {
    |                                          ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:96:5
    |
96  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
391 |             if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
391 |             if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:401:38
    |
    |
401 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:96:5
    |
96  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
401 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
401 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:427:38
    |
    |
427 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:96:5
    |
96  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
427 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
427 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:457:45
    |
    |
457 |             if let Ok(s) = SelectorElement::try_from(&source[start - add..pos]) {
    |                                             ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `SelectorElement<'a>`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:203:5
    |
203 |     fn try_from(value: &'a str) -> Result<SelectorElement, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
457 |             if let Ok(s) = css::token::MyTryFrom::try_from(&source[start - add..pos]) {
help: disambiguate the associated function for candidate #2
    |
    |
457 |             if let Ok(s) = TryFrom::try_from(&source[start - add..pos]) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:503:38
    |
    |
503 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:96:5
    |
96  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
503 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
503 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:595:40
    |
    |
595 |                 let Ok(op) = Operator::try_from(c) => {
    |                                        ^^^^^^^^ multiple `try_from` found
    |
    = note: candidate #1 is defined in an impl of the trait `TryFrom` for the type `T`
note: candidate #2 is defined in the trait `css::token::MyTryFrom`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/css/token.rs:29:5
    |
29  |     fn try_from(value: T) -> Result<Self, Self::Error>;
help: disambiguate the associated function for candidate #1
    |
    |
595 |                 let Ok(op) = TryFrom::try_from(c) => {
help: disambiguate the associated function for candidate #2
    |
    |
595 |                 let Ok(op) = css::token::MyTryFrom::try_from(c) => {

    Checking crossbeam-queue v0.1.2
    Checking aho-corasick v0.7.13
    Checking tracing-log v0.1.2
    Checking tracing-log v0.1.2
error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:576:38
    |
576 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:115:5
    |
115 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
576 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
576 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:616:38
    |
    |
616 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:115:5
    |
115 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
616 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
616 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:659:42
    |
    |
659 |             if let Ok(c) = ReservedChar::try_from(c) {
    |                                          ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:115:5
    |
115 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
659 |             if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
659 |             if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:669:38
    |
    |
669 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:115:5
    |
115 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
669 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
669 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:695:38
    |
    |
695 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:115:5
    |
115 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
695 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
695 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:732:43
    |
    |
732 | ...                   ReservedChar::try_from(c)
    |                                     ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:115:5
    |
115 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
732 |                             js::token::MyTryFrom::try_from(c)
help: disambiguate the associated function for candidate #2
    |
732 |                             TryFrom::try_from(c)
    |                             ^^^^^^^^^^^^^^^^^
    |                             ^^^^^^^^^^^^^^^^^

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:770:33
    |
770 |         if let Ok(w) = Keyword::try_from(&source[start..pos]) {
    |                                 ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `Keyword`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:237:5
    |
237 |     fn try_from(value: &str) -> Result<Keyword, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
770 |         if let Ok(w) = js::token::MyTryFrom::try_from(&source[start..pos]) {
help: disambiguate the associated function for candidate #2
    |
    |
770 |         if let Ok(w) = TryFrom::try_from(&source[start..pos]) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:939:38
    |
    |
939 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:115:5
    |
115 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
939 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
939 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:999:40
    |
    |
999 |                 let Ok(o) = Operation::try_from(c) => {
    |                                        ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `Operation`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:376:5
    |
376 |     fn try_from(value: ReservedChar) -> Result<Operation, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
999 |                 let Ok(o) = js::token::MyTryFrom::try_from(c) => {
help: disambiguate the associated function for candidate #2
    |
    |
999 |                 let Ok(o) = TryFrom::try_from(c) => {

error[E0034]: multiple applicable items in scope
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:1002:40
     |
     |
1002 |                 let Ok(o) = Condition::try_from(c) => {
     |                                        ^^^^^^^^ multiple `try_from` found
     |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `Condition`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.37/src/js/token.rs:313:5
     |
313  |     fn try_from(value: ReservedChar) -> Result<Condition, Self::Error> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
     |
1002 |                 let Ok(o) = js::token::MyTryFrom::try_from(c) => {
help: disambiguate the associated function for candidate #2
     |
     |
1002 |                 let Ok(o) = TryFrom::try_from(c) => {

error: aborting due to 18 previous errors

For more information about this error, try `rustc --explain E0034`.
