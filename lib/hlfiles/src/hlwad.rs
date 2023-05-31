use image::buffer::ConvertBuffer;
use rgb::ComponentBytes;
use rgb::FromSlice;
use std::io::{BufWriter, Cursor, Read};
use std::fs::File;
use std::path::Path;
use std::{fmt, default};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::file_dialog::FileDialog;

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
        //ret_vec.append(&mut self.n_file_pos.to_le_bytes().to_vec());
        //ret_vec.append(&mut self.n_disk_size.to_le_bytes().to_vec());
        //ret_vec.append(&mut self.n_size.to_le_bytes().to_vec());
        //ret_vec.append(&mut self.n_type.to_le_bytes().to_vec());
        //ret_vec.append(&mut self.b_compression.to_le_bytes().to_vec());
        //ret_vec.append(&mut self.padding.to_le_bytes().to_vec());
        //ret_vec.append(&mut self.sz_name.to_vec());
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
        //Self {r: b, g: r, b: g}
        Self {r, g, b}
    }

    fn to_vec(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
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
    pub image: image::RgbImage,
}

impl Texture {
    pub fn from_directory_entry(entry: DirectoryEntry, wad_data: &Vec<u8>) -> Self {
        let mut palette: [Color; 256] = [Color::new(0, 0, 0); 256];
        let palette_offset = (entry.n_file_pos + entry.n_disk_size - (256 * 3) - 2) as usize;
        let mut palette_offset_itr = palette_offset;
        let header_slice = &wad_data[(entry.n_file_pos as usize)..((entry.n_file_pos as usize) + TEXTURE_HEADER_SIZE)];
        let header = TextureHeader::from_bytes(header_slice.try_into().expect("failed")); 
        for mut color in palette.iter_mut() {
            *color = Color::new(wad_data[palette_offset_itr], wad_data[palette_offset_itr + 1], wad_data[palette_offset_itr + 2]);
            palette_offset_itr += 3;
        }
        let data = wad_data[((entry.n_file_pos as usize) + TEXTURE_HEADER_SIZE)..palette_offset].to_vec();
        let mut image_vec = Vec::<u8>::new();
        for itr in 0..((header.n_width*header.n_height) as usize) {
            let color = palette[data[itr] as usize];
            //image_vec.push(color.r);
            //image_vec.push(color.g);
            //image_vec.push(color.b);
            image_vec.append(&mut color.to_vec());
        }
        let image = image::RgbImage::from_vec(header.n_width, header.n_height, image_vec).expect("Could not create image from texture");
        Self {
            header,
            data,
            palette,
            image, 
        }
    }

    fn to_array(strings: &[&str] ) -> js_sys::Array {
        let arr = js_sys::Array::new_with_length(strings.len() as u32);
        for (i, s) in strings.iter().enumerate() {
            arr.set(i as u32, JsValue::from_str(s));
        }
        arr
    }

    fn quantize_images(images: Vec<image::RgbImage>) -> (Vec<rgb::RGBA<u8>>, Vec<u8>) {
        let mut attributes = imagequant::new(); 
        let mut histogram = imagequant::Histogram::new(&attributes);
        let mut quant_images = vec![];
        for image in images.iter() {
            let quantizer = imagequant::new();
            let (width, height) = image.dimensions();
            web_sys::console::debug_3(&"USE ME".into(), &width.into(), &height.into());
            let tmp = image.to_vec();
            let mut rgba_image_raw = vec![];
            let mut itr = 0;
            for pixel in tmp.iter() {
                rgba_image_raw.push(*pixel);
                if itr % 4 == 3 {
                    rgba_image_raw.push(0);
                    itr += 1;
                }
                itr += 1;
            }
            rgba_image_raw.push(0);
            let rgba_image = rgba_image_raw.as_rgba();
            let mut quant_image = quantizer.new_image(rgba_image, width as usize, height as usize, 0.0).unwrap();
            histogram.add_image(&quantizer, &mut quant_image);
            quant_images.push(quant_image);
        }
        let mut result = histogram.quantize(&attributes).expect("failure big style");
        let mut indices_ret_vec = vec![];
        for (idx, image) in images.iter().enumerate() {
            let (_, mut indices) = result.remapped(&mut quant_images[idx]).unwrap();
            indices_ret_vec.append(&mut indices);
            web_sys::console::debug_3(&"USE ME".into(), &indices_ret_vec.len().into(), &idx.into());
        }
        let palette = result.palette_vec();
        (palette, indices_ret_vec)
    }

