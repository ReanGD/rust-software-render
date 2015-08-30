use std;
use cgmath::{Vector3, Vector2};
use mesh::{Model, Mesh, Vertex};
use import::header3ds::*;
use material::Material;
use import::reader::Reader;
use utils::get_full_path;

pub struct Model3ds {
    reader: Reader,
    model: Model,
    mesh: Mesh,
    last_material: String,
    materials: std::collections::HashMap<String, Material>,
}

impl Model3ds {
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
        try!(self.reader.skip_ascii(header));
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

    fn read_facemap(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let name = try!(self.reader.read_asciiz(header));
        if !self.mesh.material.is_empty {
            return Err("Not supported more than one material on the mesh ".to_string());
        }
        self.mesh.material = match self.materials.get_mut(&name) {
            Some(mat) => mat.clone(),
            None => return Err(format!("Not found material with name {}", name)),
        };
        try!(self.reader.skip(header));

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

    fn read_material_name(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let size = header.left_bytes;
        self.last_material = try!(self.reader.read_ascii(header, size));
        self.materials.insert(self.last_material.clone(), Material::new());

        Ok(try!(header.check_end()))
    }

    fn read_texfile(&mut self, header: &mut Header3ds) -> Result<u32, String> {
        let size = header.left_bytes;
        let filename = try!(self.reader.read_ascii(header, size));
        match self.materials.get_mut(&self.last_material) {
            Some(mat) => try!(mat.texture_from_dir(&self.reader.current_dir, &filename)),
            None => return Err(format!("Not found material with name {}", self.last_material)),
        };

        Ok(try!(header.check_end()))
    }

    fn read_tex_map(&mut self, header: &mut Header3ds, map_param: MapParam) -> Result<u32, String> {
        let val = try!(self.reader.get_f32(header));
        match map_param {
            MapParam::UScale => assert!((val - 1.0_f32).abs() < 0.00001_f32),
            MapParam::VScale => assert!((val - 1.0_f32).abs() < 0.00001_f32),
            MapParam::UOffset => assert!((val - 0.0_f32).abs() < 0.00001_f32),
            MapParam::VOffset => assert!((val - 0.0_f32).abs() < 0.00001_f32),
        };

        Ok(try!(header.check_end()))
    }


    fn read_chunk(& mut self) -> Result<u32, String> {
        let mut header = try!(self.reader.get_header());
        // println!("{}(0x{:x}); size={};", chunk_id_to_str(header.id), header.id, header.size);
        match header.id {
            CHUNK_MAIN          => self.read_children(&mut header),
            CHUNK_VERSION       => self.skip_chunk(&mut header),
            CHUNK_PERCENT       => self.skip_chunk(&mut header),
            CHUNK_OBJMESH       => self.read_children(&mut header),
            CHUNK_MESHVERSION   => self.skip_chunk(&mut header),
            CHUNK_MASTERSCALE   => self.skip_chunk(&mut header),
            CHUNK_OBJBLOCK      => self.read_objblok(&mut header),
            CHUNK_TRIMESH       => self.read_trimesh(&mut header),
            CHUNK_VERTLIST      => self.read_vertlist(&mut header),
            CHUNK_FACELIST      => self.read_facelist(&mut header),
            CHUNK_FACEMAT       => self.read_facemap(&mut header),
            CHUNK_MAPLIST       => self.read_texcoord(&mut header),
            CHUNK_SMOOTHING     => self.skip_chunk(&mut header),
            CHUNK_TRMATRIX      => self.skip_chunk(&mut header),
            CHUNK_LIGHT         => self.skip_chunk(&mut header),
            CHUNK_SPOTLIGHT     => self.skip_chunk(&mut header),
            CHUNK_CAMERA        => self.skip_chunk(&mut header),
            CHUNK_MATERIAL      => self.read_children(&mut header),
            CHUNK_MATNAME       => self.read_material_name(&mut header),
            CHUNK_AMBIENT       => self.skip_chunk(&mut header),
            CHUNK_DIFFUSE       => self.skip_chunk(&mut header),
            CHUNK_SPECULAR      => self.skip_chunk(&mut header),
            CHUNK_SHININESS1    => self.skip_chunk(&mut header),
            CHUNK_SHININESS2    => self.skip_chunk(&mut header),
            CHUNK_TRANSPARENCY1 => self.skip_chunk(&mut header),
            CHUNK_TRANSPARENCY2 => self.skip_chunk(&mut header),
            CHUNK_REFLECTION    => self.skip_chunk(&mut header),
            CHUNK_2_SIDED       => self.skip_chunk(&mut header),
            CHUNK_SELF_ILLUM    => self.skip_chunk(&mut header),
            CHUNK_WIRE_THICKN   => self.skip_chunk(&mut header),
            CHUNK_IN_TRANC      => self.skip_chunk(&mut header),
            CHUNK_TEXTURE       => self.read_children(&mut header),
            CHUNK_REFLECT       => self.read_children(&mut header),
            CHUNK_RENDER_TYPE   => self.skip_chunk(&mut header),
            CHUNK_BUMPMAP       => self.skip_chunk(&mut header),
            CHUNK_MAPFILE       => self.read_texfile(&mut header),
            CHUNK_MAPPARAM      => self.skip_chunk(&mut header),
            CHUNK_BLUR          => self.skip_chunk(&mut header),
            CHUNK_MAPUSCALE     => self.read_tex_map(&mut header, MapParam::UScale),
            CHUNK_MAPVSCALE     => self.read_tex_map(&mut header, MapParam::VScale),
            CHUNK_MAPUOFFSET    => self.read_tex_map(&mut header, MapParam::UOffset),
            CHUNK_MAPVOFFSET    => self.read_tex_map(&mut header, MapParam::VOffset),
            CHUNK_KEYFRAMER     => self.skip_chunk(&mut header),
            _ => Err(format!("unknown chank id = 0x{:x}", header.id))
        }
    }

    pub fn load(filename: &str) -> Result<Model, String> {
        let filepath = try!(get_full_path(filename));
        let mut this = Model3ds {
            reader: try!(Reader::new(&filepath)),
            model: Model::new(),
            mesh: Mesh::new(),
            last_material: String::new(),
            materials: std::collections::HashMap::new(),
        };
        try!(this.read_chunk());


        Ok(this.model)
    }
}
