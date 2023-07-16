
captured place: CapturedPlace {
    place: Place {
        base_ty: Foo,
        base: Upvar(
            UpvarId(HirId { owner: DefId(0:15 ~ issue_69446_fnmut_capture[4b65]::main), local_id: 5 };`x`;DefId(0:16 ~ issue_69446_fnmut_capture[4b65]::main::{closure#0})),
        ),
        projections: [],
    },
    info: CaptureInfo {
        capture_kind_expr_id: Some(
            HirId {
                owner: DefId(0:15 ~ issue_69446_fnmut_capture[4b65]::main),
                local_id: 13,
            },
        ),
        path_expr_id: Some(
            HirId {
                owner: DefId(0:15 ~ issue_69446_fnmut_capture[4b65]::main),
                local_id: 13,
            },
        ),
        capture_kind: ByValue,
    },
    mutability: Mut,
    region: None,
}
upvar definition span: /Users/user/repos/rust/src/test/ui/async-await/issue-69446-fnmut-capture.rs:18:9: 18:14 (#0)
upvars_mentioned map: {
    HirId {
        owner: DefId(0:15 ~ issue_69446_fnmut_capture[4b65]::main),
        local_id: 5,
    }: Upvar {
        span: /Users/user/repos/rust/src/test/ui/async-await/issue-69446-fnmut-capture.rs:20:9: 20:10 (#0),
    },
}
