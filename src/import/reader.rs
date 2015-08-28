use std::fs::File;
use std::path::PathBuf;
use memory::bytes_to_typed;
use std::io::{Read, Seek, SeekFrom};
use import::header3ds::Header3ds;

pub struct Reader {
    reader: File,
    pub current_dir: PathBuf,
}

impl Reader {
    pub fn new(filepath: &str) -> Result<Reader, String> {
        let mut current_dir = PathBuf::from(filepath);
        if !current_dir.pop() {
            return Err(format!("not found parent dir for filepath {}", filepath));
        }

        let f = match File::open(filepath) {
            Ok(f) => f,
            Err(e) => return Err(format!("Can't open file \"{}\" with error: \"{}\"", filepath, e))
        };

        Ok(Reader {
            reader: f,
            current_dir: current_dir,
        })
    }

    #[allow(dead_code)]
    pub fn get_u16(&mut self, header: &mut Header3ds) -> Result<u16, String> {
        try!(header.update_left(2));
        let mut buff = [0x0; 2];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok((buff[0] as u16) + ((buff[1] as u16) << 8)),
            Err(e) => Err(format!("can't read 2 bytes, err = \"{}\"", e))
        }
    }

    pub fn get_u16_vec(&mut self, header: &mut Header3ds, num: usize) -> Result<Vec<u16>, String> {
        try!(header.update_left(2*num as u32));
        let mut buff: Vec<u8> = vec![0x0; 2*num];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok(bytes_to_typed::<u16>(&mut buff).iter().cloned().collect()),
            Err(e) => Err(format!("can't read {} bytes, err = \"{}\"", 2*num, e))
        }
    }

    #[allow(dead_code)]
    pub fn get_u32(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        try!(header.update_left(4));
        let mut buff = [0x0; 4];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok((buff[0] as u32) + ((buff[1] as u32) << 8) + ((buff[2] as u32) << 16) + ((buff[3] as u32) << 24)),
            Err(e) => Err(format!("can't read 4 bytes, err = \"{}\"", e))
        }
    }

    pub fn get_f32_vec(&mut self, header: &mut Header3ds, num: usize) -> Result<Vec<f32>, String> {
        try!(header.update_left(4*num as u32));
        let mut buff: Vec<u8> = vec![0x0; 4*num];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok(bytes_to_typed::<f32>(&mut buff).iter().cloned().collect()),
            Err(e) => Err(format!("can't read {} bytes, err = \"{}\"", 4*num, e))
        }
    }

    pub fn read_string(&mut self, header: &mut Header3ds, size: u32) -> Result<String, String> {
        try!(header.update_left(size));
        let mut buff: Vec<u8> = vec![0x0; size as usize];
        match self.reader.read(&mut buff) {
            Ok(_) => {},
            Err(e) => return Err(format!("can't read {} bytes, err = \"{}\"", size, e))
        };

        buff.pop().unwrap();
        match String::from_utf8(buff) {
            Ok(v) => Ok(v),
            Err(_) => Err(format!("can't read string"))
        }
    }

    pub fn skip_string(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let mut buff: Vec<u8> = vec![0xFF];
        let mut size = 0;
        while buff[0] != 0 {
            match self.reader.read(&mut buff) {
                Ok(_) => {},
                Err(e) => return Err(format!("can't read 1 bytes, err = \"{}\"", e))
            };
            try!(header.update_left(1));
            size += 1;
        }
        Ok(size)
    }

    pub fn skip(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        match self.reader.seek(SeekFrom::Current(header.left_bytes as i64)) {
            Ok(_) => {header.left_bytes = 0; return Ok(header.size)},
            Err(e) => return Err(format!("seek file error \"{}\"for chunk id = 0x{:x}", e, header.id))
        };
    }

    pub fn get_header(&mut self) -> Result<Header3ds, String> {
        let id;
        let size;
        let mut buff = [0x0; 6];
        match self.reader.read(&mut buff) {
            Ok(_) => {
                id = (buff[0] as u16) + ((buff[1] as u16) << 8);
                size = (buff[2] as u32) + ((buff[3] as u32) << 8) + ((buff[4] as u32) << 16) + ((buff[5] as u32) << 24);
            },
            Err(e) => return Err(format!("can't read 6 bytes, err = \"{}\"", e))
        };

        if size < 6 {
            return Err(format!("for chunk id = 0x{:x}, header real size is 6 bytes, but in header is {} bytes", id, size));
        }

        Ok(Header3ds {
            id: id,
            size: size,
            left_bytes: size - 6,
        })
    }
}
