use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use crate::attributes::{Attribute, AttrType};

mod attributes;

#[derive(Clone)]
struct NFT {
    num: usize,
    attributes: HashMap<AttrType, Attribute>,
}

fn main() {
    let backgrounds = attributes::get_all(AttrType::Background);
    let bodies = attributes::get_all(AttrType::Body);
    let faces = attributes::get_all(AttrType::Face);
    
    let mut nfts : Vec<NFT> = vec![];

    for i in 0..attributes::TOTAL_NUMBER_OF_ITEMS {

        // Setup attributes
        let mut attributes = HashMap::new();
        attributes.insert(AttrType::Background, backgrounds[i].clone());
        attributes.insert(AttrType::Body, bodies[i].clone());
        attributes.insert(AttrType::Face, faces[i].clone());

        // setup NFT
        let nft = NFT { 
            num: i+1,
            attributes: attributes,
        };
        nfts.push(nft);
    }

    for nft in nfts {
        println!("nft id: {}", nft.num);
        println!("\t {:?}", nft.attributes.get(&AttrType::Background).unwrap());
        println!("\t {:?}", nft.attributes.get(&AttrType::Body).unwrap());
        println!("\t {:?}", nft.attributes.get(&AttrType::Face).unwrap());
    }
    
}