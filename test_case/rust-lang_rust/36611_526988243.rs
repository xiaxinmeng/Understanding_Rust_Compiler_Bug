rust
use std::collections::HashMap;

fn main() {
    let vec:Vec<i32> = vec![2,2,1];
    single_number(vec);
}

fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = HashMap::new();

    for v in vec! {
       // 类型转换
       let str_key = v.to_string();
       //插入之前先判断,当前的key对应的value有值则加1，没值初始化为1
       if result.get(&str_key) == None {
        result.insert(str_key,1);
       }else{
            let str_key1 = v.to_string();
            let _strValue = result.get(&str_key1);
        
            let value = _strValue.unwrap();
            result.insert(str_key,value + 1);

       }
    }
   
   
   // 遍历result
   
    let mut resultK = 0;
    for (k,v) in &result {
        println!("{},{}",k,v);
        let value = *v as i32;
        if value == 1
        {
          resultK = k.parse::<i32>().unwrap();;
        }
    }
    resultK
}
