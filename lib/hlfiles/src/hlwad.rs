use std::io::{BufWriter, Read, Seek};
use std::fs::File;
use std::path::Path;
use std::fmt;
use std::collections::HashMap;

pub const WAD_HEADER_SIZE: usize = 12;

#[derive(Clone, Copy)]
pub struct WadHeader {
    pub sz_magic: [u8; 4],
    pub n_dir: u32,
    pub n_dir_offset: u32,
}

impl WadHeader {
    fn from_bytes(buf: &[u8; 12]) -> Self {
        let sz_magic: [u8; 4] = buf[..4].try_into().expect("fucked up");
        let n_dir = u32::from_le_bytes(buf[4..8].try_into().expect("fucked up"));
        let n_dir_offset = u32::from_le_bytes(buf[8..12].try_into().expect("fucked up"));
        Self {
            sz_magic,
            n_dir,
            n_dir_offset,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut ret_vec = Vec::<u8>::new();
        ret_vec.append(&mut self.sz_magic.to_vec());
        ret_vec.append(&mut self.n_dir.to_le_bytes().to_vec());
        ret_vec.append(&mut self.n_dir_offset.to_le_bytes().to_vec());
        ret_vec
    }
}

impl fmt::Debug for WadHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
         .field("sz_magic", &std::str::from_utf8(&self.sz_magic).unwrap())
         .field("n_dir", &self.n_dir)
         .field("n_dir_offset", &self.n_dir_offset)
         .finish()
    }
}

pub const DIRECTORY_ENTRY_SIZE: usize = 32;
pub const DIRECTORY_ENTRY_NAME_SIZE: usize = 16;

#[derive(Clone, Copy)]
pub struct DirectoryEntry {
    pub n_file_pos: u32,
    pub n_disk_size: u32,
    pub n_size: u32,
    pub n_type: u8,
    pub b_compression: u8,
    pub padding: u16, // unused
    pub sz_name: [u8; DIRECTORY_ENTRY_NAME_SIZE],
}

impl DirectoryEntry {
    pub fn from_bytes(buf: &[u8; 32]) -> Self {
        let n_file_pos = u32::from_le_bytes(buf[..4].try_into().expect("fucked up"));
        let n_disk_size = u32::from_le_bytes(buf[4..8].try_into().expect("fucked up"));
        let n_size = u32::from_le_bytes(buf[8..12].try_into().expect("fucked up"));
        let n_type = buf[12];
        let b_compression = buf[13];
        let padding = u16::from_le_bytes(buf[14..16].try_into().expect("fucked up"));
        let sz_name: [u8; 16] = buf[16..32].try_into().expect("fucked up");
        Self {
            n_file_pos,
            n_disk_size,
            n_size,
            n_type,
            b_compression,
            padding,
            sz_name,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut ret_vec = Vec::<u8>::new();
        ret_vec.append(&mut self.n_file_pos.to_le_bytes().to_vec());
        ret_vec.append(&mut self.n_disk_size.to_le_bytes().to_vec());
        ret_vec.append(&mut self.n_size.to_le_bytes().to_vec());
        ret_vec.append(&mut self.n_type.to_le_bytes().to_vec());
        ret_vec.append(&mut self.b_compression.to_le_bytes().to_vec());
        ret_vec.append(&mut self.padding.to_le_bytes().to_vec());
        ret_vec.append(&mut self.sz_name.to_vec());
        ret_vec
    }

    pub fn name_str(&self) -> Option<String> {
        let mut ret_opt: Option<String> = None;
        for itr in 0..DIRECTORY_ENTRY_NAME_SIZE {
            if self.sz_name[itr] == 0 {
                let substr = self.sz_name[0..itr].try_into().unwrap();
                ret_opt = Some(String::from_utf8(substr).unwrap());
                break;
            }
        }
        ret_opt
    }
}

impl fmt::Debug for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
         .field("n_file_pos", &self.n_file_pos)
         .field("n_disk_size", &self.n_disk_size)
         .field("n_size", &self.n_size)
         .field("n_type", &self.n_type)
         .field("b_compression", &self.b_compression)
         .field("padding", &self.padding)
         .field("sz_name", &self.name_str())
         .finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }

    fn to_vec(&self) -> Vec<u8> {
        vec![self.r, self.b, self.g]
    }
}

pub const TEXTURE_HEADER_SIZE: usize = 40;

#[derive(Copy, Clone)]
pub struct TextureHeader {
    pub sz_name: [u8; 16],
    pub n_width: u32,
    pub n_height: u32,
    pub mip_offsets: [u32; 4],
}


