use hashbrown::HashMap;
use core::hash::Hash;
use core::fmt;
use core::cmp::Ordering;
use core::mem;
use core::ops::Index;
#[cfg(feature = "Sort")]  use std::collections::BTreeMap;
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

    #[cfg(feature = "Sort")]
    #[allow(dead_code)]
    pub struct SortedFastVec<V,B> {
        pub   vector: Vec<V> ,//key, value
        pub   map: HashMap<V, usize>, //value, key
        pub   btreemap: BTreeMap<B,Vec<usize>>, //field_to_sort_by, key
    }

//ARGS AND TRAIT BOUNDS

pub trait Insertable: Clone + Hash + Eq {}

    impl<T: Clone + Hash + Eq> Insertable for T {}

    #[derive(Default)] //Reusable with optionals
    struct Args<V: Insertable, #[cfg(feature = "Sort")]B:Ord>{
        map: Option<HashMap<V, usize> > ,
        vector: Option<Vec<V> > ,
        #[cfg(feature = "FastRemove")] refvec: Option<Vec<Option<usize>>>, 
        key: Option<usize>,
        value: Option<V>,
        #[cfg(feature = "Sort")] btreemap: Option<BTreeMap<B,Vec<usize>> >,
        #[cfg(feature = "Sort")] sortvalue: Option< B >
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
    #[cfg(feature = "Sort")] 
    fn get_sort_keys<'a,V:Insertable,B:Ord>(btree:&'a BTreeMap<B,Vec<usize> >, sortval: &B) -> Option<&'a Vec<usize>>{ 
        btree.get(sortval)
    }

    fn keys_to_values<V:Insertable>(keys:&Vec<usize>, vec:&Vec<V>) -> Option<Vec<V>>{
        let vals: Vec<V> = keys.iter()
        .filter_map(|&k| vec.get(k))
        .cloned()
        .collect();
        Some(vals)
    }
    
    #[cfg(feature = "Sort")]
    fn get_sort_values<V:Insertable,B:Ord>(vec:&Vec<V>, btree:&BTreeMap<B, Vec<usize> >, sortval: &B) -> Option<Vec<V>>{
        let keys: &Vec<usize> = get_sort_keys::<V, B>(btree, sortval)?;
        keys_to_values(keys, &vec)
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
    
    fn fastvec_push<V: Insertable>( vec: &mut Vec<V>, map: &mut HashMap<V, usize>, value: V) -> Option<usize>{ //SHould return None if no duplicate entrys, else Some(usize) 
        let index = push_vec(vec, value.clone());
        map.insert(value, index) //returns lastkey Option<usize>
    }
    
    #[cfg(feature = "Sort")]
    fn sort_push_key<'a,V:Insertable,B:Ord>( btree: &'a mut BTreeMap<B,Vec<usize> >, sortval: B, key:usize) -> (&'a mut Vec<usize>,usize){
        let entry = btree.entry(sortval).or_insert_with(Vec::new); //&mut vec<usize>
        let index = entry.len(); //usize
        entry.push(key);
        (entry, index) //return vec and index of pushed value 
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

    fn fastvec_swap_remove_key<V:Insertable>( vec: &mut Vec<V>,  map: &mut HashMap<V, usize>, key:usize) -> Result<V,Errors> { //removed_value 
        let (removed_value, new_value) = swap_remove_by_key_old_new(vec,key)?;
        map.remove(&removed_value);
        if let Some(value) = new_value { //A None value would occur if key=last index, this is valid and shouldnt return error, just remove from the map
            map.insert(value, key);
        }
        // else {
        //     return Err(Errors::ValueOutOfBounds)
        // };

        Ok(removed_value)
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

    //Refrence
    
    //Remove
    
   // #[cfg(feature = "FastRemove")] 


    //Get
  //  #[cfg(feature = "FastRemove")] 


    //INsert
    //#[cfg(feature = "FastRemove")] 


//IMPLS

    //FASTVEC IMPLS

    #[allow(dead_code)] //Backend Shared helpers for map vector
    impl<V:Hash + Eq + Clone + Ord> FastVec<V> {

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

    pub     fn len(&self)-> usize {self.vector.len()}

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

    pub     fn push(&mut self, value:V) -> Option<usize>{
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

    pub     fn swap_remove_by_key(&mut self, key:usize) -> Result<V,Errors> { //removed_value
                fastvec_swap_remove_key(& mut self.vector,  &mut self.map , key) 
            }

    pub     fn swap_remove_by_value(&mut self, value:&V) -> Result<usize,Errors> {  //removed_key
                fastvec_swap_remove_value(& mut self.vector,  &mut self.map , value)
            }

    }

    //SORTED FASTVEC IMPLS

    #[allow(dead_code)]
    #[cfg(feature = "Sort")] 
    impl<V:Hash + Eq + Clone + Ord, B:Ord> SortedFastVec<V,B> {

    }