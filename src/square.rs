use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
use crate::Render;
use failure;
use gl;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Square {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    _ibo: buffer::ElementArrayBuffer,
    vao: buffer::VertexArray,
}
impl Square {
    pub fn _new(res: &Resources, gl: &gl::Gl) -> Result<Square, failure::Error> {
        // set up shader program

        let program = render_gl::Program::from_res(gl, res, "shaders/tri")?;

        // set up vertex buffer object

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            }, // top right
            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                clr: (1.0, 1.0, 1.0, 1.0).into(),
            }, // top left
        ];
        let indices: Vec<gl::types::GLuint> = vec![0, 1, 2, 2, 3, 0];

        let vbo = render_gl::buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        // set up vertex array object

        let vao = buffer::VertexArray::new(gl);
        vao.bind();
        // set up indices array object

        let ibo = buffer::ElementArrayBuffer::new(gl);
        ibo.bind();
        ibo.static_draw_data(&indices);
        Vertex::vertex_attrib_pointers(gl);

        Ok(Square {
            program,
            _vbo: vbo,
            _ibo: ibo,
            vao,
        })
    }
}

impl Render for Square {
    fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
