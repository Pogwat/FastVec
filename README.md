# FastVec
A Vector with fast lookup by key or value, fast insertion and fast iteration. Uses Hashmap and Vec

# CURRENTLY EXPERIMENTAL AND UNFINISHED. DO NOT USE

## Usage
Since this is super experimental funciton names might change and some funcitons may break!
At the moment here are the functons this has:
```rust 
get_by_key(&self, key:usize) -> Option<&V>
get_by_value(&self, value:&V) -> Option<usize>
mod_by_key(&mut self, key:usize, newvalue:V) -> Result<V,Errors>
mod_by_value(&mut self,value:&V,newval:V) -> Result<V,Errors>
push(&mut self, value:V) -> Option<usize>
insert(& mut self,key:usize, value:V) -> Result<(),Errors>
remove_by_key(&mut self, key:usize) -> V
remove_by_value(&mut self, value:&V) -> Result<usize, Errors>
swap_remove_by_key(&mut self, key:usize) -> Result<V,Errors>
swap_remove_by_value(&mut self, value:&V) -> Result<usize,Errors> 
```



