use std;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::ffi::CStr;

mod stb_image;

use self::stb_image::*;

pub enum SupportedImageStorage {
    Rgba8(ImageStorage<Rgba8>),
    A8(ImageStorage<A8>),
}

pub struct GeneralImage {
    w: u32,
    h: u32,
    storage: SupportedImageStorage,
}

impl GeneralImage {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<GeneralImage> {
        let mut file = File::open(&path)?;

        let file_size = file.seek(io::SeekFrom::End(0))? as usize;
        file.seek(io::SeekFrom::Start(0))?;

        let mut buf = Vec::with_capacity(file_size);
        file.read_to_end(&mut buf)?;

        let stbi = StbImage::<u8>::load_from_memory(&mut buf)?;

        Ok(GeneralImage {
            w: stbi.w as u32,
            h: stbi.h as u32,
            storage: match stbi.n {
                4 => SupportedImageStorage::Rgba8(ImageStorage::<Rgba8>::new(stbi)),
                1 => SupportedImageStorage::A8(ImageStorage::<A8>::new(stbi)),
                _ => unimplemented!(),
            },
        })
    }

    pub fn num_component(&self) -> u32 {
        match self.storage {
            SupportedImageStorage::Rgba8(_) => Rgba8::NUM_COMPONENT,
            SupportedImageStorage::A8(_) => A8::NUM_COMPONENT,
        }
    }

    pub fn num_bytes_per_component(&self) -> usize {
        match self.storage {
            SupportedImageStorage::Rgba8(_) => <Rgba8 as Pixel>::Component::NUM_BYTES,
            SupportedImageStorage::A8(_) => <A8 as Pixel>::Component::NUM_BYTES,
        }
    }

    pub fn num_bytes_per_pixel(&self) -> usize {
        match self.storage {
            SupportedImageStorage::Rgba8(_) => <Rgba8 as Pixel>::NUM_BYTES,
            SupportedImageStorage::A8(_) => <A8 as Pixel>::NUM_BYTES,
        }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn stride(&self) -> isize {
        self.num_bytes_per_pixel() as isize * self.width() as isize
    }

    pub fn bytes(&self) -> &[u8] {
        match self.storage {
            SupportedImageStorage::Rgba8(ref storage) => storage.bytes(),
            SupportedImageStorage::A8(ref storage) => storage.bytes(),
        }
    }
}

pub trait Component {
    const NUM_BYTES: usize;
}

impl Component for u8 {
    const NUM_BYTES: usize = 1;
}

impl Component for f32 {
    const NUM_BYTES: usize = 4;
}

pub trait Pixel {
    type Component: Component;
    const NUM_COMPONENT: u32;
    const NUM_BYTES: usize = Self::Component::NUM_BYTES * Self::NUM_COMPONENT as usize;

    fn component(&self, n: u32) -> Option<&Self::Component>;
}

#[repr(C)]
pub struct Rgba8 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel for Rgba8 {
    type Component = u8;
    const NUM_COMPONENT: u32 = 4;

    fn component(&self, n: u32) -> Option<&Self::Component> {
        match n {
            0 => Some(&self.r),
            1 => Some(&self.g),
            2 => Some(&self.b),
            3 => Some(&self.a),
            _ => None,
        }
    }
}

#[repr(C)]
pub struct A8 {
    a: u8,
}

impl Pixel for A8 {
    type Component = u8;
    const NUM_COMPONENT: u32 = 1;

    fn component(&self, n: u32) -> Option<&Self::Component> {
        match n {
            0 => Some(&self.a),
            _ => None,
        }
    }
}

pub struct ImageStorage<P: Pixel> {
    stbi: StbImage<P::Component>,
    _phantom: std::marker::PhantomData<P>,
}

impl<P: Pixel> ImageStorage<P> {
    fn new(stbi: StbImage<P::Component>) -> ImageStorage<P> {
        assert!(stbi.n == P::NUM_COMPONENT as i32);
        assert!(stbi.w * stbi.n == P::NUM_BYTES as i32 * stbi.w);

        ImageStorage {
            stbi,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<&P> {
        let w = self.stbi.w as u32;
        let h = self.stbi.h as u32;

        if x >= w || y >= h {
            return None;
        }

        let n = self.stbi.n as u32;
        let offset = y * w * n + x * n;
        unsafe {
            let p = self.stbi.data.offset(offset as isize);
            let pixel = p as *mut P;

            Some(&*pixel)
        }
    }

    pub fn data(&self) -> &[P::Component] {
        self.stbi.data()
    }

    pub fn bytes(&self) -> &[u8] {
        self.stbi.bytes()
    }
}

struct StbImage<C: Component> {
    data: *mut C,
    w: i32,
    h: i32,
    n: i32,
}

impl<C: Component> StbImage<C> {
    fn empty() -> StbImage<C> {
        StbImage {
            data: std::ptr::null_mut(),
            w: 0,
            h: 0,
            n: 0,
        }
    }

    fn data(&self) -> &[C] {
        unsafe { std::slice::from_raw_parts(self.data, (self.n * self.w * self.h) as usize) }
    }

    fn bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.data as *mut u8, (self.n * self.w * self.h) as usize)
        }
    }
}

impl StbImage<u8> {
    fn load_from_memory(buf: &mut [u8]) -> io::Result<StbImage<u8>> {
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

impl<C: Component> Drop for StbImage<C> {
    fn drop(&mut self) {
        unsafe { stbi_image_free(self.data as *mut u8) };
    }
}
