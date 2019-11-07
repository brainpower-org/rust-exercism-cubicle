use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), | acc, (key, value) | {
        value.iter().fold(acc, |mut acc2, v| { 
            acc2.entry((*v).to_lowercase().next().unwrap()).or_insert(*key); acc2 
        }) 
    } )
}
