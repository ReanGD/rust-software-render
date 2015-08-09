use std;
use cgmath::*;
use mesh::Mesh;
use std::fs::File;
use std::io::{Read, BufReader, BufRead, Seek, SeekFrom};

const CHUNK_MAIN:         u16 = 0x4D4D;
const CHUNK_VERSION:      u16 = 0x0002;
const CHUNK_OBJMESH:      u16 = 0x3D3D;
const CHUNK_OBJBLOCK:     u16 = 0x4000;
const CHUNK_TRIMESH:      u16 = 0x4100;
const CHUNK_VERTLIST:     u16 = 0x4110;
const CHUNK_FACELIST:     u16 = 0x4120;
const CHUNK_FACEMAT:      u16 = 0x4130;
const CHUNK_MAPLIST:      u16 = 0x4140;
const CHUNK_TRMATRIX:     u16 = 0x4160;
const CHUNK_CAMERA:       u16 = 0x4700;
const CHUNK_MATERIAL:     u16 = 0xAFFF;
// const CHUNK_MATNAME:      u16 = 0xA000;
// const CHUNK_TEXTURE:      u16 = 0xA200;
// const CHUNK_MAPFILE:      u16 = 0xA300;
// const CHUNK_KEYFRAMER:    u16 = 0xB000;
// const CHUNK_TRACKINFO:    u16 = 0xB002;
// const CHUNK_TRACKOBJNAME: u16 = 0xB010;
// const CHUNK_TRACKPIVOT:   u16 = 0xB013;
// const CHUNK_TRACKPOS:     u16 = 0xB020;
// const CHUNK_TRACKROTATE:  u16 = 0xB021;
// const CHUNK_TRACKCAMERA:  u16 = 0xB003;
// const CHUNK_TRACKFOV:     u16 = 0xB023;
// const CHUNK_TRACKROLL:    u16 = 0xB024;
// const CHUNK_TRACKCAMTGT:  u16 = 0xB004;

pub struct Loader3ds {
    reader: BufReader<File>,
}

struct Header3ds {
    id: u16,
    size: u32,
}

#[allow(dead_code)]
fn typed_to_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const u8,
                                   slice.len() * std::mem::size_of::<T>())
    }
}

fn bytes_to_typed<T>(slice: &mut [u8]) -> &mut [T] {
    unsafe {
        std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T,
                                       slice.len() / std::mem::size_of::<T>())
    }
}

impl Loader3ds {
    fn read_u16(&mut self) -> u16 {
        let mut buff = [0xFF, 0xFF];
        self.reader.read(&mut buff).unwrap();
        (buff[0] as u16) + ((buff[1] as u16) << 8)
    }

    fn read_u32(&mut self) -> u32 {
        let mut buff = [0xFF, 0xFF, 0xFF, 0xFF];
        self.reader.read(&mut buff).unwrap();
        (buff[0] as u32) + ((buff[1] as u32) << 8) + ((buff[2] as u32) << 16) + ((buff[3] as u32) << 24)
    }

    fn read_header(&mut self) -> Header3ds {
        let id = self.read_u16();
        let size = self.read_u32();
        Header3ds {
            id: id,
            size: size,
        }
    }

    fn read_vector3(&mut self) -> Vector3<f32> {
        let mut buff = [0xFF; 4*3];
        self.reader.read(&mut buff).unwrap();
        let r = bytes_to_typed::<f32>(&mut buff);
        Vector3::new(r[0], r[2], r[1])
    }

    fn read_children(&mut self, header: &Header3ds) -> Result<u32, &str> {
        let mut left_bytes = header.size - 2 - 4;
        while left_bytes > 0 {
            left_bytes -= self.read_chunk().ok().expect("error read_chunk");
            if left_bytes < 0 {
                return Err("error length");
            }
        }
        Ok(header.size)
    }

    fn skip_chunk(&mut self, header: &Header3ds) -> Result<u32, &str> {
        self.reader.seek(SeekFrom::Current(header.size as i64 - 4 - 2))
            .ok()
            .expect("seek file error");
        Ok(header.size)
    }

    fn read_objblok(&mut self, header: &Header3ds) -> Result<u32, &str> {
        let mut name = Vec::<u8>::new();
        let mut size = self.reader.read_until(0x00, &mut name)
            .ok()
            .expect("error read_until") as u32;
        let new_header = Header3ds{id: header.id, size: header.size - size};
        size += try!(self.read_children(&new_header));
        Ok(size)
    }

    fn read_vertlist(&mut self, header: &Header3ds) -> Result<u32, &str> {
        let mut size: u32 = 4 + 2;
        let num = self.read_u16();
        size += 2;
        let mut v = Vector3::<f32>::new(0.0,0.0,0.0);
        for _ in 0..num {
            v = self.read_vector3();
            size += 3*4;
        }
        if size == header.size {
            Err("error length")
        } else {
            Ok(size)
        }
    }
    fn read_facelist(&mut self, header: &Header3ds) -> Result<u32, &str> {
        let mut size: u32 = 4 + 2;
        let num = self.read_u16();
        size += 2;
        let mut indexes = Vec::<u16>::new();
        for _ in 0..num {
            for _ in 0..3 {
                indexes.push(self.read_u16());
            }
            let _ = self.read_u16();
            size += 2*4;
        }
        if size == header.size {
            Err("error length")
        } else {
            Ok(size)
        }
    }

    fn read_chunk(&mut self) -> Result<u32, &str> {
        let header = self.read_header();
        println!("id = 0x{:x}; size={};", header.id, header.size);
        match header.id {
            CHUNK_MAIN => self.read_children(&header),
            CHUNK_VERSION => self.skip_chunk(&header),
            CHUNK_OBJMESH => self.read_children(&header),
            CHUNK_OBJBLOCK => self.read_objblok(&header),
            CHUNK_TRIMESH => self.read_children(&header),
            CHUNK_VERTLIST => self.read_vertlist(&header),
            CHUNK_FACELIST => self.read_facelist(&header),
            CHUNK_FACEMAT => self.skip_chunk(&header),
            CHUNK_MAPLIST => self.skip_chunk(&header),
            CHUNK_TRMATRIX => self.skip_chunk(&header),
            CHUNK_CAMERA => self.skip_chunk(&header),
            CHUNK_MATERIAL => self.skip_chunk(&header),

            _ => Err("unknown chank id")
        }
    }

    pub fn load(filepath: &str) -> Result<Mesh, &str> {
        let f = File::open(filepath)
            .ok()
            .expect("can't open file");

        let mut this = Loader3ds {
            reader: BufReader::new(f),
        };

        this.read_chunk().unwrap();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();
        // this.read_chunk();

        let mesh = Mesh::new(24, 12);
        Ok(mesh)
    }
}
