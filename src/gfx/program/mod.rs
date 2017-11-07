pub mod draw_texture;

use std;
use super::{gl, GlContext};

pub fn compile_program(ctx: &mut GlContext, vertex_shader: &str, fragment_shader: &str) -> u32 {
    let vs = compile_shader(ctx, gl::VERTEX_SHADER, vertex_shader);
    let fs = compile_shader(ctx, gl::FRAGMENT_SHADER, fragment_shader);

    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut is_success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut is_success);
        if is_success != gl::TRUE as i32 {
            let mut buf = Vec::<u8>::with_capacity(512);
            let mut len = 0;
            gl::GetProgramInfoLog(
                program,
                buf.capacity() as i32,
                &mut len,
                buf.as_mut_ptr() as *mut i8,
            );
            buf.set_len(len as usize);
            panic!(
                "Failed to link program: {}",
                String::from_utf8(buf).unwrap()
            );
        }

        program
    }
}

pub fn compile_shader(ctx: &mut GlContext, type_: u32, source: &str) -> u32 {
    assert!(ctx.is_current());
    unsafe {
        let shader = gl::CreateShader(type_);
        gl::ShaderSource(
            shader,
            1,
            &(source.as_ptr() as *const i8),
            &(source.len() as i32),
        );
        gl::CompileShader(shader);

        let mut is_success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_success);
        if is_success != gl::TRUE as i32 {
            let mut buf = Vec::<u8>::with_capacity(512);
            let mut len = 0;
            gl::GetShaderInfoLog(
                shader,
                buf.capacity() as i32,
                &mut len,
                buf.as_mut_ptr() as *mut i8,
            );
            buf.set_len(len as usize);
            panic!(
                "Failed to compile shader: {}",
                String::from_utf8(buf).unwrap()
            );
        }

        shader
    }
}
