use hashbrown::HashMap;
use core::hash::Hash;
use core::fmt;
//use core::cmp::Ordering;
use core::mem;
use core::ops::Index;
use core::ptr;
#[cfg(feature = "FastRemove")] use crate::absolute::KeyVec;

//STRUCTS

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct FastVec<V> {
        pub   vector: Vec<V> ,//key, value
        pub   map: HashMap<V, usize>, //value, key
        #[cfg(feature = "FastRemove")]
        pub key_vec: KeyVec,
    }

//ARGS AND TRAIT BOUNDS

pub trait Insertable: Clone + Hash + Eq {}

    impl<T: Clone + Hash + Eq> Insertable for T {}
    #[allow(dead_code)]
    #[derive(Default)] //Reusable with optionals
    struct Args<V: Insertable>{
        map: Option<HashMap<V, usize> > ,
        vector: Option<Vec<V> > ,
    // #[cfg(feature = "FastRemove")] refvec: Option<Vec<Option<usize>>>, 
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

    //THE BORROW CHECKER IS SO DUMB IT WONT ALLOW NON-CONCURRENT! MUTABLE BORROWS TO SELF
    //SO NOW I NEED A MACRO BEACUSE ITS SO DUMB :(
    //Funciton args are evaluted before body. so passing 2 mutable refrences to a struct  will fail, regardless of how they are used in the funciton
        //     fn swap_refs<T>(a: &mut T, b: &mut T) {
        //          unsafe {
        //              std::ptr::swap(ptr::from_mut(a), ptr::from_mut(b));
        //          }
        //      }
    #[macro_export]
    macro_rules! swap_refs {
    ($a:expr, $b:expr) => {{
        let ptr1 = ptr::from_mut($a);
        let ptr2 = ptr::from_mut($b);
        unsafe { ptr::swap(ptr1, ptr2) };
    }};
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

        fn remove_by_value(&mut self, value:&V) -> Result<usize,Errors>;

        fn mod_to_key(&mut self, key:usize) -> Result<&mut V,Errors>; //My solution :)

        fn mod_to_value(&mut self, value:&V) -> Result<&mut usize,Errors>;

        fn push_a_value_to_key(&mut self, value:V, key:usize) -> ();

        fn push_a_value(&mut self, value:V) -> ();

        fn pop_from_keys(&mut self) -> ();

        fn len_of_vec(&self) -> usize;

        //Using previous impl methods

        fn swap_values(&mut self, value1:&V, value2:&V) -> Result<(),Errors>{
            swap_refs!(self.mod_to_value(value1)?,self.mod_to_value(value2)?);
            Ok(())
        }

        fn swap_keys(&mut self, key1:usize, key2:usize) -> Result<(),Errors> {
            swap_refs!(self.mod_to_key(key1)?, self.mod_to_key(key2)?); //LOL, YOU NEED A MACRO. GRRR! :((((((
            Ok(())
        }

        fn swap_by_keys(&mut self, key1:usize, key2:usize) -> Result<(),Errors> {
            let (value1,value2) = (self.get_by_key(key1)? , self.get_by_key(key2)?);
            self.swap_keys(key1,key2)?;
            self.swap_values(&value1,&value2)?;
            Ok(())
        }

        fn last_index(&self) -> usize { //get last elemnt of vector
            self.len_of_vec()-1
        }

        fn key_swap_remove(&mut self, key:usize ) -> Result<V,Errors> { //swaps key with last and pops for a vector
            let value = self.get_by_key(key)?;
            let last_index = self.last_index();
            self.swap_keys(key,last_index)?;
            self.pop_from_keys(); //should probably require a pop impl and use .pop() instead of .remove()
            Ok(value)
        }

        //remove_by_key() //UNSAFE

        fn swap_remove_from_value(&mut self, value:&V) -> Result<V,Errors> { //remove from hashmap + swaprm on vec. by value
            let key = self.remove_by_value(value)?;
            let value_ = self.key_swap_remove(key)?;
            Ok(value_)
        }

        fn swap_remove_from_key(&mut self, key:usize) -> Result<V,Errors> { //swap-rm key from vec, use value at key to remove from hashmap
            let value = self.key_swap_remove(key)?;
            self.remove_by_value(&value)?;
            Ok(value)

        }

        fn push_by_value(&mut self, value:V) -> usize {    
            self.push_a_value(value.clone());
            let last_index = self.last_index();
            self.push_a_value_to_key(value, last_index);
            last_index
        }

        fn mod_to(&mut self, key:usize, value:V) -> Result<V,Errors>{
            let old_value = mem::replace (self.mod_to_key(key)?, value.clone()); /*Bounds check is done here, also on mod_to_value*/
            self.remove_by_value(&old_value)?;
            self.push_a_value_to_key(value,key);
            Ok(old_value)
        }      

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

    fn remove_by_value(&mut self, value:&V) -> Result<usize,Errors> {
        let value = self.map.remove(value).ok_or(Errors::ValueOutOfBounds)?;
        Ok(value)
    }

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
                    key_vec:  KeyVec::new() ,
                }
            }

    pub     fn with_capacity(size: usize) -> Self {
                Self {
                    vector: Vec::with_capacity(size),
                    map: HashMap::with_capacity(size),
                    #[cfg(feature = "FastRemove")]
                    key_vec:  KeyVec::with_capacity(size)
                }
            } 

        //Wrappers

    pub const fn len(&self)-> usize {self.vector.len()}
    pub const fn capacity(&self) -> usize {self.vector.capacity()}
    pub fn reserve(&mut self, additional: usize){
        self.vector.reserve(additional);
        self.map.reserve(additional);}

}