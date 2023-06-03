//const VERSION: &str = env!("CARGO_PKG_VERSION");
//println!("{}", last_git_commit::LastGitCommit::new().build().unwrap().id().long());
//println!("{}", VERSION);
use std::io::{BufWriter, Read, Seek};
use std::fs::File;
use std::fmt;
use std::io::Cursor;
use std::mem;
use bytebuffer::ByteBuffer;

#[derive(Clone, Copy)]
pub struct MdlHeader {
    pub id: i32,
    pub version: i32,
    pub name: [u8; 64],
    pub data_length: i32,
    pub flags: i32,
    pub num_bones: i32,
    pub bone_index: i32,
    pub num_bone_controllers: i32,
    pub bone_controller_index: i32,
    pub num_hit_boxes: i32,
    pub hit_box_index: i32,
    pub num_seq: i32,
    pub seq_index: i32,
    pub num_seq_groups: i32,
    pub seq_group_index: i32,

    pub num_textures: i32,
    pub texture_index: i32,
    pub texture_data_index: i32,
    pub num_skin_ref: i32,
    pub num_skin_families: i32,
    pub skin_index: i32,
}

impl MdlHeader {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let mut reader = ByteBuffer::from(buf);
        reader.set_endian(bytebuffer::Endian::LittleEndian);
        let id = reader.read_i32().expect("Failed to get value");
        let version = reader.read_i32().expect("Failed to get value");
        let name = reader.read_bytes(64).expect("Failed to get value").try_into().expect("failed to convert value");
        let data_length = reader.read_i32().expect("Failed to get value");

        let _ = reader.read_bytes(60);

        let flags = reader.read_i32().expect("Failed to get value");

        let num_bones = reader.read_i32().expect("Failed to get value");
        let bone_index = reader.read_i32().expect("Failed to get value");

        let num_bone_controllers = reader.read_i32().expect("Failed to get value");
        let bone_controller_index = reader.read_i32().expect("Failed to get value");

        let num_hit_boxes = reader.read_i32().expect("Failed to get value");
        let hit_box_index = reader.read_i32().expect("Failed to get value");
        
        let num_seq = reader.read_i32().expect("Failed to get value");
        let seq_index = reader.read_i32().expect("Failed to get value");

        let num_seq_groups = reader.read_i32().expect("Failed to get value");
        let seq_group_index = reader.read_i32().expect("Failed to get value");

        let num_textures = reader.read_i32().expect("Failed to get value");
        let texture_index = reader.read_i32().expect("Failed to get value");
        let texture_data_index = reader.read_i32().expect("Failed to get value");

        let num_skin_ref = reader.read_i32().expect("Failed to get value");
        let num_skin_families = reader.read_i32().expect("Failed to get value");
        let skin_index = reader.read_i32().expect("Failed to get value");

        Self {
            id,
            version,
            name,
            data_length,
            flags,

            num_bones,
            bone_index,
            num_bone_controllers,
            bone_controller_index,
            num_hit_boxes,
            hit_box_index,
            num_seq,
            seq_index,
            num_seq_groups,
            seq_group_index,
            num_textures,
            texture_index,
            texture_data_index,
            num_skin_ref,
            num_skin_families,
            skin_index,
        }
    }
    pub fn from_reader(reader: &mut ByteBuffer) -> Self {
        reader.set_endian(bytebuffer::Endian::LittleEndian);
        let id = reader.read_i32().expect("Failed to get value");
        let version = reader.read_i32().expect("Failed to get value");
        let name = reader.read_bytes(64).expect("Failed to get value").try_into().expect("failed to convert value");
        let data_length = reader.read_i32().expect("Failed to get value");

        let _ = reader.read_bytes(60);

        let flags = reader.read_i32().expect("Failed to get value");

        let num_bones = reader.read_i32().expect("Failed to get value");
        let bone_index = reader.read_i32().expect("Failed to get value");

        let num_bone_controllers = reader.read_i32().expect("Failed to get value");
        let bone_controller_index = reader.read_i32().expect("Failed to get value");

        let num_hit_boxes = reader.read_i32().expect("Failed to get value");
        let hit_box_index = reader.read_i32().expect("Failed to get value");
        
        let num_seq = reader.read_i32().expect("Failed to get value");
        let seq_index = reader.read_i32().expect("Failed to get value");

        let num_seq_groups = reader.read_i32().expect("Failed to get value");
        let seq_group_index = reader.read_i32().expect("Failed to get value");

        let num_textures = reader.read_i32().expect("Failed to get value");
        let texture_index = reader.read_i32().expect("Failed to get value");
        let texture_data_index = reader.read_i32().expect("Failed to get value");

        let num_skin_ref = reader.read_i32().expect("Failed to get value");
        let num_skin_families = reader.read_i32().expect("Failed to get value");
        let skin_index = reader.read_i32().expect("Failed to get value");

        Self {
            id,
            version,
            name,
            data_length,
            flags,

            num_bones,
            bone_index,
            num_bone_controllers,
            bone_controller_index,
            num_hit_boxes,
            hit_box_index,
            num_seq,
            seq_index,
            num_seq_groups,
            seq_group_index,
            num_textures,
            texture_index,
            texture_data_index,
            num_skin_ref,
            num_skin_families,
            skin_index,
        }
    }
}

