use testing::shared::FastVec;
use testing::shared::Errors;


fn main() -> Result<(),Errors>{
    let mut fv: FastVec<String> = FastVec::new();   
    //fv.insert(10,"hi".to_string());
    println!("len:{}", fv.vector.len()); //len:0
    fv.push("hi".to_string());
    println!("hi is at: {}", fv.get_by_value(&"hi".to_string())?); //hi is at: 0
    println!("0 is :{}", fv.get_by_key(0)?); //0 is :hi

    fv.insert(1, "hii".to_string())?;
    println!("hii is at: {}", fv.get_by_value(&"hii".to_string())?); //hii is at: 1
    
    println!("old is {}, new is {}", fv.mod_by_key(1, "6767".to_string())?, fv.get_by_key(1)?); //old is hii, new is 6767
    
    for item in fv.iter() {
        println!("{}",item); //hi 6767
    }
    println!("one is: {}", fv[1]); //one is: 6767
    println!("elements: {}", fv); //elements: ["hi", "6767"]
    println!("full: {:?}", fv); //full: FastVec { vector: ["hi", "6767"], map: {"6767": 1, "hi": 0} }
    println!("len is {}", fv.len());
    println!("capacity is {}",fv.capacity());
    fv.reserve(12);
    println!("new capacity is {}",fv.capacity());
    Ok(())
}

