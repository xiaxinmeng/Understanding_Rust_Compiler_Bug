rust
// gdbr-command:interpreter-exec mi2 "-enable-pretty-printing"
// gdbr-check:^done
// gdbr-command:interpreter-exec mi2 "-var-create noPadding8 @ noPadding8"
// gdbr-check:^done,name="noPadding8",numchild="2",value="{...}",type="(i8, u8)",thread-id="1",has_more="0"
// gdbr-command:interpreter-exec mi2 "-var-list-children noPadding8"
// gdbr-check:^done,numchild="2",displayhint="array",children=[child={name="noPadding8.0",exp="0",numchild="0",value="-100",type="i8",thread-id="1"},child={name="noPadding8.1",exp="1",numchild="0",value="100",type="u8",thread-id="1"}],has_more="0"
