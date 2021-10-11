use std::iter::Map;
use std::collections::HashMap;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::thread_rng;

const TOTAL_NUMBER_OF_ITEMS: i32 = 100;

struct Layer {

}

struct Attributes {

}

#[derive(Clone)]
struct NFT {
    num: i32,
    letter: String,
}

struct Properties {
}

fn create_attribute_dist_vector() -> Vec<String> {
    let mut properties = HashMap::new();
    properties.insert("red", 10);
    properties.insert("green", 30);
    properties.insert("blue", 60);

    let mut total = vec![]; 
    let first = vec!["a".to_string(); 10];
    let second = vec!["b".to_string(); 30];
    let third = vec!["c".to_string(); 60];

    total.extend(first);
    total.extend(second);
    total.extend(third);
    total.shuffle(&mut thread_rng());
    total
}

fn main() {
    let mut nfts : Vec<NFT> = vec![];

    let mut rng = thread_rng();
    let roller = Uniform::from(1..TOTAL_NUMBER_OF_ITEMS);
    let total = create_attribute_dist_vector();

    for (i, letter) in total.iter().enumerate() {
        let throw = roller.sample(&mut rng);
        let nft = NFT { num: throw, letter: letter.clone() };
        nfts.push(nft);
    }

    let mut nft_only_c : Vec<NFT> = vec![];
    for nft in nfts {
        if nft.letter == "c" {
            nft_only_c.push(nft.clone());
        }
        println!("nft: {} {}", nft.num, nft.letter);
    }
    
}
