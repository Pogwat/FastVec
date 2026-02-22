//HELPER FUNCTIONS
    
    //CHECKS

    fn key_bounds<T>(vec:&Vec<T>, key:usize) -> Ordering{ //probably dont need a fucntion for something this simple
        let last_index = vec.len()-1;
        key.cmp(&last_index)
    }

    //GETS

    fn get_by_key<V:Insertable>(vec: &Vec<V>, key:usize) -> Result<V,Errors>{
        Ok(vec.get(key).ok_or(Errors::KeyOutOfBounds)?.clone())
    }

    fn get_by_value<V:Insertable>(map: &HashMap<V,usize>, value:&V) -> Result<usize,Errors> {
        Ok(map.get(value).ok_or(Errors::ValueOutOfBounds)?.clone())
    }

    //INSERTS

    //modify key's value and returns old value, consumes newvalue
    fn vec_mod_key<V:Insertable>(vec: &mut Vec<V>,key:usize, newvalue:V) -> Result<V,Errors>{ 
        let last_index = vec.len()-1;
        if key<=last_index {
            let old_value = mem::replace(&mut vec[key], newvalue);
            Ok(old_value)
        }else {return Err(Errors::KeyOutOfBounds)}

    }
    
    //change value, return old 
    fn fastvec_mod_by_value<V:Insertable>( vec: &mut Vec<V>,map: &mut HashMap<V,usize>,value:&V,newval:V) -> Result<V,Errors> {
        let key = map.remove(value).ok_or(Errors::ValueOutOfBounds)?;
        map.insert(newval.clone(),key);
        vec_mod_key(vec,key,newval)
        }

    //change value, return old
    fn fastvec_mod_by_key<V:Insertable>( vec: &mut Vec<V>,map: &mut HashMap<V,usize>,key:usize,newval:V) -> Result<V,Errors> {
        let old_value = vec_mod_key(vec,key,newval.clone())?;
        map.remove(&old_value);
        map.insert(newval,key);
        Ok(old_value)
    }
        

    fn push_vec<V:Insertable>(vec: &mut Vec<V>, value:V) -> usize { //push to a vec
        let index = vec.len(); // = last-1. vec index starts at 0, but length starts at 1. so len-1 = index. len = next element index
        vec.push(value);
        index
    }
    
    fn fastvec_push<V: Insertable>( vec: &mut Vec<V>, map: &mut HashMap<V, usize>, value: V) -> usize{ 
        let index = push_vec(vec, value.clone());
        map.insert(value, index); //returns lastkey Option<usize> //SHould return None if no duplicate entrys, else Some(usize) 
        index
    }
    

    fn fastvec_insert<V: Insertable>(vec: &mut Vec<V>, map: &mut HashMap<V, usize>, key: usize, value: V) -> Result<(),Errors> {
        if key<=vec.len(){
        vec.insert(key,value.clone());  
        map.insert(value,key);
        Ok(())
        } else {return Err(Errors::KeyOutOfBounds)}
    }


    //REMOVES

    fn  swap_remove_by_key_old_new<V:Insertable>(vec: & mut Vec<V>, key:usize) -> Result<(V,Option<V>),Errors>{ //(Old,New),Error
            let last_index:usize = vec.len()-1;
            match key.cmp(&last_index) {
                Ordering::Less => {
                    let old_value = vec.swap_remove(key);
                    let new_value = vec.get(key).ok_or(Errors::KeyOutOfBounds)?.clone();
                    return Ok((old_value, Some(new_value)))
                }
                Ordering::Equal => {
                    let old_value = vec.pop().ok_or(Errors::KeyOutOfBounds)?; //Last
                    return Ok((old_value,None))
                }
                Ordering::Greater => {
                    return Err(Errors::KeyOutOfBounds)
                }
            }
    }

    fn fastvec_swap_remove_key<V:Insertable>( vec: &mut Vec<V>,  map: &mut HashMap<V, usize>, key:usize) -> Result<(V,Option<V>),Errors> { //removed_value 
        let (removed_value, new_value) = swap_remove_by_key_old_new(vec,key)?;
        map.remove(&removed_value);
        if let Some(value) = new_value.clone() { //A None value would occur if key=last index, this is valid and shouldnt return error, just remove from the map
            map.insert(value, key);
        }
        // else {
        //     return Err(Errors::ValueOutOfBounds)
        // };

        Ok((removed_value, new_value))
    }

    fn fastvec_absolute_remove_by_key<V:Insertable>( vec: &mut Vec<V>,  map: &mut HashMap<V, usize>, refvec:&mut Vec<Option<usize>>, key:usize) -> Result<(V,Option<V>),Errors>{
        let (removed_value,new_value) = fastvec_swap_remove_key(vec,map,key)?;
        let last_index= vec.len()-1;
        refvec[last_index] = None;
        if new_value.is_some() {
        refvec[key] = Some(last_index);
        }
        Ok((removed_value,new_value))

    }

    fn fastvec_swap_remove_value<V:Insertable>( vec: &mut Vec<V>,  map: &mut HashMap<V, usize>, value:&V) -> Result<usize,Errors>{ //removed_key
        let key:usize = map.remove(value).ok_or(Errors::ValueOutOfBounds)?;
        if let (_, Some(new_value)) = swap_remove_by_key_old_new(vec,key)? {  //A None value would occur if key=last index, this is valid and shouldnt return error, just remove from the map
        map.insert(new_value, key);
        }
        // else {
        //     return Err(Errors::KeyOutOfBounds)
        // };
        Ok(key)
    }

    fn fastvec_remove_by_value<V: Insertable>( vec:&mut Vec<V>, map: &mut HashMap<V, usize>, value: &V) ->  Result<usize, Errors>{
        let key = map.remove(value).ok_or(Errors::ValueOutOfBounds)?; //Option<V>
        if let Ordering::Equal | Ordering::Less = key_bounds(vec,key) {
        vec.remove(key); 
        Ok(key)
        } else {return Err(Errors::KeyOutOfBounds)}
        
    }    

    fn fastvec_remove_by_key<V: Insertable>( vec:&mut Vec<V>, map: &mut HashMap<V, usize>, key: usize) -> Result<V,Errors> {
        if let Ordering::Equal | Ordering::Less = key_bounds(vec,key) {
            let value = vec.remove(key); //V
            map.remove(&value);
            Ok(value)
        } else {return Err(Errors::KeyOutOfBounds)}
    }

    //OLD IMPLS

    //IMPLS

    //FASTVEC IMPLS

    #[allow(dead_code)] //Backend Shared helpers for map vector
    impl<V:Hash + Eq + Clone> FastVec<V> {

    //CONSTRUCTORS
    
    pub     fn new() -> Self {
                Self {
                    vector: Vec::new(),
                    map: HashMap::new(),
                    #[cfg(feature = "FastRemove")]
                    refvec: Vec::new()
                }
            }

    pub     fn with_capacity(size: usize) -> Self {
                Self {
                    vector: Vec::with_capacity(size),
                    map: HashMap::with_capacity(size),
                    #[cfg(feature = "FastRemove")]
                    refvec: Vec::with_capacity(size)
                }
            } 
    
    //Wrappers

    pub const fn len(&self)-> usize {self.vector.len()}
    pub const fn capacity(&self) -> usize {self.vector.capacity()}
    pub fn reserve(&mut self, additional: usize){
        self.vector.reserve(additional);
        self.map.reserve(additional);}


    GETS

    pub     fn _get_by_key(&self, key:usize) -> Result<V,Errors> {
                get_by_key(&self.vector, key)
            }

    pub     fn _get_by_value(&self, value:&V) -> Result<usize, Errors> {
                get_by_value(&self.map, value)       
            }

    //INSERTS

    pub     fn mod_by_key(&mut self, key:usize, newvalue:V) -> Result<V,Errors> {
                fastvec_mod_by_key(&mut self.vector,&mut self.map,key, newvalue) 
            }

    pub     fn mod_by_value(&mut self,value:&V,newval:V) -> Result<V,Errors> {
                fastvec_mod_by_value(&mut self.vector,&mut self.map,value,newval) 
            }

    pub     fn push(&mut self, value:V) -> usize{
                fastvec_push(&mut self.vector, &mut self.map, value)
            }

    pub     fn insert(& mut self,key:usize, value:V) -> Result<(),Errors>  {
                fastvec_insert(&mut self.vector, &mut self.map, key, value) 
            }

    //REMOVES

    pub     fn _remove_by_key(&mut self, key:usize) -> Result<V,Errors> {
                fastvec_remove_by_key(&mut self.vector, &mut self.map, key)
            }

    pub     fn _remove_by_value(&mut self, value:&V) -> Result<usize, Errors> {
                fastvec_remove_by_value(&mut self.vector, &mut self.map, value)
            }

    pub     fn swap_remove_by_key(&mut self, key:usize) -> Result<(V,Option<V>),Errors> { //removed_value
                fastvec_swap_remove_key(& mut self.vector,  &mut self.map , key) 
            }

    pub     fn swap_remove_by_value(&mut self, value:&V) -> Result<usize,Errors> {  //removed_key
                fastvec_swap_remove_value(& mut self.vector,  &mut self.map , value)
            }

    }