impl fmt::Debug for MdlHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
         .field("\n\t id", &std::str::from_utf8(&self.id.to_le_bytes()).unwrap())
         .field("\n\t version", &self.version)
         .field("\n\t name", &std::str::from_utf8(&self.name).unwrap())
         .field("\n\t data_length", &self.data_length)
         .field("\n\t flags", &self.flags)
         .field("\n\t num_bones", &self.num_bones)
         .field("\n\t bones_index", &self.bone_index)
         .field("\n\t num_bone_controllers", &self.num_bone_controllers)
         .field("\n\t bone_controller_index", &self.bone_controller_index)
         .field("\n\t num_hit_boxes", &self.num_hit_boxes)
         .field("\n\t hit_box_index", &self.hit_box_index)
         .field("\n\t num_seq", &self.num_seq)
         .field("\n\t seq_index", &self.seq_index)
         .field("\n\t num_seq_groups", &self.num_seq_groups)
         .field("\n\t seq_group_index", &self.seq_group_index)
         .field("\n\t num_textures", &self.num_textures)
         .field("\n\t texture_index", &self.texture_index)
         .field("\n\t texture_data_index", &self.texture_data_index)
         .field("\n\t num_skin_ref", &self.num_skin_ref)
         .field("\n\t num_skin_families", &self.num_seq_groups)
         .field("\n\t skin_index", &self.skin_index)
         .finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub fn new(r: u8, b: u8, g: u8) -> Self {
        Self { r, b, g }
    }
}

const TEXTURE_HEADER_SIZE: usize = 80;

#[derive(Clone)]
pub struct TextureHeader {
    pub name: [u8; 64],
    pub flags: u32,
    pub width: u32,
    pub height: u32,
    pub index: u32,
}

impl TextureHeader {
    pub fn from_reader(reader: &mut ByteBuffer) -> Self {
        reader.set_endian(bytebuffer::Endian::LittleEndian);
        let name = reader.read_bytes(64).expect("failed to get array");
        let flags = reader.read_u32().expect("Failed to get u32");
        let width = reader.read_u32().expect("Failed to get u32");
        let height = reader.read_u32().expect("Failed to get u32");
        let index = reader.read_u32().expect("Failed to get u32");

        Self {
            name: name.try_into().unwrap(),
            flags,
            width,
            height,
            index,
        }
    }
}

impl fmt::Debug for TextureHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
         .field("\n\t name", &std::str::from_utf8(&self.name).unwrap())
         .field("\n\t flags", &self.flags)
         .field("\n\t width", &self.width)
         .field("\n\t height", &self.height)
         .field("\n\t index", &self.index)
         .finish()
    }
}


#[derive(Clone)]
pub struct Texture {
    pub raw_data: Vec<u8>,
    pub palette: [ColorRGB; 256],
    pub header: TextureHeader,
}

impl Texture {
    pub fn from_reader(reader: &mut ByteBuffer, header: TextureHeader) -> Self {
        reader.set_endian(bytebuffer::Endian::LittleEndian);
        let img_size: usize = (header.width * header.height) as usize;
        let raw_data = reader.read_bytes(img_size).expect("Failed to get array").try_into().expect("Failed to convert array");

        let palette_raw = reader.read_bytes(256*3).unwrap();
        let mut palette_reader = ByteBuffer::from(palette_raw);
        palette_reader.set_endian(bytebuffer::Endian::LittleEndian);
        let mut palette = vec![ColorRGB::new(0, 0, 0); 256];
        for itr in 0..256 {
            let r = palette_reader.read_u8().expect("Failed to get value");
            let g = palette_reader.read_u8().expect("Failed to get value");
            let b = palette_reader.read_u8().expect("Failed to get value");
            palette[itr] = ColorRGB::new(r, g, b);
        }

        Self {
            raw_data,
            palette: palette.try_into().unwrap(),
            header,
        }
    }
    
    pub fn to_rgb_image_vec(&self) -> Vec<u8> {
        let image_size: usize = (self.header.width * self.header.height) as usize;

        let mut vec = vec![0; image_size*3];
        let mut image_offset = 0;

        for itr in 0..image_size {
            let palette_index = self.raw_data[itr] as usize;
            let color = self.palette[palette_index];
            vec[image_offset + 0] = color.r;
            vec[image_offset + 2] = color.g;
            vec[image_offset + 1] = color.b;
            image_offset += 3;
        }

        vec
    }

