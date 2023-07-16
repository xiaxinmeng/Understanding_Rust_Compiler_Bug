
return; 
let num_items = 100;
let mut split_by = 50;
let mut start_at = 0;
let mut count = 0;  
let take = if num_items <= split_by { 
              split_by = 1;               
              num_items //Take all items at once in this case
            } else {                    
                num_items / split_by    // Take x number at a time                      
            };  
