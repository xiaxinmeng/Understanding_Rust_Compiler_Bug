
<anon>:16:17: 17:49 error: borrowed value does not live long enough
<anon>:16                 serialize::json::encode(&JsonSerialized {
<anon>:17                     really_long_field_name: 0,}).as_slice().clone()
<anon>:14:40: 27:10 note: reference must be valid for the anonymous lifetime #1 defined on the block at 14:39...
<anon>:14         fn get_json_str(&self) -> &str {
<anon>:15             if ! ALT_BEHAVIOUR {
<anon>:16                 serialize::json::encode(&JsonSerialized {
<anon>:17                     really_long_field_name: 0,}).as_slice().clone()
<anon>:18             } else {
<anon>:19                 let s0 = serialize::json::encode(&JsonSerialized { really_long_field_name: 0, });
          ...
<anon>:15:32: 18:14 note: ...but borrowed value is only valid for the block at 15:31
<anon>:15             if ! ALT_BEHAVIOUR {
<anon>:16                 serialize::json::encode(&JsonSerialized {
<anon>:17                     really_long_field_name: 0,}).as_slice().clone()
<anon>:18             } else {
<anon>:21:26: 21:28 error: `s0` does not live long enough
<anon>:21                 let s1 = s0.as_slice();
                                   ^~
<anon>:14:40: 27:10 note: reference must be valid for the anonymous lifetime #1 defined on the block at 14:39...
<anon>:14         fn get_json_str(&self) -> &str {
<anon>:15             if ! ALT_BEHAVIOUR {
<anon>:16                 serialize::json::encode(&JsonSerialized {
<anon>:17                     really_long_field_name: 0,}).as_slice().clone()
<anon>:18             } else {
<anon>:19                 let s0 = serialize::json::encode(&JsonSerialized { really_long_field_name: 0, });
          ...
<anon>:18:20: 26:14 note: ...but borrowed value is only valid for the block at 18:19
<anon>:18             } else {
<anon>:19                 let s0 = serialize::json::encode(&JsonSerialized { really_long_field_name: 0, });
<anon>:20                 println!("s0: `{}`", s0);
<anon>:21                 let s1 = s0.as_slice();
<anon>:22                 println!("s1: `{}`", s1);
<anon>:23                 let s2 = s1.clone();
          ...
error: aborting due to 2 previous errors
playpen: application terminated with error code 101
