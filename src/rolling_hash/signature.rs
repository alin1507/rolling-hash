use super::adler_32::Adler32;

//STUCT TO SAVE WEK AND STRONG HASH AND THE INDEX OF THE SIGNATURE
#[derive(Debug)]
pub struct Signatures {
    weak: u32,
    strong: String,
    index: u32,
}

impl Signatures {
    //CREATE A NEW SIGNATURE
    pub fn new(chunk_data: &[u8], index: u32) -> Self {
        let weak = Self::generate_weak(chunk_data);
        let strong = Self::generate_strong(chunk_data);

        Self {
            weak,
            strong,
            index,
        }
    }

    //GENERATE WEAK HASH
    fn generate_weak(chunk_data: &[u8]) -> u32 {
        let mut weak = Adler32::new();
        weak.write(chunk_data).sum()
    }

    //GENERATE STRONG HASH
    pub fn generate_strong(chunk_data: &[u8]) -> String {
        let digest = md5::compute(chunk_data);
        format!("{:x}", digest)
    }

    //SEE IF A MATCHING SIGNATURE IS FIND IN THE ONES FROM THE ORIGINAL FILE BASED ON WEAK AND STRONG HASH
    pub fn find(signatures: &Vec<Signatures>, weak: u32, bytes: &Vec<u8>) -> i32 {
        for sig in signatures {
            if sig.weak == weak {
                let strong = Self::generate_strong(&bytes);
                if sig.strong == strong {
                    return sig.index as i32;
                }
            }
        }

        -1
    }
}
