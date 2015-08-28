use std;
use cgmath::{Vector3, Vector2};
use mesh::{Model, Mesh, Vertex};
use import::header3ds::*;
use import::reader::Reader;
use utils::get_full_path;

pub struct Import3ds {
    reader: Reader,
    model: Model,
    mesh: Mesh,
}

impl Import3ds {
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
        if self.mesh.vertex_buffer.len() != 0 || self.mesh.index_buffer.len() != 0 {
            try!(self.mesh.calc_normal());
            let mut mesh = Mesh::new();
            std::mem::swap(&mut mesh, &mut self.mesh);
            self.model.add(mesh);
        }

        Ok(result)
    }

    fn read_vertlist(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let num = try!(self.reader.get_u16(header)) as usize;
        // println!("vertlist = {}", num);
        if num != 0 {
            let mut vb = Vec::<Vertex>::with_capacity(num);
            for v in try!(self.reader.get_f32_vec(header, num * 3)).chunks(3) {
                vb.push(Vertex{position: Vector3::new(v[0], v[2], v[1]),
                               normal: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
                               tex: Vector2::new(0.0_f32, 0.0_f32)});
            }
            self.mesh.vertex(vb);
        }

        Ok(try!(header.check_end()))
    }

    fn read_facelist(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let num = try!(self.reader.get_u16(header)) as usize;
        // println!("facelist = {}", num);
        if num != 0 {
            let mut ib = Vec::<u32>::with_capacity(num * 3);
            for i in try!(self.reader.get_u16_vec(header, num * 4)).chunks(4) {
                ib.push(i[0] as u32);
                ib.push(i[1] as u32);
                ib.push(i[2] as u32);
            }
            try!(self.mesh.index(ib));
        }
        try!(self.read_children(header));

        Ok(try!(header.check_end()))
    }

    fn read_texcoord(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let num = try!(self.reader.get_u16(header)) as usize;
        if self.mesh.vertex_buffer.len() != num {
            return Err(format!("len of vertex_buffer is {}, len of texture coordinates is {}",
                               self.mesh.vertex_buffer.len(), num));
        }

        let vb = &mut self.mesh.vertex_buffer;
        for (i, v) in try!(self.reader.get_f32_vec(header, num * 2)).chunks(2).enumerate() {
            vb[i].tex = Vector2::new(v[0], v[1]);
        }

        Ok(try!(header.check_end()))
    }

    fn read_texfile(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let size = header.left_bytes;
        let filename = try!(self.reader.read_string(header, size));
        try!(self.mesh.material.texture_from_dir(&self.reader.current_dir, &filename));

        Ok(try!(header.check_end()))
    }


    fn read_chunk(& mut self) -> Result<u32, String> {
        let mut header = try!(self.reader.get_header());
        println!("{}(0x{:x}); size={};", chunk_id_to_str(header.id), header.id, header.size);
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
            CHUNK_MAPLIST      => self.read_texcoord(&mut header),
            CHUNK_SMOOTHING    => self.skip_chunk(&mut header),
            CHUNK_TRMATRIX     => self.skip_chunk(&mut header),
            CHUNK_LIGHT        => self.skip_chunk(&mut header),
            CHUNK_SPOTLIGHT    => self.skip_chunk(&mut header),
            CHUNK_CAMERA       => self.skip_chunk(&mut header),
            CHUNK_MATERIAL     => self.read_children(&mut header),
            CHUNK_MATNAME      => self.skip_chunk(&mut header),
            CHUNK_AMBIENT      => self.skip_chunk(&mut header),
            CHUNK_DIFFUSE      => self.skip_chunk(&mut header),
            CHUNK_SPECULAR     => self.skip_chunk(&mut header),
            CHUNK_TEXTURE      => self.read_children(&mut header),
            CHUNK_BUMPMAP      => self.skip_chunk(&mut header),
            CHUNK_MAPFILE      => self.read_texfile(&mut header),
            CHUNK_MAPPARAM     => self.skip_chunk(&mut header),
            CHUNK_MAPUSCALE    => self.skip_chunk(&mut header),
            CHUNK_MAPVSCALE    => self.skip_chunk(&mut header),
            CHUNK_MAPUOFFSET   => self.skip_chunk(&mut header),
            CHUNK_MAPVOFFSET   => self.skip_chunk(&mut header),
            CHUNK_KEYFRAMER    => self.skip_chunk(&mut header),
            _ => Err(format!("unknown chank id = 0x{:x}", header.id))
        }
    }

    pub fn load(filename: &str) -> Result<Model, String> {
        let filepath = try!(get_full_path(filename));
        let mut this = Import3ds {
            reader: try!(Reader::new(&filepath)),
            model: Model::new(),
            mesh: Mesh::new(),
        };
        try!(this.read_chunk());


        Ok(this.model)
    }
}
