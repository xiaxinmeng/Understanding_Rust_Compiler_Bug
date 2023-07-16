plain
   Compiling unicase v2.6.0
error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:132:19
    |
132 |         Operator::try_from(*self).is_ok()
    |                   ^^^^^^^^ multiple `try_from` found
    |
    = note: candidate #1 is defined in an impl of the trait `TryFrom` for the type `T`
note: candidate #2 is defined in the trait `css::token::MyTryFrom`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:29:5
    |
29  |     fn try_from(value: T) -> Result<Self, Self::Error>;
help: disambiguate the associated function for candidate #1
    |
    |
132 |         TryFrom::try_from(*self).is_ok()
help: disambiguate the associated function for candidate #2
    |
    |
132 |         css::token::MyTryFrom::try_from(*self).is_ok()

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:372:38
    |
    |
372 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:93:5
    |
93  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
372 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
372 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:392:42
    |
    |
392 |             if let Ok(c) = ReservedChar::try_from(c) {
    |                                          ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:93:5
    |
93  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
392 |             if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
392 |             if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:402:38
    |
    |
402 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:93:5
    |
93  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
402 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
402 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:424:38
    |
    |
424 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:93:5
    |
93  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
424 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
424 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:447:45
    |
    |
447 |             if let Ok(s) = SelectorElement::try_from(&source[start - add..pos]) {
    |                                             ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `SelectorElement<'a>`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:198:5
    |
198 |     fn try_from(value: &'a str) -> Result<SelectorElement, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
447 |             if let Ok(s) = css::token::MyTryFrom::try_from(&source[start - add..pos]) {
help: disambiguate the associated function for candidate #2
    |
    |
447 |             if let Ok(s) = TryFrom::try_from(&source[start - add..pos]) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:482:38
    |
    |
482 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `css::token::MyTryFrom` for the type `css::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:93:5
    |
93  |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
482 |         if let Ok(c) = css::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
482 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:564:40
    |
    |
564 |                 let Ok(op) = Operator::try_from(c) => {
    |                                        ^^^^^^^^ multiple `try_from` found
    |
    = note: candidate #1 is defined in an impl of the trait `TryFrom` for the type `T`
note: candidate #2 is defined in the trait `css::token::MyTryFrom`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/css/token.rs:29:5
    |
29  |     fn try_from(value: T) -> Result<Self, Self::Error>;
help: disambiguate the associated function for candidate #1
    |
    |
564 |                 let Ok(op) = TryFrom::try_from(c) => {
help: disambiguate the associated function for candidate #2
    |
    |
564 |                 let Ok(op) = css::token::MyTryFrom::try_from(c) => {

    Checking crossbeam-queue v0.1.2
    Checking tracing-log v0.1.2
error[E0034]: multiple applicable items in scope
error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:585:38
    |
585 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:110:5
    |
110 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
585 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
585 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:625:38
    |
    |
625 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:110:5
    |
110 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
625 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
625 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:665:42
    |
    |
665 |             if let Ok(c) = ReservedChar::try_from(c) {
    |                                          ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:110:5
    |
110 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
665 |             if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
665 |             if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:675:38
    |
    |
675 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:110:5
    |
110 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
675 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
675 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:697:38
    |
    |
697 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:110:5
    |
110 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
697 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
697 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:720:33
    |
    |
720 |         if let Ok(w) = Keyword::try_from(&source[start..pos]) {
    |                                 ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `Keyword`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:232:5
    |
232 |     fn try_from(value: &str) -> Result<Keyword, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
720 |         if let Ok(w) = js::token::MyTryFrom::try_from(&source[start..pos]) {
help: disambiguate the associated function for candidate #2
    |
    |
720 |         if let Ok(w) = TryFrom::try_from(&source[start..pos]) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:889:38
    |
    |
889 |         if let Ok(c) = ReservedChar::try_from(c) {
    |                                      ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `js::token::ReservedChar`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:110:5
    |
110 |     fn try_from(value: char) -> Result<ReservedChar, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
889 |         if let Ok(c) = js::token::MyTryFrom::try_from(c) {
help: disambiguate the associated function for candidate #2
    |
    |
889 |         if let Ok(c) = TryFrom::try_from(c) {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:945:40
    |
    |
945 |                 let Ok(o) = Operation::try_from(c) => {
    |                                        ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `Operation`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:351:5
    |
351 |     fn try_from(value: ReservedChar) -> Result<Operation, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
945 |                 let Ok(o) = js::token::MyTryFrom::try_from(c) => {
help: disambiguate the associated function for candidate #2
    |
    |
945 |                 let Ok(o) = TryFrom::try_from(c) => {

error[E0034]: multiple applicable items in scope
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:948:40
    |
    |
948 |                 let Ok(o) = Condition::try_from(c) => {
    |                                        ^^^^^^^^ multiple `try_from` found
    |
note: candidate #1 is defined in an impl of the trait `js::token::MyTryFrom` for the type `Condition`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/minifier-0.0.33/src/js/token.rs:305:5
    |
305 |     fn try_from(value: ReservedChar) -> Result<Condition, Self::Error> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: candidate #2 is defined in an impl of the trait `TryFrom` for the type `T`
help: disambiguate the associated function for candidate #1
    |
948 |                 let Ok(o) = js::token::MyTryFrom::try_from(c) => {
help: disambiguate the associated function for candidate #2
    |
    |
948 |                 let Ok(o) = TryFrom::try_from(c) => {

    Checking aho-corasick v0.7.13
error: aborting due to 17 previous errors

