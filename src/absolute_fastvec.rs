use crate::shared::*;
use crate::absolute::*; 

    // pub struct FastVec<V> {
    //     pub   vector: Vec<V> ,//key, value
    //     pub   map: HashMap<V, usize>, //value, key
    //     #[cfg(feature = "FastRemove")]
    //     pub key_vec: KeyVec,
    // }

#[derive(Debug)]
pub enum FullError {
    FastVec(Errors),
    Absolute(AbsoluteErrors),
}

impl From<Errors> for FullError {
    fn from(err: Errors) -> Self {
        FullError::FastVec(err)
    }
}

impl From<AbsoluteErrors> for FullError {
    fn from(err: AbsoluteErrors) -> Self {
        FullError::Absolute(err)
    }
}



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

    // absolute_mod_to_real(fake:usize){}
}