impl TextureHeader {
    pub fn from_bytes(buf: &[u8; 40]) -> Self {
        let sz_name: [u8; 16] = buf[0..16].try_into().expect("fucked");
        let n_width = u32::from_le_bytes(buf[16..20].try_into().expect("fucked"));
        let n_height = u32::from_le_bytes(buf[20..24].try_into().expect("fucked"));
        let mip_offset_1 = u32::from_le_bytes(buf[24..28].try_into().expect("fucked"));
        let mip_offset_2 = u32::from_le_bytes(buf[28..32].try_into().expect("fucked"));
        let mip_offset_3 = u32::from_le_bytes(buf[32..36].try_into().expect("fucked"));
        let mip_offset_4 = u32::from_le_bytes(buf[36..40].try_into().expect("fucked"));
        let mip_offsets: [u32; 4] = [mip_offset_1, mip_offset_2, mip_offset_3, mip_offset_4];
        Self {
            sz_name,
            n_width,
            n_height,
            mip_offsets,
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut ret_vec = Vec::<u8>::new();
        ret_vec.append(&mut self.sz_name.to_vec());
        ret_vec.append(&mut self.n_width.to_le_bytes().to_vec());
        ret_vec.append(&mut self.n_height.to_le_bytes().to_vec());
        for itr in 0..4 {
            ret_vec.append(&mut self.mip_offsets[itr].to_le_bytes().to_vec());
        }
        ret_vec
    }
}

impl fmt::Debug for TextureHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
         .field("sz_name", &std::str::from_utf8(&self.sz_name).unwrap())
         .field("n_width", &self.n_width)
         .field("n_height", &self.n_height)
         .field("mip_offset_1", &self.mip_offsets[0])
         .field("mip_offset_2", &self.mip_offsets[1])
         .field("mip_offset_3", &self.mip_offsets[2])
         .field("mip_offset_4", &self.mip_offsets[3])
         .finish()
    }
}

#[derive(Clone)]
pub enum MIPMAP_LEVEL {
    LEVEL0 = 0,
    LEVEL1 = 1,
    LEVEL2 = 2,
    LEVEL3 = 3,
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub header: TextureHeader,
    pub data: Vec<u8>,
    pub palette: [Color; 256],
}

impl Texture {
    pub fn from_directory_entry(entry: DirectoryEntry, wad_data: &Vec<u8>) -> Self {
        let mut palette: [Color; 256] = [Color::new(0, 0, 0); 256];
        let mut palette_offset = (entry.n_file_pos + entry.n_disk_size - (256 * 3) - 2) as usize;
        let header_slice = &wad_data[(entry.n_file_pos as usize)..((entry.n_file_pos as usize) + TEXTURE_HEADER_SIZE)];
        let header = TextureHeader::from_bytes(header_slice.try_into().expect("failed")); 
        for mut color in palette.iter_mut() {
            *color = Color::new(wad_data[palette_offset], wad_data[palette_offset + 1], wad_data[palette_offset + 2]);
            palette_offset += 3;
        }
        let data = wad_data[((entry.n_file_pos as usize) + TEXTURE_HEADER_SIZE)..palette_offset].to_vec();
        Self {
            header,
            data,
            palette,
        }
    }

    pub fn to_rgb_image_vec(&self, level: MIPMAP_LEVEL) -> Vec<u8> {
        let div = 1 << level.clone() as usize;
        let image_size = ((self.header.n_width/div) * (self.header.n_height/div)) as usize;
        let mut vec = vec![0; image_size * 3];
        let mut image_offset = 0;
        for idx in 0..image_size {
            let palette_index = self.data[idx + (self.header.mip_offsets[level.clone() as usize] as usize) - TEXTURE_HEADER_SIZE] as usize;
            let color = self.palette[palette_index];
            vec[image_offset] = color.r;
            vec[image_offset + 1] = color.g;
            vec[image_offset + 2] = color.b;
            image_offset += 3;
        }
        vec
    }

    pub fn to_rgb_color_vec(&self) -> Vec<Color> {
        let image_size = (self.header.n_width * self.header.n_height) as usize;
        let mut vec = vec![Color::new(0, 0, 0); image_size];
        for idx in 0..image_size {
            let palette_index = self.data[idx] as usize;
            let color = self.palette[palette_index];
            vec[idx] = color;
        }
        vec
    }

    fn to_vec(&mut self) -> Vec<u8> {
        let mut ret_vec = Vec::<u8>::new(); 
        ret_vec.append(&mut self.header.to_vec());
        ret_vec.append(&mut self.data);
        for color in self.palette {
            ret_vec.append(&mut color.to_vec());
        }
        ret_vec
    }

    fn calculated_size(&self) -> u32 {
        (TEXTURE_HEADER_SIZE + self.data.len() + (256 * 3) + 2) as u32
    }
}

#[derive(Debug, Clone)]
pub struct EntryPair {
    pub dir_entry: DirectoryEntry,
    pub texture: Texture,
}

#[derive(Debug, Clone)]
pub struct WadFile {
    pub header: WadHeader,
    pub entries: Vec<EntryPair>, 
}

impl WadFile {
    pub fn validate_header(buf: &Vec::<u8>) -> bool {
        if buf.len() < WAD_HEADER_SIZE {
            return false
        }
        let header = WadHeader::from_bytes(buf[0..WAD_HEADER_SIZE].try_into().unwrap());
        let sz_magic_str =  std::str::from_utf8(&header.sz_magic).unwrap();
        if sz_magic_str != "WAD3" {
            return false
        }
        true
    }

