

extern crate time;
extern crate test;


extern crate serialize;
use serialize::json;


use std::rand::Rng;
use std::rand;
use test::Bencher;
use std::collections::{TreeMap, HashMap};

pub struct TestStruct1  {
    data_int1: int,
    data_int2: int,
    data_int3: int,
    data_str1: String,
    data_str2: String,
    data_str3: String,
    data_map: Option<Vec<Box<TestStruct1>>>,
    data_vector: Vec<u8>,
    data_vector_s: Vec<String>,
}

#[bench]
fn bench_decode(b: &mut Bencher) {
    let mut object = TestStruct1
         {data_int1: -999, data_int2: 999, data_int3: 9999, data_str1:"toto".to_string(), data_str2:"toto".to_string(), data_str3:"toto".to_string(),
         data_vector:vec![2,3,4,5], data_vector_s:vec!["hi".to_string(), "mom".to_string()], data_map:None};

    object = TestStruct1
         {data_int1: -999, data_int2: 999, data_int3: 9999,  data_str1:"toto".to_string(), data_str2:"toto".to_string(), data_str3:"toto".to_string(),
         data_vector:vec![2,3,4,5], data_vector_s:vec!["hi".to_string(), "mom".to_string()], data_map:Some(vec![box object])};

    object = TestStruct1
         {data_int1: -999, data_int2: 999, data_int3: 9999, data_str1:"toto".to_string(), data_str2:"toto".to_string(), data_str3:"toto".to_string(),
         data_vector:vec![2,3,4,5], data_vector_s:vec!["hi".to_string(), "mom".to_string()], data_map:Some(vec![box object])};

     // Serialize using `json::encode`
    let encoded = json::encode(&object);

    b.iter( || {
        let _ = json::from_str(encoded.as_slice());
    });
}


#[bench]
fn bench_hashmap_insert(b: &mut Bencher) {

    let v0: Vec<int> = rand::task_rng().gen_iter::<int>().take(9).collect();

    let mut v: Vec<String> = Vec::new();

    for i in v0.iter() {
        v.push("test_".to_string() + i.to_string());
    }

    b.iter(|| {
        let mut m = HashMap::new();

        for i_s in v.iter() {
            m.insert(i_s.clone(), i_s);
        }
    });
}



#[bench]
fn bench_treemap_insert(b: &mut Bencher) {


    let v0: Vec<int> = rand::task_rng().gen_iter::<int>().take(9).collect();

    let mut v: Vec<String> = Vec::new();

    for i in v0.iter() {
        v.push("test_".to_string() +  i.to_string());
    }

    b.iter(|| {
        let mut m = TreeMap::new();

        for i_s in v.iter() {
            m.insert(i_s.clone(), i_s);
        }
    });
}


