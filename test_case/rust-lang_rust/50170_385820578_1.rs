diff
  impl<'a, T> From<Cow<'a, [T]>> for Vec<T>
  impl<'a, T> From<&'a [T]> for Cow<'a, [T]> 
  impl<'a, T> From<Vec<T>> for Cow<'a, [T]> 
+ impl<'a, T> From<&'a Vec<T>> for Cow<'a, [T]>

  impl<'a> From<Cow<'a, str>> for String
  impl<'a> From<&'a str> for Cow<'a, str>
  impl<'a> From<String> for Cow<'a, str>
+ impl<'a> From<&'a String> for Cow<'a, str>

+ impl<'a> From<Cow<'a, Path>> for PathBuf
  impl<'a> From<&'a Path> for Cow<'a, Path>
  impl<'a> From<PathBuf> for Cow<'a, Path>
+ impl<'a> From<&'a PathBuf> for Cow<'a, Path>

+ impl<'a> From<Cow<'a, CStr>> for CString
+ impl<'a> From<&'a CStr> for Cow<'a, CStr>
+ impl<'a> From<CString> for Cow<'a, CStr>
+ impl<'a> From<&'a CString> for Cow<'a, CStr>

+ impl<'a> From<Cow<'a, OsStr>> for OsString
+ impl<'a> From<&'a OsStr> for Cow<'a, OsStr>
+ impl<'a> From<OsString> for Cow<'a, OsStr>
+ impl<'a> From<&'a OsString> for Cow<'a, OsStr>