    pub fn from_path(path: &Path) -> Self {
        let mut wad_file = File::open(path).expect("fucked");
        let mut wad_data = vec![];
        wad_file.read_to_end(&mut wad_data).unwrap();
        Self::from_bytes(&wad_data)
    }

    pub fn from_bytes(wad_data: &Vec<u8>) -> Self {
        let header_buf: [u8; WAD_HEADER_SIZE] = wad_data[0..WAD_HEADER_SIZE].try_into().expect("fucked");
        let header = WadHeader::from_bytes(&header_buf);
        let mut entries: Vec<EntryPair> = Vec::<EntryPair>::new();
        for itr in 0..(header.n_dir as usize) {
            let entry_offset = (header.n_dir_offset as usize) + (itr * DIRECTORY_ENTRY_SIZE);
            let dir_entry = DirectoryEntry::from_bytes(&wad_data[entry_offset..(entry_offset + DIRECTORY_ENTRY_SIZE)].try_into().expect("fucked"));
            let texture = Texture::from_directory_entry(dir_entry, &wad_data);
            entries.push(EntryPair { dir_entry, texture });
        }
        Self {
            header,
            entries,
        }
    }

    fn gen_header(&self, dir_offset: u32) -> WadHeader {
        WadHeader {
            sz_magic: "WAD3".as_bytes().try_into().unwrap(),
            n_dir: self.entries.len() as u32,
            n_dir_offset: dir_offset,
        }
    }

    pub fn regenerate(&mut self) {
        let mut offset_count = WAD_HEADER_SIZE as u32;
        for entry in self.entries.iter_mut() {
            let size = entry.texture.calculated_size();
            entry.dir_entry = DirectoryEntry {
                n_file_pos: offset_count,
                n_disk_size: size,
                n_size: size,
                n_type: 0x43,
                b_compression: 0,
                padding: 42069,
                sz_name: entry.dir_entry.sz_name,
            };
            offset_count += size;
        }
        self.header = self.gen_header(offset_count);
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let mut ret_vec = Vec::<u8>::new();
        ret_vec.append(&mut self.header.to_vec());
        for entry in &mut self.entries {
            ret_vec.append(&mut entry.texture.to_vec());
        }
        for entry in &mut self.entries {
            ret_vec.append(&mut entry.dir_entry.to_vec());
        }
        ret_vec
    }
}

pub struct WadFileWidget {
    pub wad_file: WadFile,
    pub wad_image: Option<egui::TextureHandle>,
    pub textures: Vec<egui::TextureHandle>,
    pub texture_index: usize,
    pub update_texture: bool,
    pub init_textures: bool,
    pub name: String,
    pub id: usize,
}

impl WadFileWidget {
    pub fn from_bytes(buf: &Vec<u8>, id: usize) -> Self {
        let wad_file = WadFile::from_bytes(buf);
        let wad_image = None;
        let textures = vec![];
        let texture_index = 0;
        let update_texture = true;
        let init_textures = true;
        let name = String::from_utf8_lossy(&wad_file.entries[0].dir_entry.sz_name).to_string();
        Self {
            wad_file,
            wad_image,
            textures,
            texture_index,
            update_texture,
            init_textures,
            name,
            id,
        }
    }
}

impl super::HlFileWidget for WadFileWidget {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View as _;
        egui::Window::new(self.name.as_str())
            .open(open)
            .scroll2([true, true])
            .id(egui::Id::new(self.id))
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for WadFileWidget {
    fn ui(&mut self, ui: &mut egui::Ui) {
        if self.init_textures {
            self.textures.clear();
            for entry in self.wad_file.entries.iter() {
                let texture = &entry.texture;
                let egui_image = ui.ctx().load_texture(
                    "my-image", 
                    egui::ColorImage::from_rgb(
                        [texture.header.n_width as usize, texture.header.n_height as usize], 
                        &texture.to_rgb_image_vec(MIPMAP_LEVEL::LEVEL0)),
                    Default::default());
                self.textures.push(egui_image);
            }
            self.init_textures = false;
        }
        ui.horizontal(|ui| {
            match &self.wad_image {
                Some(image) => {
                    ui.horizontal_centered(|ui| {
                        ui.set_height(256.);
                        ui.set_width(512.);
                        ui.menu_image_button(image.into(), image.size_vec2(), |ui| {
                            if ui.button("Download").clicked() {
                                ui.close_menu();
                            } 
                            if ui.button("Upload").clicked() {
                                ui.close_menu();
                            } 
                            if ui.button("Close").clicked() {
                                ui.close_menu();
                            } 
                        });
                        //ui.add(egui::Image::new(image, image.size_vec2()));
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
            self.wad_image = Some(self.textures[self.texture_index].to_owned());
            self.update_texture = false;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("ass");
    }
    #[test]
    fn read_header() {
    }
}
