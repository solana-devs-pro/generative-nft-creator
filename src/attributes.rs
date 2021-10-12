use rand::thread_rng;
use rand::seq::SliceRandom;

pub const TOTAL_NUMBER_OF_ITEMS: usize = 88;

pub struct Distribution {
    name: String,
    total_number: usize,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    name: String,
    attr_type: AttrType,
    rarity: f32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum AttrType {
    Background,
    Body,
    Face,
}

fn body_types() -> Vec<Distribution> {
    vec![
        Distribution { name: "yellow".to_string(), total_number: 5, },
        Distribution { name: "green".to_string(), total_number: 20, },
        Distribution { name: "normal".to_string(), total_number: 63, },
    ]
}

fn background_types() -> Vec<Distribution> {
    vec![
        Distribution { name: "super_rare".to_string(), total_number: 8, },
        Distribution { name: "black".to_string(), total_number: 40, },
        Distribution { name: "turquoise".to_string(), total_number: 40, },
    ]
}

fn face_types() -> Vec<Distribution> {
    vec![
        Distribution { name: "round".to_string(), total_number: 23, },
        Distribution { name: "square".to_string(), total_number: 27, },
        Distribution { name: "oblong".to_string(), total_number: 38, },
    ]
}

pub fn get_all(attr_type: AttrType) -> Vec<Attribute> {
    let mut return_types = vec![];

    let dists = match attr_type {
        AttrType::Background => background_types(),
        AttrType::Body => body_types(),
        AttrType::Face => face_types(),
    };

    for dist in dists {
        // need to cast to f32
        let num_of_attrs: f32 = dist.total_number as f32;
        let total_num: f32 = TOTAL_NUMBER_OF_ITEMS as f32; 
        // setup attribute
        let attribute = Attribute { 
            name: dist.name,
            attr_type: attr_type.clone(),
            rarity: num_of_attrs / total_num,
        };
        return_types.extend(vec![attribute; dist.total_number]);
    }
    // make sure to shuffle before returning
    // https://docs.rs/rand/0.6.4/rand/seq/trait.SliceRandom.html
    return_types.shuffle(&mut thread_rng());
    return_types
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
        assert_eq!(num, TOTAL_NUMBER_OF_ITEMS);
    }

    #[test]
    fn test_face_type_total_num() {
        let dists = face_types();
        let mut num = 0;
        for dist in dists {
            num += dist.total_number;
        }
        assert_eq!(num, TOTAL_NUMBER_OF_ITEMS);
    }

    #[test]
    fn test_bg_type_total_num() {
        let dists = background_types();
        let mut num = 0;
        for dist in dists {
            num += dist.total_number;
        }
        assert_eq!(num, TOTAL_NUMBER_OF_ITEMS);
    }
}