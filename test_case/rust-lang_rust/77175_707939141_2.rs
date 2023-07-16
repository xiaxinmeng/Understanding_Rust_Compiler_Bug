
2:rustcDEBUG rustc_resolve::late::lifetimes lifetime_uses: {
	DefId(0:12 ~ issue_77175[317d]::bar::'_): Many, 
	DefId(0:4 ~ issue_77175[317d]::bar::'a): Many, 
	DefId(0:11 ~ issue_77175[317d]::bar::{opaque#0}::'_): Many, 
	DefId(0:10 ~ issue_77175[317d]::bar::{opaque#0}::'a): One(lifetime(HirId { owner: DefId(0:5 ~ issue_77175[317d]::bar::{opaque#0}), local_id: 1 }: 'a))
}
