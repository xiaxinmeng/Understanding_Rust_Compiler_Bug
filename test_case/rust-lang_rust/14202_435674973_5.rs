
$ rustc ./main.rs -Z unpretty=flowgraph=114 -Z flowgraph-print-loans -Z flowgraph-print-moves -Z flowgraph-print-assigns -Z flowgraph-print-all -o gv.dot
$ xdot gv.dot
