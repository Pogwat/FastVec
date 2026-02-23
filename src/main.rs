use testing::shared::FastVec;
use testing::shared::Errors;
use testing::shared::ValueMapKeyVec;
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

