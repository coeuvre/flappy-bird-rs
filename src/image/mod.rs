use std;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::ffi::CStr;

mod stb_image;

use self::stb_image::*;

pub enum Pixel {
    Rgba8 { r: u8, g: u8, b: u8, a: u8 },
}

pub struct Image {
    stbi: StbImage,
}

impl Image {
    pub fn iter(&self) -> PixelIter {
        PixelIter {
            image: self,
            offset: 0,
        }
    }
}

pub struct PixelIter<'a> {
    image: &'a Image,
    offset: u32,
}

impl<'a> Iterator for PixelIter<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let w = self.image.stbi.w as u32;
        let h = self.image.stbi.h as u32;
        let n = self.image.stbi.n as u32;
        let x = self.offset % w;
        let y = self.offset / w;
        let stride = w * n;

        if y >= h {
            return None;
        }

        let pixel = unsafe {
            let offset = y * stride + x * n;
            let p = self.image.stbi.data.offset(offset as isize);
            match n {
                4 => Pixel::Rgba8 {
                    r: *p.offset(0),
                    g: *p.offset(1),
                    b: *p.offset(2),
                    a: *p.offset(3),
                },
                _ => unreachable!(),
            }
        };

        self.offset += 1;

        Some(pixel)
    }
}

struct StbImage {
    data: *mut u8,
    w: i32,
    h: i32,
    n: i32,
}

impl StbImage {
    fn empty() -> StbImage {
        StbImage {
            data: std::ptr::null_mut(),
            w: 0,
            h: 0,
            n: 0,
        }
    }

    fn load_from_memory(buf: &mut [u8]) -> io::Result<StbImage> {
        let mut stbi = StbImage::empty();
        unsafe {
            stbi.data = stbi_load_from_memory(
                buf.as_mut_ptr(),
                buf.len() as i32,
                &mut stbi.w,
                &mut stbi.h,
                &mut stbi.n,
                0,
            );
        };

        if stbi.data.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, unsafe {
                CStr::from_ptr(stbi_failure_reason())
                    .to_string_lossy()
                    .into_owned()
            }));
        }

        Ok(stbi)
    }
}


impl Drop for StbImage {
    fn drop(&mut self) {
        unsafe { stbi_image_free(self.data) };
    }
}

impl Image {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Image> {
        let mut file = File::open(path)?;

        let file_size = file.seek(io::SeekFrom::End(0))? as usize;
        file.seek(io::SeekFrom::Start(0))?;

        let mut buf = Vec::with_capacity(file_size);
        file.read_to_end(&mut buf)?;

        let stbi = StbImage::load_from_memory(&mut buf)?;

        assert!(stbi.n == 4);

        Ok(Image { stbi })
    }
}
