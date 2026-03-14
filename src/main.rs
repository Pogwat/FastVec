use testing::shared::FastVec;
use testing::shared::Errors;
use testing::shared::ValueMapKeyVec;

//absoulute::Errors
use testing::absolute::KeyVec;
use testing::absolute::AbsoluteKeys;

use testing::swap_refs;
use core::ptr;



fn main() -> Result<(),Errors>{
    let mut fv: FastVec<String> = FastVec::new();   
    //fv.insert(10,"hi".to_string());
     println!("len:{}", fv.len_of_vec()); //len:0
     fv.push_by_value("hi".to_string());
     println!("hi is at: {}", fv.get_by_value(&"hi".to_string())?); //hi is at: 0
     println!("0 is :{}", fv.get_by_key(0)?); //0 is :hi
    let mut a = vec![1,5,6];
    println!("vec before swap: {:?}",a);
    swap_refs!(&mut a[0], &mut a[2]);
    println!("vec after swap: {:?}",a);
    absoulute();

    // let (old_v,_) = fv.mod_to(0, "3".to_string())?;
    // println!("{}",old_v);
    // fv.push_by_value("hii".to_string());
    // println!("hii is at: {}", fv.get_by_value(&"hii".to_string())?); //hii is at: 1
    
    // for item in fv.iter() {
    //     println!("{}",item); //hi 6767
    // }
    // println!("one is: {}", fv[1]); //one is: 6767
    // println!("elements: {}", fv); //elements: ["hi", "6767"]
    // println!("full: {:?}", fv); //full: FastVec { vector: ["hi", "6767"], map: {"6767": 1, "hi": 0} }
    // println!("len is {}", fv.len());
    // println!("capacity is {}",fv.capacity());
    // fv.reserve(12);
    // println!("new capacity is {}",fv.capacity());
    Ok(())
}

fn absoulute() -> Result<(), testing::absolute::AbsoluteErrors> {
    let mut keys = KeyVec::new();
    let mut values = vec![10,20,30,40,50];
    values.iter().enumerate().for_each(|(k,v)|  {
        keys.push_fake(k) ;
        keys.push_reals_fake(k) ;
    });
    println!("{:?}", values);
    println!("{:?}", keys);
    println!("fake3 value:{}, fake2_real_key:{}", values[keys.get_fake(4)?.unwrap()], keys.get_fake(4)?.unwrap());
    values.swap_remove(keys.get_fake(2)?.unwrap());
    keys.remove_from_real_fake_by_fake(2);

    println!("after swap: {:?}", values);
    println!("after swap:  {:?}", keys);
    println!("fake3 value:{}, fake2_real_key:{}", values[keys.get_fake(4)?.unwrap()], keys.get_fake(4)?.unwrap());
    //The values in real that fake refrences is the same
    Ok(())
}

