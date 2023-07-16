diff
- (None, Some(val)) | (Some(val), None) => source + val,
+ (None, Some(&val)) | (Some(&val), None) => source + val,
