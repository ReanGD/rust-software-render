use std::mem::swap;
use cgmath::*;
use mesh::{Model, Mesh, Vertex};
use memory::bytes_to_typed;
use std::fs;
use utils::get_full_path;
use std::io::{Read, Seek, SeekFrom};

const CHUNK_MAIN:         u16 = 0x4D4D;
const CHUNK_VERSION:      u16 = 0x0002;
const CHUNK_OBJMESH:      u16 = 0x3D3D;
const CHUNK_MESHVERSION:  u16 = 0x3D3E;
const CHUNK_MASTERSCALE:  u16 = 0x0100;
const CHUNK_OBJBLOCK:     u16 = 0x4000;
const CHUNK_TRIMESH:      u16 = 0x4100;
const CHUNK_VERTLIST:     u16 = 0x4110;
const CHUNK_FACELIST:     u16 = 0x4120;
const CHUNK_FACEMAT:      u16 = 0x4130;
const CHUNK_MAPLIST:      u16 = 0x4140;
const CHUNK_SMOOTHING:    u16 = 0x4150;
const CHUNK_TRMATRIX:     u16 = 0x4160;
const CHUNK_LIGHT:        u16 = 0x4600;
const CHUNK_SPOTLIGHT:    u16 = 0x4610;
const CHUNK_CAMERA:       u16 = 0x4700;
const CHUNK_MATERIAL:     u16 = 0xAFFF;
const CHUNK_MATNAME:      u16 = 0xA000;
const CHUNK_TEXTURE:      u16 = 0xA200;
const CHUNK_MAPFILE:      u16 = 0xA300;
const CHUNK_KEYFRAMER:    u16 = 0xB000;

struct Reader {
    reader: fs::File
}

pub struct Loader3ds {
    reader: Reader,
    model: Model,
    mesh: Mesh,
}

struct Header3ds {
    id: u16,
    size: u32,
    left_bytes: u32,
}

impl Header3ds {
    fn check_end(&self) -> Result<u32, String> {
        if self.left_bytes == 0 {
            Ok(self.size)
        } else {
            Err(format!("error length chunk in end for id = 0x{:x} left {} bytes", self.id, self.left_bytes))
        }
    }

    fn update_left(&mut self, read_bytes: u32) -> Result<(), String> {
        if self.left_bytes < read_bytes {
            return Err(format!("for chunk id = 0x{:x}, real read {} bytes, left {} bytes", self.id, read_bytes, self.left_bytes));
        } else {
            self.left_bytes -= read_bytes;
            Ok(())
        }
    }
}

impl Reader {
    fn new(filepath: &str) -> Result<Reader, String> {
        let f = match fs::File::open(filepath) {
            Ok(f) => f,
            Err(e) => return Err(format!("Can't open file \"{}\" with error: \"{}\"", filepath, e))
        };
        Ok(Reader {reader: f})
    }

