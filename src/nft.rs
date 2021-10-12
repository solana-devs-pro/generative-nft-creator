use rand::thread_rng;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::attributes::{self, Attribute, AttributeType};

pub const TOTAL_EDITION_SIZE: usize = 88;

#[derive(Clone)]
pub struct NFT {
    pub num: usize,
    pub attributes: HashMap<AttributeType, Attribute>,
}

pub fn get_all() -> Vec<NFT> {
    let backgrounds = attributes::get_attributes(AttributeType::Background);
    let bodies = attributes::get_attributes(AttributeType::Body);
    let faces = attributes::get_attributes(AttributeType::Face);

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
    let special_nfts = get_special_nfts();
    nfts.extend(special_nfts);

    // return the nfts
    nfts
}

fn get_special_nfts() -> Vec<NFT> {
    vec![
        // add your Attribute here
        // NFT {
        // }
    ]
}
