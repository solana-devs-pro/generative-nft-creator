use crate::nft::NFT;
use crate::attributes::AttributeType;

// image stuff
use image::{GenericImageView, imageops};

mod attributes;
mod nft;

fn main() {
    let nfts = nft::get_all();

    for nft in nfts {
        overlay_images(&nft);
    }
}

fn overlay_images(nft: &NFT) {
    let bg = nft.attributes.get(&AttributeType::Background).unwrap().filename.clone();
    let face = nft.attributes.get(&AttributeType::Face).unwrap().filename.clone();
    let body = nft.attributes.get(&AttributeType::Body).unwrap().filename.clone();
    let horn = nft.attributes.get(&AttributeType::Horn).unwrap().filename.clone();

    let mut base = image::open(format!("./assets/bg/{}", bg)).unwrap();
    let face = image::open(format!("./assets/face/{}", face)).unwrap();
    let body = image::open(format!("./assets/body/{}", body)).unwrap();
    let horn = image::open(format!("./assets/horn/{}", horn)).unwrap();

    image::imageops::overlay(&mut base, &face, 0, 0);
    image::imageops::overlay(&mut base, &body, 0, 0);
    image::imageops::overlay(&mut base, &horn, 0, 0);

    base.save(format!("./output/{}.jpeg", nft.num)).unwrap();

    println!("created nft id: {}", nft.num);
    println!("\t {:?}", nft.attributes.get(&AttributeType::Background).unwrap());
    println!("\t {:?}", nft.attributes.get(&AttributeType::Body).unwrap());
    println!("\t {:?}", nft.attributes.get(&AttributeType::Face).unwrap());
    println!("\t {:?}", nft.attributes.get(&AttributeType::Horn).unwrap());
}