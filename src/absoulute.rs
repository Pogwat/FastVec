use crate::swap_refs;
use core::fmt;
use core::ptr;

    #[derive(Debug)]
    pub struct KeyVec { 
        pub reals_fake: Vec<usize>, 
        pub fakes: Vec<Option<usize>>
    }

//ERRORS
    #[derive(Debug)]
    pub enum Errors {
        FakeKeyOutOfBounds(usize),
        RealsFakeKeyOutOfBounds(usize)
    }

    impl fmt::Display for Errors {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Errors::FakeKeyOutOfBounds(fake_key) => write!(f, "Fake Key: {} is Out of Bounds", fake_key),
                Errors::RealsFakeKeyOutOfBounds(reals_fake_key) => write!(f, "Reals Fake Key: {}  is Out of Bounds", reals_fake_key ),
            }
        }
    }


pub trait AbsoluteKeys{

    //Real and RealsFake are copuled. could be a tuple instead: Vec<(V,FakeKey)>

    // Real: Vec<V>   
    // RealsFake: Vec<usize>
    // Fakes: Vec<usize>

    fn get_fake(&self, key:usize) -> Result<Option<usize>,Errors>;
    fn mod_to_fake(&mut self, key:usize) -> Result<&mut Option<usize>,Errors>;
    fn pop_fake(&mut self) -> ();
    fn push_fake(&mut self, value:usize) -> ();
    fn fakes_len(&self) -> usize;
    fn delete_fake(&mut self, key:usize) -> Result<Option<usize>,Errors>;

    fn reals_fake_len(&self) -> usize;
    fn get_reals_fake(&self, key:usize) -> Result<usize,Errors>;
    fn mod_to_reals_fake(&mut self,key:usize) -> Result<&mut usize,Errors>; 
    fn pop_reals_fake(&mut self) -> ();
    fn push_reals_fake(&mut self, key:usize) -> ();

    //Using above impl methods

    fn swap_fakes(&mut self, fake1:usize, fake2:usize) -> Result<(),Errors> {
        swap_refs!(self.mod_to_fake(fake1)?, self.mod_to_fake(fake2)?);
        Ok(())
    }

    fn swap_reals_fake(&mut self, reals_fake1:usize, reals_fake2:usize) -> Result<(),Errors> {
        swap_refs!(self.mod_to_reals_fake(reals_fake1)?, self.mod_to_reals_fake(reals_fake2)?);
        Ok(())
    }

    fn last_real(&self) -> usize  {
        self.reals_fake_len()-1
    }

    fn last_fake(&self) -> usize {
        self.fakes_len()-1
    }

    fn swap_remove_reals_fake(&mut self, reals_fake1:usize) -> Result<(usize,usize),Errors> { //Returns the fakes stored in at reals_fake1 and last_real_fake
        let last_reals_fake_index = self.last_real();
        let (fake1, last_reals_fake) = (self.get_reals_fake(reals_fake1)?, self.get_reals_fake(last_reals_fake_index)?);
        self.swap_reals_fake(reals_fake1, last_reals_fake_index)?;
        self.pop_reals_fake();
        Ok((fake1, last_reals_fake))
    }

    fn remove_from_real_fake_by_fake(&mut self, fake1:usize) -> Result<(usize,usize),Errors>{ //Returns new key of last real-fake and its fake key
        let real1 = self.get_fake(fake1)?.unwrap();    //get real value to remove 
        let last_rf = self.get_reals_fake(self.last_real())?; //get fake key of value that is bveing swapped into real1

        self.delete_fake(fake1)?; //set the fake key we are removing to None
        self.swap_remove_reals_fake(real1)?; // remove value at real1 replace it with last
        //swap_remove_from_reals(real1)?; // remove value at real1 replace it with last

        *self.mod_to_fake(last_rf)? = Some(real1); // update the real key stored for fake
        //*self.mod_to_reals_fake(real1)? = last_rf; //update fake stored for real key //FAKE INDEX SHOULD NOT CHNAGE!!!!
        Ok((real1,last_rf))
    }

    // fn remove_by_fake(&mut self, fake1:usize) -> Result<(usize,usize),Errors>{
    //     let (real_to_remove, real_fake_lasts_new_key) = remove_from_real_fake_by_fake(fake1)?;
    //     self.swap_remove_from_reals(real1)?;
    //     Ok((real_to_remove, real_fake_lasts_new_key))
    // }

}

impl AbsoluteKeys for KeyVec {

    //FAKES
        fn get_fake(&self, key:usize) -> Result<Option<usize>,Errors> {
            Ok(self.fakes.get(key).ok_or(Errors::FakeKeyOutOfBounds(key))?.clone())
        }

        fn mod_to_fake(&mut self, key:usize) -> Result<&mut Option<usize>,Errors> {
            let fake_entry = self.fakes.get_mut(key).ok_or(Errors::FakeKeyOutOfBounds(key))?; 
            Ok(fake_entry) 
 
        }

        fn pop_fake(&mut self) -> (){
            self.fakes.pop();
        }

        fn push_fake(&mut self, value:usize) -> () {
            self.fakes.push(Some(value));
        }

        fn fakes_len(&self) -> usize {
            self.fakes.len()
        }

        fn delete_fake(&mut self, key:usize) -> Result<Option<usize>,Errors>{
            let fake_entry = self.fakes.get_mut(key).ok_or(Errors::FakeKeyOutOfBounds(key))?;
            Ok(fake_entry.take())
        }

    //REALS FAKE
        fn reals_fake_len(&self) -> usize {
            self.reals_fake.len()
        }

        fn get_reals_fake(&self,key:usize) -> Result<usize,Errors> {
            Ok(*self.reals_fake.get(key).ok_or(Errors::RealsFakeKeyOutOfBounds(key))?)
        }

        fn mod_to_reals_fake(& mut self,key:usize) -> Result<&mut usize,Errors> {
            Ok(self.reals_fake.get_mut(key).ok_or(Errors::RealsFakeKeyOutOfBounds(key))?)
        }

        fn pop_reals_fake(& mut self) -> () {
            self.reals_fake.pop();
        }

        fn push_reals_fake(&mut self, key:usize) -> () {
            self.reals_fake.push(key)
        }

}

impl KeyVec {
    pub fn new() -> Self {
        Self {
            reals_fake:    Vec::new()  ,
            fakes:    Vec::new()  
        }
    }

    pub fn with_capacity(size:usize) -> Self {
        Self {
            reals_fake:    Vec::with_capacity(size),
            fakes:    Vec::with_capacity(size)
        }
    }
}