    #[allow(dead_code)]
    fn get_u16(&mut self, header: &mut Header3ds) -> Result<u16, String> {
        try!(header.update_left(2));
        let mut buff = [0x0; 2];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok((buff[0] as u16) + ((buff[1] as u16) << 8)),
            Err(e) => Err(format!("can't read 2 bytes, err = \"{}\"", e))
        }
    }

    fn get_u16_vec(&mut self, header: &mut Header3ds, num: usize) -> Result<Vec<u16>, String> {
        try!(header.update_left(2*num as u32));
        let mut buff: Vec<u8> = vec![0x0; 2*num];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok(bytes_to_typed::<u16>(&mut buff).iter().cloned().collect()),
            Err(e) => Err(format!("can't read {} bytes, err = \"{}\"", 2*num, e))
        }
    }

    #[allow(dead_code)]
    fn get_u32(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        try!(header.update_left(4));
        let mut buff = [0x0; 4];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok((buff[0] as u32) + ((buff[1] as u32) << 8) + ((buff[2] as u32) << 16) + ((buff[3] as u32) << 24)),
            Err(e) => Err(format!("can't read 4 bytes, err = \"{}\"", e))
        }
    }

    fn get_f32_vec(&mut self, header: &mut Header3ds, num: usize) -> Result<Vec<f32>, String> {
        try!(header.update_left(4*num as u32));
        let mut buff: Vec<u8> = vec![0x0; 4*num];
        match self.reader.read(&mut buff) {
            Ok(_) => Ok(bytes_to_typed::<f32>(&mut buff).iter().cloned().collect()),
            Err(e) => Err(format!("can't read {} bytes, err = \"{}\"", 4*num, e))
        }
    }

    fn skip_string(&mut self, header: &mut Header3ds) -> Result<u32, String> {
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

    fn skip(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        match self.reader.seek(SeekFrom::Current(header.left_bytes as i64)) {
            Ok(_) => {header.left_bytes = 0; return Ok(header.size)},
            Err(e) => return Err(format!("seek file error \"{}\"for chunk id = 0x{:x}", e, header.id))
        };
    }

    fn get_header(&mut self) -> Result<Header3ds, String> {
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

impl Loader3ds {
    fn read_children(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        while header.left_bytes != 0 {
            try!(header.update_left(try!(self.read_chunk())));
        }
        Ok(try!(header.check_end()))
    }

    fn skip_chunk(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        try!(self.reader.skip(header));
        Ok(try!(header.check_end()))
    }

    fn read_objblok(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        try!(self.reader.skip_string(header));
        self.read_children(header)
    }

    fn read_trimesh(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let result = try!(self.read_children(header));
        try!(self.mesh.calc_normal());
        let mut mesh = Mesh::new();
        swap(&mut mesh, &mut self.mesh);
        self.model.add(mesh);

        Ok(result)
    }

    fn read_vertlist(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let num = try!(self.reader.get_u16(header)) as usize;
        let mut vb = Vec::<Vertex>::with_capacity(num);
        for v in try!(self.reader.get_f32_vec(header, num * 3)).chunks(3) {
            vb.push(Vertex{position: Vector3::new(v[0], v[2], v[1]),
                           normal: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
                           tex: Vector2::new(0.0_f32, 0.0_f32)});
        }
        self.mesh.vertex(vb);

        Ok(try!(header.check_end()))
    }

    fn read_facelist(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let num = try!(self.reader.get_u16(header)) as usize;
        let mut ib = Vec::<u32>::with_capacity(num * 3);
        for i in try!(self.reader.get_u16_vec(header, num * 4)).chunks(4) {
            ib.push(i[0] as u32);
            ib.push(i[1] as u32);
            ib.push(i[2] as u32);
        }
        try!(self.mesh.index(ib));
        try!(self.read_children(header));

        Ok(try!(header.check_end()))
    }

    fn read_chunk(& mut self) -> Result<u32, String> {
        let mut header = try!(self.reader.get_header());
        // println!("id = 0x{:x}; size={};", header.id, header.size);
        match header.id {
            CHUNK_MAIN         => self.read_children(&mut header),
            CHUNK_VERSION      => self.skip_chunk(&mut header),
            CHUNK_OBJMESH      => self.read_children(&mut header),
            CHUNK_MESHVERSION  => self.skip_chunk(&mut header),
            CHUNK_MASTERSCALE  => self.skip_chunk(&mut header),
            CHUNK_OBJBLOCK     => self.read_objblok(&mut header),
            CHUNK_TRIMESH      => self.read_trimesh(&mut header),
            CHUNK_VERTLIST     => self.read_vertlist(&mut header),
            CHUNK_FACELIST     => self.read_facelist(&mut header),
            CHUNK_FACEMAT      => self.skip_chunk(&mut header),
            CHUNK_MAPLIST      => self.skip_chunk(&mut header),
            CHUNK_SMOOTHING    => self.skip_chunk(&mut header),
            CHUNK_TRMATRIX     => self.skip_chunk(&mut header),
            CHUNK_LIGHT        => self.skip_chunk(&mut header),
            CHUNK_SPOTLIGHT    => self.skip_chunk(&mut header),
            CHUNK_CAMERA       => self.skip_chunk(&mut header),
            CHUNK_MATERIAL     => self.skip_chunk(&mut header),
            CHUNK_MATNAME      => self.skip_chunk(&mut header),
            CHUNK_TEXTURE      => self.skip_chunk(&mut header),
            CHUNK_MAPFILE      => self.skip_chunk(&mut header),
            CHUNK_KEYFRAMER    => self.skip_chunk(&mut header),
            _ => Err(format!("unknown chank id = 0x{:x}", header.id))
        }
    }

    pub fn load(filename: &str) -> Result<Model, String> {
        let filepath = try!(get_full_path(filename));
        let mut this = Loader3ds {
            reader: try!(Reader::new(&filepath)),
            model: Model::new(),
            mesh: Mesh::new(),
        };
        try!(this.read_chunk());
        

        Ok(this.model)
    }
}
