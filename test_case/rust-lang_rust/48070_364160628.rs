
% rustc issue-48070.rs                          # sanity check
% rustc issue-48070.rs -Z borrowck=mir          # mir-borrowck (lexically via EndRegion)
% rustc issue-48070.rs -Z borrowck=mir -Z nll   # okay, then it must be NLL...?
% 