    pub fn from_image(image: image::RgbImage) -> Self {
        let images = Self::gen_mipmaps(image);
        let (width, height) = images[0].dimensions();
        let (palette_vec, mut indices_vec) = Texture::quantize_images(images);
        let mut palette_array: [Color; 256] = [Color::new(0, 0, 0); 256];
        for (itr, color) in palette_vec.iter().enumerate() {
            let r = color.a;
            let g = color.g;
            let b = color.b;
            palette_array[itr] = Color::new(r, g, b);
        }
        let mut size = width * height;
        let mip_offset_1 = TEXTURE_HEADER_SIZE as u32;
        let mip_offset_2 = mip_offset_1 + size;
        let mut size = (width/2) * (height/2);
        let mip_offset_3 = mip_offset_2 + size;
        let mut size = (width/4) * (height/4);
        let mip_offset_4 = mip_offset_3 + size;
        let mip_offsets = [mip_offset_1, mip_offset_2, mip_offset_3, mip_offset_4];
        web_sys::console::debug_5(&"HELP ME".into(), &mip_offset_1.into(), &mip_offset_2.into(), &mip_offset_3.into(), &mip_offset_4.into());
        let header = TextureHeader {
            sz_name: [b'c', b'u', b'm', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            n_width: width, 
            n_height: height, 
            mip_offsets,
        };
        let mut data_vec = vec![];
        data_vec.append(&mut indices_vec);
        data_vec.append(&mut vec![0, 0]);
        let mut rgb_image_vec = vec![];
        for idx in 0..(width*height) {
            let color = palette_array[data_vec[idx as usize] as usize];
            rgb_image_vec.append(&mut color.to_vec());
        }
        web_sys::console::debug_2(&"PISS ME".into(), &palette_array.len().into());
        let image = image::RgbImage::from_vec(width, height, rgb_image_vec).unwrap();
        Self { 
            header,
            data: data_vec,
            palette: palette_array,
            image, 
        }
    }

    fn gen_mipmaps(image: image::RgbImage) -> Vec<image::RgbImage> {
        let orig_image = image.clone();
        let (mut width, mut height) = image.dimensions();
        let mut ret_vec = vec![];
        ret_vec.push(image);
        width = width / 2;
        height = height / 2;
        for itr in 0..3 {
            let img = image::imageops::resize(&orig_image, width, height, image::imageops::FilterType::Gaussian);
            width = width / 2;
            height = height / 2;
            ret_vec.push(img);
        }
        ret_vec
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
        ret_vec.push(0x00);
        ret_vec.push(0x00);
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
                padding: 0,
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
    file_dialog: FileDialog,
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
            file_dialog: Default::default(),
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
                        //ui.set_height(256.);
                        //ui.set_width(512.);
                        ui.menu_image_button(image.into(), image.size_vec2(), |ui| {
                            if ui.button("Download").clicked() {
                                let texture = &self.wad_file.entries[self.texture_index].texture;
                                let texture_header = texture.header;
                                let width = texture_header.n_width;
                                let height = texture_header.n_height;
                                let mut raw_image = Cursor::new(Vec::new());
                                let mut image_writer = BufWriter::new(raw_image);
                                texture.image.write_to(&mut image_writer, image::ImageFormat::Bmp);
                                self.file_dialog.save("cummy.bmp", image_writer.into_inner().unwrap().into_inner());
                                ui.close_menu();
                            } 
                            if ui.button("Upload").clicked() {
                                self.file_dialog.open(); 
                                ui.close_menu();
                            } 
                            if ui.button("Close").clicked() {
                                ui.close_menu();
                            } 
                        });
                        if let Some(file) = self.file_dialog.get() {
                            let name = self.wad_file.entries[self.texture_index].texture.header.sz_name;
                            let image: image::RgbImage = image::load_from_memory_with_format(&file[..], image::ImageFormat::Bmp).unwrap().to_rgb8();
                            self.wad_file.entries[self.texture_index].texture = Texture::from_image(image);
                            self.wad_file.entries[self.texture_index].texture.header.sz_name = name;
                            self.wad_file.regenerate();
                            self.update_texture = true;
                            self.init_textures = true;
                        }
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
        if ui.button("Download").clicked() {
            self.wad_file.regenerate();
            self.file_dialog.save("sick-ass-shit.wad", self.wad_file.to_bytes());
        }
        if ui.button("Piss").clicked() {
            self.wad_file.to_bytes();
        }

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
