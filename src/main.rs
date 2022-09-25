mod bitset;
use bitset::{BitSet32, u32_to_vec};
use bitmaps::Bitmap;
use std::collections::BTreeMap;
use itertools::Itertools;
/*


struct BitSet32 {
    bitmap: Bitmap<32>, 
}


impl BitSet32 {

    pub fn new() -> BitSet32 {
        BitSet32 { bitmap: Bitmap::new()}
    }

    pub fn add(&mut self, member: usize) {
        if member < 32 {
            self.bitmap.set(member, true);
        }
        else { 
            panic!("member {} out of range (max 32)",member);
        }
    }
    
    pub fn remove(&mut self, member: usize) {
        if member < 32 {
            self.bitmap.set(member, false);
        }
        else { 
            panic!("member {} out of range (max 32)",member);
        }
    }

    pub fn add_from_vec(&mut self, list: Vec<usize>) {
        for item in list {
            self.add(item);
        }
    }

    pub fn get_set_id(&self) -> u32 {
        self.bitmap.into_value()
    }

    pub fn get_vec(&self) -> Vec<usize> {
        let indices: Vec<usize> = self.bitmap.into_iter().collect();
        indices
    }


}

pub fn u32_to_vec(value: u32) -> Vec<usize> {

    let mut remaining = value;
    let mut result = Vec::<usize>::new();


    let mut index  : usize = 0;
    while remaining > 0 {
        if remaining & 1 == 1 {
            result.push(index)
        }
        remaining = remaining >> 1;
        index += 1;
    }
    result

}
*/

fn main() {

    let data = vec!(5,7,3,1,);

    let mut bitmap: Bitmap<16> = Bitmap::new();
    for i in data {
        bitmap.set(i,true);
    }
    let value :u32 = bitmap.into_value().into();

    println!("Bitmap is {:?} value is {}",bitmap, value);

    let vertex = vec![0,1,2,3];
    let vset = vertex.into_iter().combinations(2);
    let mut set_map = BTreeMap::<u32,i32>::new();
    println!("VSet {:?}",vset);
    for set in vset {
        println!("Set: {:?}",set);
        let mut bitset = BitSet32::new();
        bitset.add_from_vec(set);
        let setid = bitset.get_set_id();
        let value = setid as i32;
        set_map.insert(setid,value);

    }

    println!("Set Map {:#?}",set_map);
    for (key, val) in set_map {
        println!("Found {:?} -> {:?}",key, u32_to_vec(key));
        
    }


}
