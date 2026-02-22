use hashbrown::HashMap;
use core::hash::Hash;
use core::fmt;
use core::cmp::Ordering;
use core::mem;
use core::ops::Index;
use core::ptr;

//Need derive for custom fields from a struct, So only sorrting by one value now
//using cfg logic with diffrent types and impl is nightmare, No struct<V,B> for me

//STRUCTS

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct FastVec<V> {
        pub   vector: Vec<V> ,//key, value
        pub   map: HashMap<V, usize>, //value, key
        #[cfg(feature = "FastRemove")]
        refvec: Vec< Option<usize> > //8 bytes overhead per element

    }
    //#[cfg(not(feature = "Sort"))] 


//ARGS AND TRAIT BOUNDS

pub trait Insertable: Clone + Hash + Eq {}

    impl<T: Clone + Hash + Eq> Insertable for T {}
    #[allow(dead_code)]
    #[derive(Default)] //Reusable with optionals
    struct Args<V: Insertable>{
        map: Option<HashMap<V, usize> > ,
        vector: Option<Vec<V> > ,
        #[cfg(feature = "FastRemove")] refvec: Option<Vec<Option<usize>>>, 
        key: Option<usize>,
        value: Option<V>,
    }

//all of this could just be wrappers around Vec trait impls

//ITER
    pub struct VIter<'a,V> {
        data: &'a [V],
        index: usize,
    }

    impl<'a,V> Iterator for VIter<'a,V> {
        type Item = &'a V;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let value = &self.data[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    impl <V:Hash + Eq + Clone + Ord> FastVec<V> {
        pub fn iter(&self) -> VIter<'_,V> {
            VIter { data: &self.vector, index: 0 }
        }
    }

//INDEX
    impl<V> Index<usize> for FastVec<V> {
        type Output = V;

        fn index(&self, index: usize) -> &Self::Output {
            &self.vector[index]
        }
    }

//FORMATING
    impl<V: fmt::Display  + fmt::Debug> fmt::Display for FastVec<V> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.vector.iter()).finish()
        }
}

//ERRORS
    #[derive(Debug)]
    pub enum Errors {
        KeyOutOfBounds,
        ValueOutOfBounds,
    }

    impl fmt::Display for Errors {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Errors::KeyOutOfBounds => write!(f, "Key is Out of Bounds"),
                Errors::ValueOutOfBounds => write!(f, "Value is Out of Bounds"),
            }
        }
    }

    trait ValueMapKeyVec<V:Insertable>{
        /*      NOTES 
        Keys are stored in a Map
        Values are stored in A Vec
        */
        //HashMap<value,key>
        //Vec<key,value>

        fn get_by_key(&self, key:usize) -> Result<V,Errors>;

        fn get_by_value(&self, value:V) -> Result<usize,Errors>;

        //remove_a_key(&mut self, key:usize) -> Result<V,Errors>

        fn remove_by_value(&mut self, value:V) -> Result<usize,Errors>;

        // fn mod_to_key(&mut self, key:usize) -> Result<&mut V,Errors>;

        // fn mod_to_value(&mut self, value:V) -> Result<&mut usize,Errors>;

        fn mod_to_keys(&mut self) -> Result<&mut Vec<V>,Errors>; //Cant have mutable refrence to self multiple times so one big refrence it is

        fn mod_to_values(&mut self) -> Result<&mut HashMap<V,usize>,Errors>;

        fn push_a_value_to_key(&mut self, value:V, key:usize) -> ();

        fn push_a_value(&mut self, value:V) -> ();

        fn pop_from_keys(&mut self) -> ();

        fn len_of_vec(&self) -> usize;



        //Using previous impl methods

        fn get_from_value(&self, value:V) -> Result<V,Errors> {
            let key = self.get_by_value(value)?;
            let vvalue = self.get_by_key(key)?;
            Ok(vvalue)  
        }

        // fn value_swap(&mut self, value1:V, value2:V) -> Result<(),Errors> { //Swaps values stored at hashmap keys
        //     let value1_mod = self.mod_to_value(value1)?;
        //     let value2_mod = self.mod_to_value(value2)?;
        //     mem::swap( value1_mod,  value2_mod); 
        //     Ok(())
        // }

        // fn key_swap(&mut self, key1:usize, key2:usize ) -> Result<(),Errors> { //Swaps values stored at vector keys
        //     let key1_mod = self.mod_to_key(key1)?;
        //     let key2_mod = self.mod_to_key(key2)?;
        //     mem::swap( key1_mod,  key2_mod);
        //     Ok(())
        // }

        fn swap_keys(&mut self, key1:usize, key2:usize) -> Result<(),Errors> {
            let key_mod = self.mod_to_keys()?;
            let key1_ptr = &raw mut key_mod[key1];
            let key2_ptr = &raw mut key_mod[key2];

            unsafe {
                ptr::swap(key1_ptr, key2_ptr);
            }
            Ok(())
        }

                fn swap_values(&mut self, key1:V, key2:V) -> Result<(),Errors> {
            let key_mod = self.mod_to_values()?;
            let key1_ptr = &raw mut *key_mod.get_mut(&key1).unwrap();   
            let key2_ptr = &raw mut *key_mod.get_mut(&key2).unwrap();    

            unsafe {
                ptr::swap(key1_ptr, key2_ptr);
            }
            Ok(())
        }

        fn last_index(&self) -> usize { //get last elemnt of vector
            self.len_of_vec()-1
        }

        fn key_swap_remove(&mut self, key1:usize ) -> Result<V,Errors> { //swaps key with last and pops for a vector
            let value = self.get_by_key(key1)?;
            let last_index = self.last_index();
            self.swap_keys(key1,last_index)?;
            self.pop_from_keys(); //should probably require a pop impl and use .pop() instead of .remove()
            Ok(value)
        }

        //remove_by_key() //UNSAFE

        fn swap_remove_from_value(&mut self, value:V) -> Result<V,Errors> { //remove from hashmap + swaprm on vec. by value
            let key = self.remove_by_value(value)?;
            let value = self.key_swap_remove(key)?;
            Ok(value)
        }

        fn swap_remove_from_key(&mut self, key:usize) -> Result<V,Errors> { //swap-rm key from vec, use value at key to remove from hashmap
            let value = self.key_swap_remove(key)?;
            self.remove_by_value(value.clone())?;
            Ok(value)

        }

        fn push_by_value(&mut self, value:V) -> () { 
            let last_index = self.last_index();
            self.push_a_value_to_key(value.clone(), last_index);
            self.push_a_value(value);
        }




    }

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


    //GETS

    pub     fn get_by_key(&self, key:usize) -> Result<V,Errors> {
                get_by_key(&self.vector, key)
            }

    pub     fn get_by_value(&self, value:&V) -> Result<usize, Errors> {
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

    pub     fn remove_by_key(&mut self, key:usize) -> Result<V,Errors> {
                fastvec_remove_by_key(&mut self.vector, &mut self.map, key)
            }

    pub     fn remove_by_value(&mut self, value:&V) -> Result<usize, Errors> {
                fastvec_remove_by_value(&mut self.vector, &mut self.map, value)
            }

    pub     fn swap_remove_by_key(&mut self, key:usize) -> Result<(V,Option<V>),Errors> { //removed_value
                fastvec_swap_remove_key(& mut self.vector,  &mut self.map , key) 
            }

    pub     fn swap_remove_by_value(&mut self, value:&V) -> Result<usize,Errors> {  //removed_key
                fastvec_swap_remove_value(& mut self.vector,  &mut self.map , value)
            }

    }
