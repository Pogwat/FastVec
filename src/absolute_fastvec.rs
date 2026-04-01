use crate::full_error::FullError;
use crate::absolute::*; 
use crate::shared::*;

    // pub struct FastVec<V> {
    //     pub   vector: Vec<V> ,//key, value
    //     pub   map: HashMap<V, usize>, //value, key
    //     #[cfg(feature = "FastRemove")]
    //     pub key_vec: KeyVec,
    // }

#[allow(dead_code)]
impl <V:Insertable>FastVec<V> {

    // fn absolute_remove(fake:usize){}

    pub fn absolute_get(&self ,fake:usize) -> Result<V,FullError> {
        if let Some(real_at_fake) = self.key_vec.get_fake(fake)? {
            Ok(self.get_by_key(real_at_fake)?)
        } else {Err(FullError::Absolute(AbsoluteErrors::FakeKeyOutOfBounds(fake)))}
    }

    pub fn absolute_push(&mut self ,value:V) -> (){
        self.key_vec.extend_initalize(1); 
        self.push_by_value(value);
    }

    pub fn absolute_remove(&mut self ,fake:usize) -> Result<(usize,V),FullError>{
        let (real,_) =  self.key_vec.remove_from_real_fake_by_fake(fake)?;
        let (real_value,_) = self.swap_remove_by_key(real)?;
        Ok((real,real_value))  
    }

     pub fn absolute_mod(&mut self ,fake:usize, newval:V) -> Result<(usize, V),FullError>{
        let real_key = self.key_vec.wrapped_get_fake(fake)?;
        let old_value = self.mod_to(real_key,newval)?;
        Ok((real_key,old_value))
     }
}