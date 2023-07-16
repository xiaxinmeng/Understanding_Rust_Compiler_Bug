
digraph finite_state_machine {
	rankdir=LR;
	size="8,5"
        "&[u8]" -> String [label = "String::from_lossy()"]
        "&str" -> "&OsStr" [ label = "OsStr::new()"]
        "OsString" -> String [ label = "into_string()"]
        "&OsStr" -> OsString [ label = "to_os_string()" ]
        "&str" -> OsString [ label = "from()"]
	String -> "&str" [ label = "&" ];
        "&str" -> String [ label = "to_owned(), to_string()"]
        "&Path" -> String [ label = "to_string()"]
        "&Path" -> "&OsStr" [ label = "as_os_str()" ]
        PathBuf -> String [label = "to_string()"]
        PathBuf -> "&Path" [label = "&" ]
        PathBuf -> OsString [label = "into_os_string()"]
        "&OsStr" -> "&Path" [label = "Path::new()"]
       OsString -> "&OsStr" 
}
