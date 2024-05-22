#[allow(dead_code)]
pub struct Header {
    magic: [u8; 2],
    flags: u8,
    cpu_id: u8,
    header_size: u8,
    unused: u8,
    version: u16,
    text_size: i32,
    data_size: i32,
    bss_size: i32,
    entry_point: i32,
    total_allocated: i32,
    symbol_table_size: i32,
    text_relocation_size: i32,
    data_relocation_size: i32,
    text_relocation_base: i32,
    data_relocation_base: i32,
}

const MIN_HEADER_LEN: usize = 32;

#[allow(dead_code)]
impl Header {
    fn init_short_header(bytes: [u8; 32]) -> Header {
        return Header {
            magic: [bytes[0], bytes[1]],
            flags: bytes[2],
            cpu_id: bytes[3],
            header_size: bytes[4],
            unused: bytes[5],
            version: u16::from_le_bytes([bytes[6], bytes[7]]),
            text_size: i32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            data_size: i32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            bss_size: i32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            entry_point: i32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            total_allocated: i32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            symbol_table_size: i32::from_le_bytes(bytes[28..32].try_into().unwrap()),
            text_relocation_size: 0,
            data_relocation_size: 0,
            text_relocation_base: 0,
            data_relocation_base: 0,
        };
    }

    fn is_magic_correct(&self) -> bool {
        return self.magic[0] == 0x01 && self.magic[1] == 0x03;
    }

    fn has_relocation(&self) -> bool {
        return usize::from(self.header_size) > MIN_HEADER_LEN;
    }

    fn has_external_symbol(&self) -> bool {
        return usize::from(self.header_size) > MIN_HEADER_LEN + 8;
    }

    fn has_line_number_entries(&self) -> bool {
        return usize::from(self.header_size) > MIN_HEADER_LEN + 16;
    }

    fn has_type_offset(&self) -> bool {
        return usize::from(self.header_size) > MIN_HEADER_LEN + 24;
    }

    pub fn get_text_position(&self) -> usize {
        return usize::from(self.header_size);
    }

    pub fn get_text_size(&self) -> i32 {
        return self.text_size;
    }

    // TODO: replace with a proper error when a problem arise
    pub fn init(bytes: &Vec<u8>) -> Result<Header, &'static str> {
        if bytes.len() < MIN_HEADER_LEN {
            return Err("bytes content is not big enough to contain a valid header");
        }
        if bytes.len() % 8 == 0 {
            return Err("bytes length is not a multiple of 8");
        }
        let mut header = Header::init_short_header(<[u8; 32]>::try_from(&bytes[0..32]).unwrap());
        if !header.is_magic_correct() {
            return Err("magic number provided by the bytes' header is not correct");
        }
        if !header.has_relocation() {
            return Ok(header);
        }
        header.text_relocation_size = i32::from_le_bytes(bytes[32..36].try_into().unwrap());
        header.data_relocation_size = i32::from_le_bytes(bytes[36..40].try_into().unwrap());
        if !header.has_external_symbol() {
            return Ok(header);
        }
        header.text_relocation_base = i32::from_le_bytes(bytes[40..44].try_into().unwrap());
        header.data_relocation_size = i32::from_le_bytes(bytes[44..48].try_into().unwrap());
        return Ok(header);
    }
}
