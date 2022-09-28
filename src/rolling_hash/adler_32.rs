//STRUCT FOR ADLER CHECKSUM
#[derive(Debug)]
pub struct Adler32 {
    pub bytes: Vec<u8>,
    pub count: u16,
    pub last_out: u8,
    a: u16,
    b: u16,
}

//THE LARGEST PRIME NUMBER SMALLER THAN 2^16
const MODULO: u16 = 65521;

impl Adler32 {
    //NEW CHECKSUM
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            count: 0,
            last_out: 0,
            a: 0,
            b: 0,
        }
    }
    //WRITE VALUES IN A AND B FOR CHUNK OF DATA
    pub fn write(&mut self, data: &[u8]) -> &mut Self {
        for (index, char) in data.iter().enumerate() {
            self.a += u16::from(*char);
            self.b += (data.len() - index) as u16 * u16::from(*char);
            self.count += 1;
        }

        self.a %= MODULO;
        self.b %= MODULO;

        self
    }

    //CALCULATE WEAK HASH WITH FORMULA a + 2^16 * b
    pub fn sum(&self) -> u32 {
        let base: u32 = 2;
        u32::from(self.a) + base.pow(16) * u32::from(self.b)
    }

    //ADD ANOTHER BYTE INTO THE CHUNK
    pub fn roll_in(&mut self, input: &u8) -> &mut Self {
        self.a = (self.a + u16::from(*input)) % MODULO;
        self.b = (self.a + self.b) % MODULO;
        self.bytes.push(*input);
        self.count += 1;

        self
    }

    //REMOVE THE FIRST BYTE FROM THE CHUNK, ADD ONE AT THE END AND RECALCULATE A AND B
    pub fn roll_out(&mut self) -> &mut Self {
        if self.bytes.len() == 0 {
            self.count = 0;
            return self;
        }

        self.last_out = self.bytes[0];
        self.a = (self.a - u16::from(self.last_out)) % MODULO;
        self.b = (self.b - (self.bytes.len() as u16 * u16::from(self.last_out))) % MODULO;
        self.bytes = self.bytes[1..].to_vec();
        self.count -= 1;

        self
    }
}
