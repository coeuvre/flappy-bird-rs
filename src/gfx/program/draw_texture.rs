use std;
use std::mem;
use std::os::raw::c_void;

use math::{GlMat3, Trans2};
use super::super::{gl, GlContext, GlTexture2D};
use super::compile_program;

pub struct DrawTextureProgram {
    vao: u32,
    vbo: u32,
    ebo: u32,
    program: u32,
    loc_mvp: i32,
}

impl DrawTextureProgram {
    pub fn new(ctx: &mut GlContext) -> DrawTextureProgram {
        let program = compile_program(ctx, VERTEX_SHADER, FRAGMENT_SHADER);
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        let loc_mvp;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::STREAM_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                std::ptr::null(),
                gl::STREAM_DRAW,
            );

            // t0
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<DrawTextureVertexAttrib>() as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            // t1
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<DrawTextureVertexAttrib>() as i32,
                mem::size_of::<[f32; 3]> as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // t2
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<DrawTextureVertexAttrib>() as i32,
                mem::size_of::<[f32; 6]> as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            // pos
            gl::VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<DrawTextureVertexAttrib>() as i32,
                mem::size_of::<[f32; 9]> as *const c_void,
            );
            gl::EnableVertexAttribArray(3);

            // texcoord
            gl::VertexAttribPointer(
                4,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<DrawTextureVertexAttrib>() as i32,
                mem::size_of::<[f32; 11]> as *const c_void,
            );
            gl::EnableVertexAttribArray(4);

            // color
            gl::VertexAttribPointer(
                5,
                4,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<DrawTextureVertexAttrib>() as i32,
                mem::size_of::<[f32; 13]> as *const c_void,
            );
            gl::EnableVertexAttribArray(5);

            gl::BindVertexArray(0);

            gl::UseProgram(program);
            gl::Uniform1i(
                gl::GetUniformLocation(program, "texture0".as_ptr() as *const i8),
                0,
            );
            loc_mvp = gl::GetUniformLocation(program, "MVP".as_ptr() as *const i8);
        }

        DrawTextureProgram {
            vao,
            vbo,
            ebo,
            program,
            loc_mvp,
        }
    }

    pub fn draw(&mut self, ctx: &mut GlContext, texture: &GlTexture2D, trans: Trans2) {
        assert!(ctx.is_current());

        let glm = GlMat3::from(trans);
        let vertices = [
            DrawTextureVertexAttrib {
                t0: [glm.e[0], glm.e[1], glm.e[2]],
                t1: [glm.e[3], glm.e[4], glm.e[5]],
                t2: [glm.e[6], glm.e[7], glm.e[8]],
                pos: [1.0, 1.0],
                texcoord: [1.0, 1.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            DrawTextureVertexAttrib {
                t0: [glm.e[0], glm.e[1], glm.e[2]],
                t1: [glm.e[3], glm.e[4], glm.e[5]],
                t2: [glm.e[6], glm.e[7], glm.e[8]],
                pos: [1.0, 0.0],
                texcoord: [1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            DrawTextureVertexAttrib {
                t0: [glm.e[0], glm.e[1], glm.e[2]],
                t1: [glm.e[3], glm.e[4], glm.e[5]],
                t2: [glm.e[6], glm.e[7], glm.e[8]],
                pos: [0.0, 0.0],
                texcoord: [0.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            DrawTextureVertexAttrib {
                t0: [glm.e[0], glm.e[1], glm.e[2]],
                t1: [glm.e[3], glm.e[4], glm.e[5]],
                t2: [glm.e[6], glm.e[7], glm.e[8]],
                pos: [0.0, 1.0],
                texcoord: [0.0, 1.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
        ];

        let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of::<DrawTextureVertexAttrib>() as isize * 4,
                vertices.as_ptr() as *const c_void,
                gl::STREAM_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                mem::size_of::<u32>() as isize * 6,
                indices.as_ptr() as *const c_void,
                gl::STREAM_DRAW,
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id.id);

            gl::UseProgram(self.program);
            let mvp = GlMat3::from(Trans2::identity());
            gl::UniformMatrix3fv(self.loc_mvp, 1, gl::FALSE, mvp.e.as_ptr());

            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DrawTextureVertexAttrib {
    t0: [f32; 3],
    t1: [f32; 3],
    t2: [f32; 3],
    pos: [f32; 2],
    texcoord: [f32; 2],
    color: [f32; 4],
}

const VERTEX_SHADER: &'static str = r#"
#version 330 core

uniform mat3 MVP;

layout (location = 0) in mat3 a_transform;
layout (location = 3) in vec2 a_pos;
layout (location = 4) in vec2 a_texcord;
layout (location = 5) in vec4 a_color;

out vec2 v_texcoord;
out vec4 v_color;

void main() {
    gl_Position = vec4(MVP * a_transform * vec3(a_pos, 1), 1);
    v_texcoord = a_texcord;
    v_color = a_color;
}"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 330 core

uniform sampler2D texture0;

in vec2 v_texcoord;
in vec4 v_color;

out vec4 frag_color;

void main() {
    vec4 tex_color = texture(texture0, v_texcoord);
    // Pre-multiply alpha
    tex_color = vec4(tex_color.rgb * tex_color.a, tex_color.a);

    frag_color = tex_color * v_color;
}
"#;
