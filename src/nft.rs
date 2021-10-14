use rand::thread_rng;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::attributes::{self, Attribute, AttributeType};

pub const TOTAL_EDITION_SIZE: usize = 88;
const SPECIAL_NFT_IDS: [usize; 2] = [8, 88];

#[derive(Clone)]
pub struct NFT {
    pub num: usize,
    pub attributes: HashMap<AttributeType, Attribute>,
}

pub fn get_all() -> Vec<NFT> {
    let backgrounds = attributes::get_attributes(AttributeType::Background);
    let bodies = attributes::get_attributes(AttributeType::Body);
    let faces = attributes::get_attributes(AttributeType::Face);
    let horns = attributes::get_attributes(AttributeType::Horn);

    let mut nfts : Vec<NFT> = vec![];

    for i in 0..TOTAL_EDITION_SIZE {

        // Setup attributes
        let mut attributes = HashMap::new();
        attributes.insert(AttributeType::Background, backgrounds[i].clone());
        attributes.insert(AttributeType::Body, bodies[i].clone());
        attributes.insert(AttributeType::Face, faces[i].clone());
        attributes.insert(AttributeType::Horn, horns[i].clone());
       
        let num = i+1;

        if SPECIAL_NFT_IDS.contains(&num) {
            println!("skip {}", num);
            continue; // add special NFT here
        } else {
            // setup NFT
            let nft = NFT { 
                num: num,
                attributes: attributes,
            };
            nfts.push(nft);
        }
   }

    // get special ones to add
    let special_nfts = get_special_nfts();
    nfts.extend(special_nfts);

    // shuffle
    nfts.shuffle(&mut thread_rng());
    
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
