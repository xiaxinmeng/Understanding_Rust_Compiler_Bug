
-*- mode: ack; default-directory: "~/Dev/Mozilla/rust.git/src/" -*-
Ack started at Sun Jul  6 19:14:56

ack -A 3 'deriving.*Ord' | grep 'enum '
libcore/option.rs-152-pub enum Option<T> {
libcore/result.rs-288-pub enum Result<T, E> {
libglob/lib.rs-204-enum PatternToken {
libglob/lib.rs-213-enum CharSpecifier {
libnum/bigint.rs-787-pub enum Sign { Minus, Zero, Plus }
librustc/back/link.rs-49-pub enum OutputType {
librustc/driver/config.rs-138-pub enum CrateType {
librustc/lint/mod.rs-208-pub enum Level {
librustc/middle/subst.rs-228-pub enum ParamSpace {
librustc/middle/ty.rs-653-pub enum BoundRegion {
libserialize/json.rs-156-pub enum Json {
libsyntax/attr.rs-372-pub enum StabilityLevel {
test/compile-fail/deriving-span-PartialOrd-enum-struct-variant.rs-20-enum Enum {
test/compile-fail/deriving-span-PartialOrd-enum.rs-20-enum Enum {
test/compile-fail/deriving-span-TotalOrd-enum-struct-variant.rs-20-enum Enum {
test/compile-fail/deriving-span-TotalOrd-enum.rs-20-enum Enum {
test/run-pass/deriving-cmp-generic-enum.rs-14-enum E<T> {
test/run-pass/deriving-cmp-generic-struct-enum.rs-16-enum ES<T> {

Ack finished at Sun Jul  6 19:15:11
