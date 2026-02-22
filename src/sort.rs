use crate::shared::*;
use core::hash::Hash;
use std::collections::BTreeMap;

    #[allow(dead_code)]
    pub struct SortedFastVec<V,B> {
        pub   fastvec:  FastVec<V>,
        pub   btreemap: BTreeMap<B,Vec<usize>>, //field_to_sort_by, key
        pub   ref_vec: Vec<(B,usize)> // same element index as fastvec contains btree key and vec value 
    }



        #[allow(dead_code)]
    fn keys_to_values<V:Insertable>(keys:&Vec<usize>, vec:&Vec<V>) -> Option<Vec<V>>{
        let vals: Vec<V> = keys.iter()
        .filter_map(|&k| vec.get(k))
        .cloned()
        .collect();
        Some(vals)
    }
    

    fn get_sort_keys<'a,V:Insertable,B:Ord>(btree:&'a BTreeMap<B,Vec<usize> >, sortval: &B) -> Option<&'a Vec<usize>>{ 
        btree.get(sortval)
    }

    fn get_sort_values<V:Insertable,B:Ord>(vec:&Vec<V>, btree:&BTreeMap<B, Vec<usize> >, sortval: &B) -> Option<Vec<V>>{
        let keys: &Vec<usize> = get_sort_keys::<V, B>(btree, sortval)?;
        keys_to_values(keys, &vec)
    }

    fn sort_push_key<'a,V:Insertable,B:Ord>( btree: &'a mut BTreeMap<B,Vec<usize> >, sortval: B, key:usize) -> (&'a mut Vec<usize>,usize){
        let entry = btree.entry(sortval).or_insert_with(Vec::new); //&mut vec<usize>
        let index = entry.len(); //usize
        entry.push(key);
        (entry, index) //return vec and index of pushed value 
    }

    fn fastvec_sort_push_at_val<V: Insertable, B:Ord>( fastvec: &mut FastVec<V>, btreemap: &mut BTreeMap<B,Vec<usize>>, sortval:B, val:V) -> (usize,usize){
        let last_index = fastvec.push(val); //returns last usize
        let entry_vec = btreemap.entry(sortval).or_insert_with(Vec::new); //insert val at sortval
        let entry_vec_index= entry_vec.len();
        entry_vec.push(last_index);
        (last_index, entry_vec_index)
    }
    fn sort_fastvec_push<V: Insertable, B:Ord + Clone>( fastvec: &mut FastVec<V>, btreemap: &mut BTreeMap<B,Vec<usize>>, ref_vec:&mut Vec<(B,usize)>,sortval:B, val:V) -> (usize,usize){
        let  (last_index, entry_vec_index) = fastvec_sort_push_at_val( fastvec, btreemap,sortval.clone(),val);
        ref_vec[last_index]= (sortval, entry_vec_index); //same as push
        (last_index, entry_vec_index)
    }
    // fn sort_fastvec_remove<V: Insertable, B:Ord + Clone>( fastvec: &mut FastVec<V>, btreemap: &mut BTreeMap<B,Vec<usize>>, ref_vec:&mut Vec<(B,usize)>, key:usize) -> (usize,usize){
    //     fastvec.remove_by_key(key);
    //     let (sortval, entry_vec_index) = ref_vec.remove(key);
    //     let mut_entry = btreemap.get_mut(&sortval)?;
    //     mut_entry.remove(entry_vec_index); //other vectors will have indexs changed

    // }

    sort_fastvec_by_key<V: Insertable, B:Ord + Clone>( fastvec: &mut FastVec<V>, btreemap: &mut BTreeMap<B,Vec<usize>>, ref_vec:&mut Vec<(B,usize)>, key:usize) -> (usize,usize){

    }

        //SORTED FASTVEC IMPLS

    #[allow(dead_code)]
    impl<V:Hash + Eq + Clone + Ord, B:Ord> SortedFastVec<V,B> {
    //copy pasted from above
        //CONSTRUCTORS
    
    pub     fn new() -> Self {
                Self {
                    fastvec: FastVec::new(),
                    btreemap: BTreeMap::new(),
                    ref_vec: Vec::new()
                }
            }

    // pub     fn with_capacity(size: usize) -> Self {
    //             Self {
    //                 FastVec::with_capacity(size),
    //             }
    //         } 
    
    //Wrappers

    pub const fn len(&self)-> usize {self.fastvec.len()}
    pub const fn capacity(&self) -> usize {self.fastvec.capacity()}
    pub fn reserve(&mut self, additional: usize){self.fastvec.reserve(additional);}

    //GETS

    pub     fn get_by_key(&self, key:usize) -> Result<V,Errors> {
                self.fastvec.get_by_key( key)
            }

    pub     fn get_by_value(&self, value:&V) -> Result<usize, Errors> {
                self.fastvec.get_by_value(value)       
            }

    //INSERTS

    pub     fn mod_by_key(&mut self, key:usize, newvalue:V) -> Result<V,Errors> {
                self.fastvec.mod_by_key(key, newvalue) 
            }

    pub     fn mod_by_value(&mut self,value:&V,newval:V) -> Result<V,Errors> {
                self.fastvec.mod_by_value(value,newval) 
            }

    pub     fn push(&mut self, value:V) -> usize{
                self.fastvec.push(value)
            }

    pub     fn insert(& mut self,key:usize, value:V) -> Result<(),Errors>  {
                self.fastvec.insert(key, value) 
            }

    //REMOVES

    pub     fn remove_by_key(&mut self, key:usize) -> Result<V,Errors> {
                self.fastvec.remove_by_key(key)
            }

    pub     fn remove_by_value(&mut self, value:&V) -> Result<usize, Errors> {
                self.fastvec.remove_by_value(value)
            }

    pub     fn swap_remove_by_key(&mut self, key:usize) -> Result<(V,Option<V>),Errors> { //removed_value
                self.fastvec.swap_remove_by_key( key) 
            }

    pub     fn swap_remove_by_value(&mut self, value:&V) -> Result<usize,Errors> {  //removed_key
                self.fastvec.swap_remove_by_value( value)
            }


    }