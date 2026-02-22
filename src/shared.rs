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

    pub trait ValueMapKeyVec<V:Insertable>{
        /*      NOTES 
        Keys are stored in a Map
        Values are stored in A Vec
        */
        //HashMap<value,key>
        //Vec<key,value>

        fn get_by_key(&self, key:usize) -> Result<V,Errors>;

        fn get_by_value(&self, value:&V) -> Result<usize,Errors>;

        //remove_a_key(&mut self, key:usize) -> Result<V,Errors>

        fn remove_by_value(&mut self, value:V) -> Result<usize,Errors>;

        fn mod_to_key(&mut self, key:usize) -> Result<&mut V,Errors>; //My solution :)

        fn mod_to_value(&mut self, value:&V) -> Result<&mut usize,Errors>;

        // fn mod_to_keys(&mut self) -> &mut Vec<V>; //Cant have mutable refrence to self multiple times so one big refrence it is

         //fn mod_to_values(&mut self) -> &mut HashMap<V,usize>;

        fn push_a_value_to_key(&mut self, value:V, key:usize) -> ();

        fn push_a_value(&mut self, value:V) -> ();

        fn pop_from_keys(&mut self) -> ();

        fn len_of_vec(&self) -> usize;



        //Using previous impl methods

        fn swap_muts(&mut self, key1:usize, key2:usize) -> Result<(),Errors>{
            let key1_mod = self.mod_to_key(key1)?;
            let key1_ptr = ptr::from_mut(key1_mod);
            let key2_mod = self.mod_to_key(key2)?;
            let key2_ptr = ptr::from_mut(key2_mod);
            unsafe {
            ptr::swap(key1_ptr, key2_ptr);
            }
            Ok(())
        }

        fn swap_values(&mut self, value1:&V, value2:&V) -> Result<(),Errors>{
            let value1_mod = self.mod_to_value(value1)?;
            let value1_ptr = ptr::from_mut(value1_mod);
            let value2_mod = self.mod_to_value(value2)?;
            let value2_ptr  = ptr::from_mut(value2_mod);
            unsafe {
            ptr::swap(value1_ptr, value2_ptr);
            }
            Ok(())
        }

        fn swap_two_muts<T>(mut1:& mut T, mut2:& mut T) -> (){
            let key1_ptr = ptr::from_mut(mut1);
            let key2_ptr = ptr::from_mut(mut2);
            unsafe {
            ptr::swap(key1_ptr, key2_ptr);
            }
        }





        // fn get_key_from_value(&self, value:&V) -> Result<V,Errors> {
        //     let key = self.get_by_value(value)?;
        //     let vvalue = self.get_by_key(key)?;
        //     Ok(vvalue)  
        // }

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

        // fn swap_keys(&mut self, key1:usize, key2:usize) -> Result<(),Errors> {
        //     let key_mod = self.mod_to_keys();
        //     let key1_ptr = &raw mut key_mod[key1];
        //     let key2_ptr = &raw mut key_mod[key2];

        //     unsafe {
        //         ptr::swap(key1_ptr, key2_ptr);
        //     }
        //     Ok(())
        // }

        // fn swap_values(&mut self, value1:V, value2:V) -> Result<(),Errors> {
        //     let key_mod = self.mod_to_values();
        //     let value1_ptr = &raw mut *key_mod.get_mut(&value1).unwrap();   
        //     let value2_ptr = &raw mut *key_mod.get_mut(&value2).unwrap();    

        //     unsafe {
        //         ptr::swap(value1_ptr, value2_ptr);
        //     }
        //     Ok(())
        // }

        fn last_index(&self) -> usize { //get last elemnt of vector
            self.len_of_vec()-1
        }

        fn key_swap_remove(&mut self, key1:usize ) -> Result<V,Errors> { //swaps key with last and pops for a vector
            let value = self.get_by_key(key1)?;
            let last_index = self.last_index();
            self.swap_muts(key1,last_index)?;
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

        fn push_by_value(&mut self, value:V) -> usize {    
            self.push_a_value(value.clone());
            let last_index = self.last_index();
            self.push_a_value_to_key(value, last_index);
            last_index
        }

        // fn mod_to(&mut self, key:usize, value:V) -> Result<(V,Option<usize>),Errors>{
        //     self.bounds_check(key)?; 
        //     let keys = self.mod_to_keys();
        //     let old_value = mem::replace(&mut keys[key], value.clone());
        //     let values = self.mod_to_values(); 
        //     values.remove(&old_value);     
        //     let old_key_of_value: Option<usize> = values.insert(value,key);
        //     Ok((old_value,old_key_of_value))
        // }

        fn bounds_check(&self, key:usize) -> Result<usize,Errors>{
            let last_index = self.last_index();
            if !key<=last_index {
                return Err(Errors::KeyOutOfBounds)
            }
            Ok(last_index)
        }


    }

impl <V:Insertable>ValueMapKeyVec<V> for FastVec<V> {
    fn get_by_key(&self, key:usize) -> Result<V,Errors> {
        let element = self.vector.get(key).ok_or(Errors::KeyOutOfBounds)?.clone();
        Ok(element)
    }

    fn get_by_value(&self, value:&V) -> Result<usize,Errors> {
        let element = self.map.get(value).ok_or(Errors::ValueOutOfBounds)?.clone();
        Ok(element)
    }

    fn remove_by_value(&mut self, value:V) -> Result<usize,Errors> {
        let value = self.map.remove(&value).ok_or(Errors::ValueOutOfBounds)?;
        Ok(value)
    }

    // fn mod_to_keys(&mut self) -> &mut Vec<V> {
    //     &mut self.vector 
    // }

    fn mod_to_key(&mut self, key:usize) -> Result<&mut V,Errors> {
        self.bounds_check(key)?;
        let refrence = &mut self.vector[key];
        Ok(refrence)
    }
    
    fn mod_to_value(&mut self, value:&V) -> Result<&mut usize,Errors> {
        match self.map.get_mut(value) {
            Some(index) => Ok(index),
            None => Err(Errors::ValueOutOfBounds),
        }
    }

    // fn mod_to_values(&mut self) -> &mut HashMap<V,usize> {
    //     &mut self.map
    // }

    fn push_a_value_to_key(&mut self, value:V, key:usize) -> (){
        self.map.insert(value,key);
    }

    fn push_a_value(&mut self, value:V) -> () {
        self.vector.push(value);
    }

    fn pop_from_keys(&mut self) -> () {
        self.vector.pop();
    }

    fn len_of_vec(&self) -> usize {
        self.vector.len()
    }
}




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

}