    /*
    fn save_img(&self) {
        let null_terminator: char = 0 as char;
        let filename = ::std::str::from_utf8(&self.header.name).unwrap().trim_matches(|c| c == null_terminator);

        let mut img = Image::new(self.header.width as u32, self.header.height as u32);

        for (itr, (x, y)) in img.coordinates().enumerate() {
            let palette_index = self.raw_data[itr] as usize;
            img.set_pixel(x, y, self.palette[palette_index]); 
        }

        let _ = img.save(filename);
    }
    */
}

const MDL_HEADER_SIZE:usize  = 1024;

pub struct MdlFile {
    header: MdlHeader,
    textures: Vec<Texture>,
}

impl MdlFile {
    pub fn from_bytes(buf: &Vec::<u8>) -> Option<Self> {
        if !Self::validate_header(buf) {
            return None
        }

        let mut reader = ByteBuffer::from_vec(buf.to_owned());
        let header = MdlHeader::from_reader(&mut reader);
        let textures = Self::get_textures_from_header(header, &mut reader);

        Some(Self {
            header,
            textures,
        })
    }

    fn get_textures_from_header(header: MdlHeader, reader: &mut ByteBuffer) -> Vec<Texture> {
        let mut ret_vec = vec![];
        let texture_offset = header.texture_index as usize;
        let num_textures = header.num_textures as usize;
        reader.set_endian(bytebuffer::Endian::LittleEndian);
        reader.set_rpos(texture_offset);
        let mut header_vec = vec![];
        for _itr in 0..num_textures {
            header_vec.push(TextureHeader::from_reader(reader));
        }
        for texture_header in header_vec.iter() {
            let data_offset = texture_header.index as usize;
            reader.set_rpos(data_offset);
            ret_vec.push(Texture::from_reader(reader, texture_header.to_owned()));
        }

        ret_vec
    }

    pub fn validate_header(buf: &Vec::<u8>) -> bool {
        if buf.len() < MDL_HEADER_SIZE {
            return false
        }
        let header = MdlHeader::from_bytes(buf[0..MDL_HEADER_SIZE].try_into().unwrap());
        let id_bytes = header.id.to_le_bytes();
        let id =  std::str::from_utf8(&id_bytes).unwrap();
        if id != "IDST" {
            return false
        }
        true
    }
}

pub struct MdlFileWidget {
    pub mdl_file: MdlFile,
    pub mdl_image: Option<egui::TextureHandle>,
    pub textures: Vec<egui::TextureHandle>,
    pub texture_index: usize,
    pub update_texture: bool,
    pub init_textures: bool,
    pub name: String,
    pub visible: bool,
    pub id: usize,
}

impl MdlFileWidget {
    pub fn from_bytes(buf: &Vec<u8>, id: usize) -> Self {
        let mdl_file = MdlFile::from_bytes(buf).expect("When they enter the last level, it's exciting");
        let mdl_image = None;
        let textures = vec![];
        let texture_index = 0;
        let update_texture = true;
        let init_textures = true;
        let name = String::from_utf8_lossy(&mdl_file.header.name).to_string();
        Self {
            mdl_file,
            mdl_image,
            textures,
            texture_index,
            update_texture,
            init_textures,
            name,
            visible: true,
            id,
        }
    }
}

impl super::HlFileWidget for MdlFileWidget {
    fn show(&mut self, ctx: &egui::Context) {
        let mut vis = self.visible;
        use super::View as _;
        egui::Window::new(self.name.as_str())
            .open(&mut vis)
            .scroll2([true, true])
            .id(egui::Id::new(self.id))
            .show(ctx, |ui| self.ui(ui));
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn get_visibility(&mut self) -> bool {
        self.visible 
    }
}


impl super::View for MdlFileWidget {
    fn ui(&mut self, ui: &mut egui::Ui) {
        if self.init_textures {
            self.textures.clear();
            for texture in self.mdl_file.textures.iter() {
                let egui_image = ui.ctx().load_texture(
                    "my-image", 
                    egui::ColorImage::from_rgb(
                        [texture.header.width as usize, texture.header.height as usize], 
                        &texture.to_rgb_image_vec()),
                    Default::default());
                self.textures.push(egui_image);
            }
            self.init_textures = false;
        }

        ui.horizontal(|ui| {
            match &self.mdl_image {
                Some(image) => {
                    ui.horizontal(|ui| {
                        ui.set_height(256.);
                        ui.set_width(512.);
                        ui.add(egui::Image::new(image, image.size_vec2()));
                    });
                },
                None => {
                    let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                        "my-image", 
                        egui::ColorImage::example(),
                        Default::default());
                    ui.add_sized([300., 300.], egui::Image::new(texture, texture.size_vec2()));
                },
            }
        });
        egui::ScrollArea::horizontal()
            .id_source("second")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (itr, texture) in self.textures.iter().enumerate() {
                        let response = ui.add(egui::ImageButton::new(texture, texture.size_vec2()));
                        if response.clicked() {
                            self.texture_index = itr;
                            self.update_texture = true;
                        }
                    }
                });
            });
        if self.update_texture {
            self.mdl_image = Some(self.textures[self.texture_index].to_owned());
            self.update_texture = false;
        }
    }
}
