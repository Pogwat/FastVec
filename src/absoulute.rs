use crate::shared::*;
use hashbrown::HashMap;
use core::hash::Hash;





AbsoluteVec<V> {
    fastvec: FastVec<V>
    fake_to_real: HashMap<usize,usize>
    last_fake_real: (usize,usize)
}

impl <V:Hash+Eq+Clone>AbsoluteVec<V>{
     pub     fn swap_remove_by_key(&mut self, key:usize) -> Result<(V,Option<V>),Errors> { //removed_value //keys new value
                let real_key = fake_to_real.get_mut(&key).ok_or()?
                let (removed_value, new_value) = self.fastvec.swap_remove_by_key( key)? ;
                

            }

    pub     fn swap_remove_by_value(&mut self, value:&V) -> Result<usize,Errors> {  //removed_key
                self.fastvec.swap_remove_by_value( value)
            }   
}

