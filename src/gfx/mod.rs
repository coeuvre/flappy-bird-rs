extern crate gl;

pub mod image;

use std;
use std::os::raw::c_void;

use self::image::GeneralImage;

pub struct GlContext {}

impl GlContext {
    pub fn is_current(&self) -> bool {
        true
    }

    pub fn gen_texture_2d(&mut self) -> GlTextureId {
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
        }
        GlTextureId {
            id: texture,
            target: gl::TEXTURE_2D,
        }
    }

    pub fn bind_texture(&mut self, texture_id: &GlTextureId) {
        assert!(self.is_current());
        unsafe {
            gl::BindTexture(texture_id.target, texture_id.id);
        }
    }
}

pub struct GlTextureId {
    id: u32,
    target: u32,
}

pub struct GlTexture2D {
    ctx: *mut GlContext,
    id: GlTextureId,
    w: u32,
    h: u32,
}

impl GlTexture2D {
    pub fn from_image(ctx: &mut GlContext, image: &GeneralImage) -> GlTexture2D {
        assert!(ctx.is_current());
        assert!(image.num_bytes_per_component() == 1);

        let id = ctx.gen_texture_2d();
        ctx.bind_texture(&id);

        let w = image.width();
        let h = image.height();
        let bytes = image.bytes();
        let stride = image.stride();

        let texture_stride: isize;
        let num_pixels;
        let internal_format;
        let format;
        match image.num_component() {
            4 => {
                texture_stride = w as isize * 4;
                num_pixels = w;
                internal_format = gl::SRGB8_ALPHA8;
                format = gl::RGBA;
            }
            1 => {
                texture_stride = ((w as f32 / 4.0).ceil() * 4.0) as isize;
                num_pixels = texture_stride as u32;
                internal_format = gl::R8;
                format = gl::RED;

                let swizzle_mask = [
                    gl::ONE as i32,
                    gl::ONE as i32,
                    gl::ONE as i32,
                    gl::RED as i32,
                ];
                unsafe {
                    gl::TexParameteriv(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_SWIZZLE_RGBA,
                        swizzle_mask.as_ptr(),
                    )
                };
            }
            _ => unreachable!(),
        }

        assert!(stride <= texture_stride);
        assert!(texture_stride % 4 == 0);

        // flip image vertically
        let mut vec = Vec::<u8>::with_capacity(texture_stride as usize * h as usize);
        unsafe {
            let mut dst: *mut u8 = vec.as_mut_ptr().offset(texture_stride * (h - 1) as isize);
            let mut src: *const u8 = bytes.as_ptr();

            for _ in 0..h {
                std::ptr::copy_nonoverlapping(src, dst, stride as usize);
                src = src.offset(stride);
                dst = dst.offset(-texture_stride);
            }
        }

        unsafe {
            gl::TexParameteri(id.target, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(id.target, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(id.target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(id.target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, num_pixels as i32);
            gl::TexImage2D(
                id.target,
                0,
                internal_format as i32,
                w as i32,
                h as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                vec.as_ptr() as *const c_void,
            );
        }

        GlTexture2D { ctx, id, w, h }
    }

    // fn set_parameteri(&mut self, name: u32, value: i32) {
    //     unsafe {
    //         (*self.ctx).bind_texture(&self.id);
    //         gl::TexParameteri(self.id.target, name, value);
    //     }
    // }

    // fn set_image_2d<T>(
    //     &mut self,
    //     texture_id: &GlTextureId,
    //     level: i32,
    //     internal_format: i32,
    //     width: i32,
    //     height: i32,
    //     format: u32,
    //     type_: u32,
    //     data: *const T,
    // ) {
    //     self.bind_texture(texture_id);
    // }
}
