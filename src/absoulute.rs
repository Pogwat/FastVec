use crate::shared::*;
use hashbrown::HashMap;
use core::hash::Hash;
use crate::shared::swap_refs;

 struct KeyVec<V> { 
    fake_key_of_corosponding_real: Vec<usize> 
    fake_keys: Vec<Opiton<usize>>
 }

//ERRORS
    #[derive(Debug)]
    pub enum Errors {
        FakeKeyOutOfBounds,
        RealKeyOutOfBounds
    }

    impl fmt::Display for Errors {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Errors::FakeKeyOutOfBounds => write!(f, "Fake Key is Out of Bounds"),
                Errors::RealsFakeKeyOutOfBounds => write!(f, "Reals Fake Key is Out of Bounds"),
                Errors::RealKeyOutOfBounds => write!(f, "Real Key is Out of Bounds"),
            }
        }
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

pub trait AbsoluteKeys{

//Real and RealsFake are copuled. could be a tuple instead: Vec<(V,FakeKey)>

// Real: Vec<V>   
// RealsFake: Vec<usize>
// Fakes: Vec<usize>


get_fake(&self, key:usize) -> Result<usize,Errors>;
mod_to_fake(&mut self, key:usize) -> Result<&mut usize,Errors>;
pop_fake(&mut self) -> ();
push_fake(&mut self, value:usize) -> ();
fakes_len(&self) -> usize;
delete_fake(&mut self, key:usize) -> Result<Option<usize>,Errors>;

reals_fake_len(&self) -> usize;
get_reals_fake(usize:key) -> Result<usize,Errors>
mod_to_reals_fake(usize:key) -> Result<&mut usize,Errors>; 
pop_reals_fake() -> ();
push_reals_fake(&mut self, key:usize) -> ();
//swap_remove_reals_fake(usize:key) -> Result<usize,Errors>


//swap_remove_from_reals(&mut self, key1:usize) -> Result<V,Errors>

//Using above impl methods

swap_fakes(&mut self, fake1:usize, fake2:usize) -> Result<(),Errors> {
    swap_refs!(mod_to_fake(fake1)?, mod_to_fake(fake2)?);
}

swap_reals_fake(&mut self, reals_fake1:usize, reals_fake2:usize) -> Result<(),Errors> {
    swap_refs!(mod_to_reals_fake(reals_fake1)?, mod_to_reals_fake(reals_fake2)?);
}

last_real(&self) -> usize  {
    reals_fake_len()-1
}

last_fake(&self) -> usize {
    fakes_len()-1
}


swap_remove_reals_fake(&mut self, reals_fake1:usize) -> Result<(usize,usize),Errors> { //Returns the fakes stored in at reals_fake1 and last_real_fake
    let last_reals_fake_index = last_real();
    let (fake1, last_reals_fake) = (get_reals_fake(reals_fake1)?, get_reals_fake(last_reals_fake_index)?);
    swap_reals_fake(mod_to_reals_fake(reals_fake1)?, mod_to_reals_fake(last_reals_fake_index)?)?;
    pop_reals_fake();
    Ok((fake1, last_reals_fake))
}

remove_from_real_fake_by_fake(&mut self, fake1:usize) -> Result<(usize,usize),Errors>{ //Returns new key of last real-fake and its fake key
    let real1 = get_fake(fake1)?;    //get real value to remove 
    let last_rf = get_reals_fake(last_real())?; //get fake key of value that is bveing swapped into real1

    delete_fake(fake1)?; //set the fake key we are removing to None
    swap_remove_reals_fake(real1)?; // remove value at real1 replace it with last
    //swap_remove_from_reals(real1)?; // remove value at real1 replace it with last

    *mod_to_fake(last_rf)? = real1; // update the real key stored for fake
    *mod_to_reals_fake(real1)? = last_rf; //update fake stored for real key
    Ok((real1,last_rf))
}

remove_by_fake(&mut self, fake1:usize) -> Result<(usize,usize),Errors>{
    let (real_to_remove, real_fake_lasts_new_key) = remove_from_real_fake_by_fake(fake1)?;
    swap_remove_from_reals(real1)?;
    Ok((real_to_remove, real_fake_lasts_new_key))
}



}

impl <V:Insertable> AbsoluteKeys for FastVec<V> {

//FAKES
get_fake(&self, key:usize) -> Result<usize,Errors> {
    self.fakes.get(key).ok_or(Errors::FakeKeyOutOfBounds)?
}

mod_to_fake(&mut self, key:usize) -> Result<&mut usize,Errors> {
    self.fakes.get_mut(key).ok_or(Errors::FakeKeyOutOfBounds)?
}

pop_fake(&mut self) -> (){
    self.fakes.pop();
}

push_fake(&mut self, value:usize) -> () {
    self.fakes.push(Some(value));
}

fakes_len(&self) -> usize {
    self.fakes.len()
}

delete_fake(&mut self, key:usize) -> Result<Option<usize>,Errors>{
    self.fakes[key] = None;
}


//REALS FAKE
reals_fake_len(&self) -> usize {
    self.reals_fake.len()
}

get_reals_fake(usize:key) -> Result<usize,Errors> {
    self.reals_fake.get(key).ok_or(Errors::RealsFakeKeyOutOfBounds)?
}

mod_to_reals_fake(usize:key) -> Result<&mut usize,Errors> {
    self.reals_fake.get_mut(key).ok_or(Errors::RealsFakeKeyOutOfBounds)?
}

pop_reals_fake() -> () {
    self.reals_fake.pop();
}

push_reals_fake(&mut self, key:usize) -> () {
    self.reals_fake.push(key)
}

}

