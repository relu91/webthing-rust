use std::collections::btree_map::BTreeMap;
use super::w3c_list::W3CList;




#[derive(Debug)]
pub (crate) struct DescriptiveData {
    pub stype : W3CList<String>,
    pub title           : Option<String>,
    pub description     : Option<String> ,
    pub titles  : BTreeMap<String,String>,
    pub descriptions   : BTreeMap<String,String>,

}

impl DescriptiveData {
    pub (crate) fn new() -> Self {
        Self {
            stype : W3CList::None,
            title : None,
            description  : None,
            titles :  BTreeMap::new(),
            descriptions : BTreeMap::new()
        }    
    }
}
