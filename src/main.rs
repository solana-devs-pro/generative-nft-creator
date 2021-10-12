use std::collections::HashMap;
use crate::nft::NFT;
use crate::attributes::AttributeType;

// image stuff
use image::GenericImageView;

mod attributes;
mod nft;

fn main() {
    let nfts = nft::get_all();

    for nft in nfts {
        println!("nft id: {}", nft.num);
        println!("\t {:?}", nft.attributes.get(&AttributeType::Background).unwrap());
        println!("\t {:?}", nft.attributes.get(&AttributeType::Body).unwrap());
        println!("\t {:?}", nft.attributes.get(&AttributeType::Face).unwrap());
    }
}