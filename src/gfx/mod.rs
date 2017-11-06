pub mod image;

use gl;

use self::image::Image;

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
    pub fn from_image(ctx: &mut GlContext, image: &Image) -> GlTexture2D {
        assert!(ctx.is_current());

        let id = ctx.gen_texture_2d();
        ctx.bind_texture(&id);

        unsafe {
            gl::TexParameteri(id.target, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(id.target, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(id.target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(id.target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            // gl::PixelStorei(gl::UNPACK_ROW_LENGTH, numberOfPixels);
            // gl::TexImage2D(id.target, 0, internalFormat, tex->actualWidth, tex->actualHeight, 0, format, GL_UNSIGNED_BYTE, texBuf);
        }

        GlTexture2D {
            ctx: ctx,
            id,
            w: image.width(),
            h: image.height(),
        }
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
