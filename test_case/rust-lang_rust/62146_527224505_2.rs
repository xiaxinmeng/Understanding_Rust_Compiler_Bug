diff
diff --git a/Cargo.toml b/Cargo.toml
index b458e18..1c968e3 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -46,7 +46,8 @@ version = "^2.0"
 default-features = false

 [dependencies.lexical-core]
-version = "^0.4.0"
+path = "../../rust-lexical/lexical-core"
+version = "^0.6.0"
 optional = true

 [dev-dependencies]
diff --git a/src/number/complete.rs b/src/number/complete.rs
index 28abfdc..51bcd06 100644
--- a/src/number/complete.rs
+++ b/src/number/complete.rs
@@ -832,16 +832,9 @@ pub fn float<T, E:ParseError<T>>(input: T) -> IResult<T, f32, E>
 where
   T: crate::traits::AsBytes + InputLength + Slice<RangeFrom<usize>>,
 {
-  let res = ::lexical_core::try_atof32_slice(input.as_bytes());
-
-  match res.error.code {
-    ::lexical_core::ErrorCode::Success => Ok((input.slice(input.input_len()..), res.value)),
-    ::lexical_core::ErrorCode::InvalidDigit => if res.error.index == 0 {
-      Err(Err::Error(E::from_error_kind(input, ErrorKind::Float)))
-    } else {
-      Ok((input.slice(res.error.index..), res.value))
-    },
-    _ => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
+  match ::lexical_core::parse_partial::<f32>(input.as_bytes()) {
+    Ok((v, i)) => Ok((input.slice(i..), v)),
+    Err(_) => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
   }
 }

@@ -906,16 +899,9 @@ pub fn double<T, E:ParseError<T>>(input: T) -> IResult<T, f64, E>
 where
   T: crate::traits::AsBytes + InputLength + Slice<RangeFrom<usize>>,
 {
-  let res = ::lexical_core::try_atof64_slice(input.as_bytes());
-
-  match res.error.code {
-    ::lexical_core::ErrorCode::Success => Ok((input.slice(input.input_len()..), res.value)),
-    ::lexical_core::ErrorCode::InvalidDigit => if res.error.index == 0 {
-      Err(Err::Error(E::from_error_kind(input, ErrorKind::Float)))
-    } else {
-      Ok((input.slice(res.error.index..), res.value))
-    },
-    _ => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
+  match ::lexical_core::parse_partial::<f64>(input.as_bytes()) {
+    Ok((v, i)) => Ok((input.slice(i..), v)),
+    Err(_) => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
   }
 }

diff --git a/src/number/streaming.rs b/src/number/streaming.rs
index 914578c..8e1c97c 100644
--- a/src/number/streaming.rs
+++ b/src/number/streaming.rs
@@ -829,18 +829,15 @@ pub fn float<T, E:ParseError<T>>(input: T) -> IResult<T, f32, E>
 where
   T: crate::traits::AsBytes + InputLength + Slice<RangeFrom<usize>>,
 {
-  let res = ::lexical_core::try_atof32_slice(input.as_bytes());
-
-  match res.error.code {
-    ::lexical_core::ErrorCode::Success => Err(Err::Incomplete(Needed::Unknown)),
-    ::lexical_core::ErrorCode::InvalidDigit => {
-      if res.error.index == 0 {
-        Err(Err::Error(E::from_error_kind(input, ErrorKind::Float)))
+  match ::lexical_core::parse_partial::<f32>(input.as_bytes()) {
+    Ok((v, i)) => {
+      if i == input.input_len() {
+        Err(Err::Incomplete(Needed::Unknown))
       } else {
-        Ok((input.slice(res.error.index..), res.value))
+        Ok((input.slice(i..), v))
       }
     },
-    _ => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
+    Err(_) => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
   }
 }

@@ -905,18 +902,15 @@ pub fn double<T, E:ParseError<T>>(input: T) -> IResult<T, f64, E>
 where
   T: crate::traits::AsBytes + InputLength + Slice<RangeFrom<usize>>,
 {
-  let res = ::lexical_core::try_atof64_slice(input.as_bytes());
-
-  match res.error.code {
-    ::lexical_core::ErrorCode::Success => Err(Err::Incomplete(Needed::Unknown)),
-    ::lexical_core::ErrorCode::InvalidDigit => {
-      if res.error.index == 0 {
-        Err(Err::Error(E::from_error_kind(input, ErrorKind::Float)))
+  match ::lexical_core::parse_partial::<f64>(input.as_bytes()) {
+    Ok((v, i)) => {
+      if i == input.input_len() {
+        Err(Err::Incomplete(Needed::Unknown))
       } else {
-        Ok((input.slice(res.error.index..), res.value))
+        Ok((input.slice(i..), v))
       }
-    }
-    _ => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
+    },
+    Err(_) => Err(Err::Error(E::from_error_kind(input, ErrorKind::Float))),
   }
 }
