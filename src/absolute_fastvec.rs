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
        self.push_a_value(value);
    }

    pub fn absolute_remove(&mut self ,fake:usize) -> Result<(usize,V),FullError>{
        let (real,_) =  self.key_vec.remove_from_real_fake_by_fake(fake)?;
        let real_value = self.swap_remove_from_key(real)?;
        Ok((real,real_value))  
    }

    // absolute_mod_to_real(fake:usize){}
}