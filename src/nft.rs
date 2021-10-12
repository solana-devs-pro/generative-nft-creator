use rand::thread_rng;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::attributes;

pub const TOTAL_EDITION_SIZE: usize = 88;

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
#[derive(Clone)]
pub struct NFT {
    pub num: usize,
    pub attributes: HashMap<AttributeType, Attribute>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum AttributeType {
    Background,
    Body,
    Face,
}

fn get_attributes(attr_type: AttributeType) -> Vec<Attribute> {
    let mut attributes = vec![];

    let dists = match attr_type {
        AttributeType::Background => attributes::background_types(),
        AttributeType::Body => attributes::body_types(),
        AttributeType::Face => attributes::face_types(),
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

pub fn get_all() -> Vec<NFT> {
    let backgrounds = get_attributes(AttributeType::Background);
    let bodies = get_attributes(AttributeType::Body);
    let faces = get_attributes(AttributeType::Face);

    let mut nfts : Vec<NFT> = vec![];

    for i in 0..TOTAL_EDITION_SIZE {

        // Setup attributes
        let mut attributes = HashMap::new();
        attributes.insert(AttributeType::Background, backgrounds[i].clone());
        attributes.insert(AttributeType::Body, bodies[i].clone());
        attributes.insert(AttributeType::Face, faces[i].clone());

        // setup NFT
        let nft = NFT { 
            num: i+1,
            attributes: attributes,
        };
        nfts.push(nft);
    }

    // get special ones to add
    let special_nfts = attributes::get_special_nfts();
    nfts.extend(special_nfts);

    // return the nfts
    nfts
}
