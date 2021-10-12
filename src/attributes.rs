use crate::nft::{NFT, TOTAL_EDITION_SIZE};
use rand::thread_rng;
use std::collections::HashMap;
use rand::seq::SliceRandom;

pub struct Distribution {
    pub name: String,
    pub total_number: usize,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub attr_type: AttributeType,
    pub rarity: f32,
}


#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum AttributeType {
    Background,
    Body,
    Face,
}

pub fn body_types() -> Vec<Distribution> {
    vec![
        Distribution { name: "yellow".to_string(), total_number: 5, },
        Distribution { name: "green".to_string(), total_number: 20, },
        Distribution { name: "normal".to_string(), total_number: 63, },
    ]
}

pub fn background_types() -> Vec<Distribution> {
    vec![
        Distribution { name: "super_rare".to_string(), total_number: 8, },
        Distribution { name: "black".to_string(), total_number: 40, },
        Distribution { name: "turquoise".to_string(), total_number: 40, },
    ]
}

pub fn face_types() -> Vec<Distribution> {
    vec![
        Distribution { name: "round".to_string(), total_number: 23, },
        Distribution { name: "square".to_string(), total_number: 27, },
        Distribution { name: "oblong".to_string(), total_number: 38, },
    ]
}

pub fn get_attributes(attr_type: AttributeType) -> Vec<Attribute> {
    let mut attributes = vec![];

    let dists = match attr_type {
        AttributeType::Background => background_types(),
        AttributeType::Body => body_types(),
        AttributeType::Face => face_types(),
    };

    for dist in dists {
        // cast to f32 for getting rarity
        let num_of_attrs: f32 = dist.total_number as f32;
        let total_num: f32 = TOTAL_EDITION_SIZE as f32; 

        // setup attribute
        let attribute = Attribute { 
            name: dist.name,
            attr_type: attr_type.clone(),
            rarity: num_of_attrs / total_num,
        };
        attributes.extend(vec![attribute; dist.total_number]);
    }

    // make sure to shuffle before returning
    // https://docs.rs/rand/0.6.4/rand/seq/trait.SliceRandom.html
    attributes.shuffle(&mut thread_rng());
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_type_total_num() {
        let dists = body_types();
        let mut num = 0;
        for dist in dists {
            num += dist.total_number;
        }
        assert_eq!(num, TOTAL_EDITION_SIZE);
    }

    #[test]
    fn test_face_type_total_num() {
        let dists = face_types();
        let mut num = 0;
        for dist in dists {
            num += dist.total_number;
        }
        assert_eq!(num, TOTAL_EDITION_SIZE);
    }

    #[test]
    fn test_bg_type_total_num() {
        let dists = background_types();
        let mut num = 0;
        for dist in dists {
            num += dist.total_number;
        }
        assert_eq!(num, TOTAL_EDITION_SIZE);
    }
}