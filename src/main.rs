mod shared;
use shared::FastVec;


fn main() -> Result<(),std::io::Error> {
    let mut fv: FastVec<String> = FastVec::new();   
    //fv.insert(10,"hi".to_string());
    println!("len:{}", fv.vector.len()); //len:0

    fv.push("hi".to_string());
    println!("hi is at: {}", fv.get_by_value(&"hi".to_string()).unwrap()); //hi is at: 0
    println!("0 is :{}", fv.get_by_key(0).unwrap()); //0 is :hi

    fv.insert(1, "hii".to_string()).unwrap();
    println!("hii is at: {}", fv.get_by_value(&"hii".to_string()).unwrap()); //hii is at: 1
    
    println!("old is {}, new is {}", fv.mod_by_key(1, "6767".to_string()).unwrap(), fv.get_by_key(1).unwrap()); //old is hii, new is 6767
    
    for item in fv.iter() {
        println!("{}",item); //hi 6767
    }
    println!("one is: {}", fv[1]); //one is: 6767
    println!("elements: {}", fv); //elements: ["hi", "6767"]
    println!("full: {:?}", fv); //full: FastVec { vector: ["hi", "6767"], map: {"6767": 1, "hi": 0} }

Ok(()) 
}

