
# ========
# captured on: Sun Oct 30 21:00:17 2016
# hostname : ubuntu
# os release : 4.4.0-45-generic
# perf version : 4.4.21
# arch : x86_64
# nrcpus online : 8
# nrcpus avail : 8
# cpudesc : Intel(R) Core(TM) i7-6700K CPU @ 4.00GHz
# cpuid : GenuineIntel,6,94,3
# total memory : 16415168 kB
# cmdline : /usr/lib/linux-tools-4.4.0-45/perf record rustc test.rs 
# event : name = cycles, , size = 112, { sample_period, sample_freq } = 4000, sample_type = IP|TID|TIME|PERIOD, disabled = 1, inherit = 1, mmap = 1, comm = 1, freq = 1, enable_on_exec = 1, task = 1, sample_id_all = 1, exclude_guest = 1, mmap2 = 1, comm_exec = 1
# HEADER_CPU_TOPOLOGY info available, use -I to display
# HEADER_NUMA_TOPOLOGY info available, use -I to display
# pmu mappings: cpu = 4, msr = 6, software = 1, tracepoint = 2, breakpoint = 5
# ========
#
#
# Total Lost Samples: 0
#
# Samples: 53K of event 'cycles'
# Event count (approx.): 53214853068
#
# Overhead  Command  Shared Object                  Symbol                                                                                         
# ........  .......  .............................  ...............................................................................................................................................................................................................................
#
    96.65%  rustc    libstd-6eb85298.so             [.] backtrace_alloc                                                                                                                                                                                                            
     1.06%  rustc    libstd-6eb85298.so             [.] swap                                                                                                                                                                                                                       
     0.23%  rustc    libstd-6eb85298.so             [.] backtrace_qsort                                                                                                                                                                                                            
     0.11%  rustc    libstd-6eb85298.so             [.] read_function_entry                                                                                                                                                                                                        
     0.11%  rustc    libstd-6eb85298.so             [.] line_compare                                                                                                                                                                                                               
     0.10%  rustc    libstd-6eb85298.so             [.] read_uleb128                                                                                                                                                                                                               
     0.10%  rustc    libstd-6eb85298.so             [.] elf_symbol_compare                                                                                                                                                                                                         
     0.09%  rustc    rustc                          [.] swap                                                                                                                                                                                                                       
     0.08%  rustc    [unknown]                      [k] 0xffffffff813feb49                                                                                                                                                                                                         
     0.08%  rustc    libstd-6eb85298.so             [.] read_line_program                                                                                                                                                                                                          
     0.07%  rustc    libstd-6eb85298.so             [.] advance             
