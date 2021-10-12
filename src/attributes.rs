use crate::nft::{Distribution, NFT, TOTAL_EDITION_SIZE};


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

pub fn get_special_nfts() -> Vec<NFT> {
    vec![
        // add your Attribute here
        // NFT {
        // }
    ]
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