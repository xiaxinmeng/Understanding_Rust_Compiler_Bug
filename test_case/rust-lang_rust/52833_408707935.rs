rust
async fn connect_to_known_network<'a>(                                                               
    sme: &'a fidl_sme::ClientSmeProxy, ssid: &'a [u8], known_ess: &'a KnownEss,                      
) -> Result<bool, failure::Error> {                                                                  
    let ssid_str = String::from_utf8_lossy(ssid).into_owned();                                       
    println!("wlancfg: Auto-connecting to '{}'", ssid_str);                                          
    let connect_txn = start_connect_txn(sme, &ssid, &known_ess.password)?;                           
    let r = await!(wait_until_connected(connect_txn))?;                                              
    match r {                                                                                        
        fidl_sme::ConnectResultCode::Success => {                                                    
            println!("wlancfg: Auto-connected to '{}'", ssid_str);                                   
            Ok(true)                                                                                 
        },                                                                                           
        other => {                                                                                   
            println!("wlancfg: Failed to auto-connect to '{}': {:?}", ssid_str, other);              
            Ok(false)                                                                                
        },                                                                                           
    }                                                                                                
} 
