
# ========
# captured on: Sun Oct 30 21:16:37 2016
# hostname : kron
# os release : 4.4.0-45-generic
# perf version : 4.4.21
# arch : x86_64
# nrcpus online : 8
# nrcpus avail : 8
# cpudesc : Intel(R) Core(TM) i7-6700K CPU @ 4.00GHz
# cpuid : GenuineIntel,6,94,3
# total memory : 32863380 kB
# cmdline : /usr/lib/linux-tools-4.4.0-45/perf record rustc test.rs 
# event : name = cycles:pp, , size = 112, { sample_period, sample_freq } = 4000, sample_type = IP|TID|TIME|PERIOD, disabled = 1, inherit = 1, mmap = 1, comm = 1, freq = 1, enable_on_exec = 1, task = 1, precise_ip = 2, sample_id_all = 1, exclude_guest = 1, mmap2 = 1, comm_exe
# HEADER_CPU_TOPOLOGY info available, use -I to display
# HEADER_NUMA_TOPOLOGY info available, use -I to display
# pmu mappings: intel_pt = 7, intel_bts = 6, cpu = 4, msr = 8, software = 1, tracepoint = 2, cstate_core = 9, cstate_pkg = 10, breakpoint = 5
# ========
#
#
# Total Lost Samples: 0
#
# Samples: 149  of event 'cycles:pp'
# Event count (approx.): 159792095
#
# Overhead  Command  Shared Object                  Symbol                                                                                         
# ........  .......  .............................  ...............................................................................................................................................................................................................................
#
    10.85%  rustc    ld-2.23.so                     [.] do_lookup_x                                                                                                                                                                                                                
     7.68%  rustc    librustc_metadata-6eb85298.so  [.] rustc_metadata::decoder::_$LT$impl$u20$rustc_metadata..cstore..MetadataBlob$GT$::load_key_map::hf181e213f9856db2                                                                                                           
     6.81%  rustc    ld-2.23.so                     [.] strcmp                                                                                                                                                                                                                     
     6.54%  rustc    ld-2.23.so                     [.] check_match                                                                                                                                                                                                                
     3.45%  rustc    librustc_metadata-6eb85298.so  [.] _$LT$rustc_metadata..schema..Entry$LT$$u27$tcx$GT$$u20$as$u20$serialize..serialize..Decodable$GT$::decode::h6190a6fb1f040282                                                                                               
     3.35%  rustc    [unknown]                      [k] 0xffffffff813feaf7                                                                                                                                                                                                         
     3.04%  rustc    libstd-6eb85298.so             [.] mallocx                                                                                                                                                                                                                    
     3.03%  rustc    ld-2.23.so                     [.] _dl_check_map_versions                                                                                                                                                                                                     
     2.84%  rustc    libsyntax-6eb85298.so          [.] _$LT$std..collections..hash..map..HashMap$LT$K$C$$u20$V$C$$u20$S$GT$$GT$::get::h92e875ddcaeef5eb                                                                                                                           
     2.74%  rustc    ld-2.23.so                     [.] _dl_relocate_object                                                                                                                                                                                                        
     2.73%  rustc    [unknown]                      [k] 0xffffffff811cbca7    
