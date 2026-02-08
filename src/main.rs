use std::os::unix::net::UnixStream;
use std::env::var;
use std::io::{BufRead, BufReader};

use hashbrown::HashMap;
use slab::Slab; //stable indicies
mod shared;
use shared::FastVec;

use std::collections::BTreeMap;


fn main() -> Result<(),std::io::Error> {
    let mut fv: FastVec<String> = FastVec::new();   
    //fv.insert(10,"hi".to_string());
    println!("len:{}", fv.vector.len());
    fv.push("hi".to_string());
    println!("hi is at: {}", fv.get_by_value(&"hi".to_string()).unwrap());
    println!("0 is :{}", fv.get_by_key(0).unwrap());

    fv.insert(1, "hii".to_string()).unwrap();
    println!("hii is at: {}", fv.get_by_value(&"hii".to_string()).unwrap());
    
    println!("old is {}, new is {}", fv.mod_by_key(1, "6767".to_string()).unwrap(), fv.get_by_key(1).unwrap());
    //let prr= fv.get_by_key(10).unwrap();
    //println!("{}", prr);
    std::process::exit(0);
    //let vector: Vec<String> = Vec::new();
        //let map = HashMap::new();
        //let btreemap: BTreeMap<u8,Vec<usize>> = BTreeMap::new();
   
    //let fv = FastVec {vector,map,
    //btreemap   
//};
    
    println!("hello");
    // Slabs | id, String | Entry n
    // Hashmap| id, Entry -> Slab1.entry(n) & Slab2.entry(n)
    let mut slab_works: Slab<u8> = Slab::new(); //workspace
    let mut slab_ids: Slab<String> = Slab::new(); //winid
    let mut slab_content: Slab<(String,String)> = Slab::new(); //(initialtitle,initialclass)
    let mut map: HashMap<String,(u8,u8,u8)> = HashMap::new();  //workspace, vector index, slab index

    //for id sorting
    let mut ordered_map: BTreeMap<u8,Vec<String>> = BTreeMap::new(); //workdspace and winids

let sock: String =         format!(
        "{}/hypr/{}/.socket2.sock",
        var("XDG_RUNTIME_DIR").unwrap(),
        var("HYPRLAND_INSTANCE_SIGNATURE").unwrap()
    );
    let stream = UnixStream::connect(sock).unwrap();
    let reader = BufReader::new(stream);

        for line in reader.lines() {
            let line = line?; // Result<String, std::io::Error>
     
            if let Some((prefix, value)) = line.split_once(">>") {

                match prefix {


                    "openwindow" => { // openwindow>>55c018ac1aa0,3,kitty,kitty           
                        let parts: Vec<&str> = value.split(',').collect();
                        let [id, workspace, initialclass, initialtitle]: [&str; 4] = parts.try_into().expect("not 4 arguments in openwindow");            
                        let workspace: u8 = workspace.parse().expect("workspace in openwindow is not u8");                    
                        let (id,initialclass, initialtitle): (String,String,String) = (id.to_string(),initialclass.to_string(),initialtitle.to_string());
                        let slab_index = slab_works.insert(workspace.clone());
                        slab_ids.insert(id.clone());
                        slab_content.insert((initialtitle,initialclass));
                        let vec = ordered_map.entry(workspace).or_insert_with(Vec::new);
                        let vec_index = vec.len() as u8; 
                        vec.push(id.clone());  
                        map.insert(id, (workspace, vec_index.clone(),slab_index as u8));
                        println!("created:{}",vec_index);
                        println!("{}",vec.len());
                        for (k,v) in slab_ids.iter(){
                            println!("({},{}) ",k,v);
                        }

                        //data.format();
                    }

                    "closewindow" => { // closewindow>>55c018ac1aa0    
                        let id:&str = value;
                        let (workspace,r_vec_index,slab_index) = map.remove(id).unwrap();
                        slab_works.remove(slab_index as usize);
                        slab_ids.remove(slab_index as usize);
                        slab_content.remove(slab_index as usize);
                        
                        let vec = ordered_map.get_mut(&workspace).unwrap();

                        if vec.len() as u8 - 1 != r_vec_index {                        
                        vec.swap_remove(r_vec_index as usize);
                        let lasts_id = vec.get(r_vec_index as usize).unwrap();
                        let (workspace, vec_index, slab_index) = map.get_mut(lasts_id).unwrap();
                        *vec_index = r_vec_index;                                                                        
                        }else{vec.pop();};
                        
                        println!("removed:{}",r_vec_index);
                        println!("{}",vec.len());
                        for (k,v) in slab_ids.iter(){
                            println!("({},{}) ",k,v);
                        }


                        //data.format();    
                    }
                
                   _ => {}
                
                }





            }

        }
    


Ok(()) 
}

