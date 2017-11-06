use std;
// use std::ops::Index;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::ffi::CStr;

mod stb_image;

use self::stb_image::*;

#[derive(Clone, Copy, Debug)]
pub enum Pixel {
    Rgba8 { r: u8, g: u8, b: u8, a: u8 },
    A8 { a: u8 },
}

pub struct Image {
    path: Option<PathBuf>,
    stbi: StbImage,
}

impl Image {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Image> {
        let path_buf = path.as_ref().to_path_buf();
        let mut file = File::open(path)?;

        let file_size = file.seek(io::SeekFrom::End(0))? as usize;
        file.seek(io::SeekFrom::Start(0))?;

        let mut buf = Vec::with_capacity(file_size);
        file.read_to_end(&mut buf)?;

        let stbi = StbImage::load_from_memory(&mut buf)?;

        assert!(stbi.n == 4);

        Ok(Image {
            path: Some(path_buf),
            stbi,
        })
    }

    pub fn width(&self) -> u32 {
        self.stbi.w as u32
    }

    pub fn height(&self) -> u32 {
        self.stbi.h as u32
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref().map(|path_buf| path_buf.as_path())
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        let w = self.stbi.w as u32;
        let h = self.stbi.h as u32;

        if x >= w || y >= h {
            return None;
        }

        let n = self.stbi.n as u32;
        let stride = w * n;
        let offset = y * stride + x * n;
        let pixel = unsafe {
            let p = self.stbi.data.offset(offset as isize);
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

        Some(pixel)
    }

    pub fn pixels(&self) -> Pixels {
        Pixels {
            image: self,
            offset: 0,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        self.stbi.bytes()
    }
}

pub struct Pixels<'a> {
    image: &'a Image,
    offset: u32,
}

impl<'a> Iterator for Pixels<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let w = self.image.stbi.w as u32;
        let x = self.offset % w;
        let y = self.offset / w;
        self.offset += 1;
        self.image.pixel(x, y)
    }
}

// impl<'a> Index<(u32, u32)> for Pixels<'a> {
//     type Output = Option<Pixel>;

//     fn index(&self, index: (u32, u32)) -> &Self::Output {
//         self.image.get_pixel(index.0, index.1)
//     }
// }

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

    fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data, (self.n * self.w * self.h) as usize) }
    }
}


impl Drop for StbImage {
    fn drop(&mut self) {
        unsafe { stbi_image_free(self.data) };
    }
}
