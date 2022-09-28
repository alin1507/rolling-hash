use std::str::{self, Utf8Error};

use super::{adler_32::Adler32, signature::Signatures};

pub const CHUNK_SIZE: u32 = 16;

//STRUCT TO SAVE SIGNATURES, DELTA AND THE TEXT FROM THE ORIGINAL FILE
pub struct Rolling {
    pub signatures: Vec<Signatures>,
    pub delta: Vec<Block>,
    pub original_text: String,
}

//STRUCT TO SAVE CHUNKS OF DATA
#[derive(Debug)]
pub struct Block {
    start: u32,
    end: u32,
    pub bytes: Vec<u8>,
}

impl Block {
    //RETURN NEW CHUNK OF DATA WITH START, END AND POSSIBLE ADDITIONAL DATA
    pub fn new(index: u32, additional_data: &[u8], last_block: bool) -> Self {
        let multiplier = match last_block {
            true => 0,
            false => 1,
        };

        Self {
            start: index * CHUNK_SIZE,
            end: index * CHUNK_SIZE + CHUNK_SIZE * multiplier,
            bytes: additional_data.to_vec(),
        }
    }
}

impl Rolling {
    //RETURN SIGNATURES, DELTA AND ORIGINAL TEXT FILE FOR TWO VERSIONS OF A FILE
    pub fn new(original_text: String, v2: &[u8]) -> Self {
        //GENERATE SIGNATURES FOR THE ORIGINAL TEXT
        let signatures = Self::generate_signatures(original_text.as_bytes());

        //CREATE NEW WEAK HASH
        let mut weak = Adler32::new();

        //VECTORS TO SAVE DELTA, EXTRA BYTES FROM DELTA AND POTENTIAL REMAINING BYTES FROM THE END OF THE FILE
        let mut delta: Vec<Block> = vec![];
        let mut extra_bytes: Vec<u8> = vec![];
        let mut remaining_bytes: Vec<u8> = vec![];

        //ITERATION BYTE BY BYTE IN THE UPDATED FILE
        for byte in v2 {
            //ADD BYTES TO WEAK AND CALCULATE A AND B
            weak.roll_in(byte);
            remaining_bytes.push(*byte);

            //IF WE HAVE LESS THAN AGREED CHUNK SIZE THERE IS NOTHING TO DO
            if weak.count < CHUNK_SIZE as u16 {
                continue;
            }

            //IF WE HAVE MORE THAN AGREED CHUNK SIZE WE NEED TO ROLL OUT
            if weak.count > CHUNK_SIZE as u16 {
                weak.roll_out();
                extra_bytes.push(weak.last_out);
            }

            //SEE IF THE CURRENT CHUNK OF DATA FROM UPDATED FILE IS FOUND IN THE ORIGINAL FILE
            let index = Signatures::find(&signatures, weak.sum(), &weak.bytes);

            //IF THE SIGNATURE WAS FOUND CREATE A NEW BLOCK, ADD IT TO DELTA AND RESET THE WEAK, EXTRA BYTES AND THE REMAINING BYTES
            if index != -1 {
                let new_block = Block::new(index as u32, &extra_bytes, false);
                delta.push(new_block);

                weak = Adler32::new();
                extra_bytes = vec![];
                remaining_bytes = vec![];
            }
        }

        //IF WE ARE AT THE LAST BYTE AND THERE ARE SOME EXTRA BYTES LEFT CREATE A BLOCK FROM THEM AND ADD THEM TO DELTA
        if remaining_bytes.len() > 0 {
            delta.push(Block::new(0, &remaining_bytes, true));
        }

        //RETURN THE ROLLING STRUCT
        Rolling {
            signatures,
            delta,
            original_text,
        }
    }

    //RETURN WEAK AND STRONG HASH FOR GIVEN DATA
    pub fn generate_signatures(bytes: &[u8]) -> Vec<Signatures> {
        let chunks = bytes.chunks(CHUNK_SIZE as usize);
        let mut signatures = vec![];

        for (index, chunk) in chunks.enumerate() {
            signatures.push(Signatures::new(chunk, index as u32));
        }

        signatures
    }

    //GENERATE TEXT FROM DELTA (FOR TESTING PURPOSES)
    pub fn generate_new_version_with_delta(&self) -> Result<String, Utf8Error> {
        let mut new_version = String::new();

        for block in &self.delta {
            let mut str_chunk =
                self.original_text[block.start as usize..block.end as usize].to_string();

            if block.bytes.len() > 0 {
                str_chunk = format!("{}{}", str::from_utf8(&block.bytes)?, str_chunk);
            }

            new_version = format!("{}{}", new_version, str_chunk);
        }

        Ok(new_version)
    }
}
