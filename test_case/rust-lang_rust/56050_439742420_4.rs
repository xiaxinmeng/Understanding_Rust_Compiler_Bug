rust

let ip: Ipv6Addr = "ff01::".parse().unwrap();         
assert!(!ip.is_unspecified());                        
assert!(!ip.is_loopback());                           
assert!(!ip.is_unique_local());                       
assert!(!ip.is_global());                             
assert!(!ip.is_unicast_link_local());                 
assert!(!ip.is_unicast_link_local_strict());          
assert!(!ip.is_unicast_global());                     
assert!(!ip.is_documentation());                      
assert!(ip.is_multicast());                           
assert_eq!(ip.multicast_scope(), Some(InterfaceLocal))
