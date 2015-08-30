pub const CHUNK_MAIN:         u16 = 0x4D4D;
pub const CHUNK_VERSION:      u16 = 0x0002;
pub const CHUNK_PERCENT:      u16 = 0x0030;
pub const CHUNK_OBJMESH:      u16 = 0x3D3D;
pub const CHUNK_MESHVERSION:  u16 = 0x3D3E;
pub const CHUNK_MASTERSCALE:  u16 = 0x0100;
pub const CHUNK_OBJBLOCK:     u16 = 0x4000;
pub const CHUNK_TRIMESH:      u16 = 0x4100;
pub const CHUNK_VERTLIST:     u16 = 0x4110;
pub const CHUNK_FACELIST:     u16 = 0x4120;
pub const CHUNK_FACEMAT:      u16 = 0x4130;
pub const CHUNK_MAPLIST:      u16 = 0x4140;
pub const CHUNK_SMOOTHING:    u16 = 0x4150;
pub const CHUNK_TRMATRIX:     u16 = 0x4160;
pub const CHUNK_LIGHT:        u16 = 0x4600;
pub const CHUNK_SPOTLIGHT:    u16 = 0x4610;
pub const CHUNK_CAMERA:       u16 = 0x4700;
pub const CHUNK_MATERIAL:     u16 = 0xAFFF;
pub const CHUNK_MATNAME:      u16 = 0xA000;
pub const CHUNK_AMBIENT:      u16 = 0xA010;
pub const CHUNK_DIFFUSE:      u16 = 0xA020;
pub const CHUNK_SPECULAR:     u16 = 0xA030;
pub const CHUNK_SHININESS1:   u16 = 0xA040;
pub const CHUNK_SHININESS2:   u16 = 0xA041;
pub const CHUNK_TRANSPARENCY1:u16 = 0xA050;
pub const CHUNK_TRANSPARENCY2:u16 = 0xA052;
pub const CHUNK_REFLECTION:   u16 = 0xA053;
pub const CHUNK_2_SIDED:      u16 = 0xA081;
pub const CHUNK_SELF_ILLUM:   u16 = 0xA084;
pub const CHUNK_WIRE_THICKN:  u16 = 0xA087;
pub const CHUNK_IN_TRANC:     u16 = 0xA08A;
pub const CHUNK_RENDER_TYPE:  u16 = 0xA100;
pub const CHUNK_TEXTURE:      u16 = 0xA200;
pub const CHUNK_REFLECT:      u16 = 0xA220;
pub const CHUNK_BUMPMAP:      u16 = 0xA230;
pub const CHUNK_MAPFILE:      u16 = 0xA300;
pub const CHUNK_MAPPARAM:     u16 = 0xA351;
pub const CHUNK_BLUR:         u16 = 0xA353;
pub const CHUNK_MAPUSCALE:    u16 = 0xA354;
pub const CHUNK_MAPVSCALE:    u16 = 0xA356;
pub const CHUNK_MAPUOFFSET:   u16 = 0xA358;
pub const CHUNK_MAPVOFFSET:   u16 = 0xA35A;
pub const CHUNK_KEYFRAMER:    u16 = 0xB000;

pub enum MapParam {
    UScale,
    VScale,
    UOffset,
    VOffset,
}

pub fn chunk_id_to_str(chunk_id: u16) -> String {
    match chunk_id {
        0x4D4D => "CHUNK_MAIN",
        0x0002 => "CHUNK_VERSION",
        0x0030 => "CHUNK_PERCENT",
        0x3D3D => "CHUNK_OBJMESH",
        0x3D3E => "CHUNK_MESHVERSION",
        0x0100 => "CHUNK_MASTERSCALE",
        0x4000 => "CHUNK_OBJBLOCK",
        0x4100 => "CHUNK_TRIMESH",
        0x4110 => "CHUNK_VERTLIST",
        0x4120 => "CHUNK_FACELIST",
        0x4130 => "CHUNK_FACEMAT",
        0x4140 => "CHUNK_MAPLIST",
        0x4150 => "CHUNK_SMOOTHING",
        0x4160 => "CHUNK_TRMATRIX",
        0x4600 => "CHUNK_LIGHT",
        0x4610 => "CHUNK_SPOTLIGHT",
        0x4700 => "CHUNK_CAMERA",
        0xAFFF => "CHUNK_MATERIAL",
        0xA000 => "CHUNK_MATNAME",
        0xA010 => "CHUNK_AMBIENT",
        0xA020 => "CHUNK_DIFFUSE",
        0xA030 => "CHUNK_SPECULAR",
        0xA040 => "CHUNK_SHININESS1",
        0xA041 => "CHUNK_SHININESS2",
        0xA050 => "CHUNK_TRANSPARENCY1",
        0xA052 => "CHUNK_TRANSPARENCY2",
        0xA053 => "CHUNK_REFLECTION",
        0xA081 => "CHUNK_2_SIDED",
        0xA084 => "CHUNK_SELF_ILLUM",
        0xA087 => "CHUNK_WIRE_THICKN",
        0xA08A => "CHUNK_IN_TRANC",
        0xA100 => "CHUNK_RENDER_TYPE",
        0xA200 => "CHUNK_TEXTURE",
        0xA220 => "CHUNK_REFLECT",
        0xA230 => "CHUNK_BUMPMAP",
        0xA300 => "CHUNK_MAPFILE",
        0xA351 => "CHUNK_MAPPARAM",
        0xA353 => "CHUNK_BLUR",
        0xA354 => "CHUNK_MAPUSCALE",
        0xA356 => "CHUNK_MAPVSCALE",
        0xA358 => "CHUNK_MAPUOFFSET",
        0xA35A => "CHUNK_MAPVOFFSET",
        0xB000 => "CHUNK_KEYFRAMER",
        _ => "CHUNK_UNKNOWN",
    }.to_string()
}


pub struct Header3ds {
    pub id: u16,
    pub size: u32,
    pub left_bytes: u32,
}

impl Header3ds {
    pub fn check_end(&self) -> Result<u32, String> {
        if self.left_bytes == 0 {
            Ok(self.size)
        } else {
            Err(format!("error length chunk in end for id = 0x{:x} left {} bytes",
                        self.id, self.left_bytes))
        }
    }

    pub fn update_left(&mut self, read_bytes: u32) -> Result<(), String> {
        if self.left_bytes < read_bytes {
            return Err(format!("for chunk id = 0x{:x}, real read {} bytes, left {} bytes",
                               self.id, read_bytes, self.left_bytes));
        } else {
            self.left_bytes -= read_bytes;
            Ok(())
        }
    }
}
