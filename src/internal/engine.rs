use web_sys::*;

use crate::shader::*;
use crate::webgl_utils::*;

static U_TRANSFORM: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];

pub struct Program {
   gl: WebGlRenderingContext,
   program: WebGlProgram,
   buffer: WebGlBuffer,
   color_buffer: WebGlBuffer,
   u_opacity: WebGlUniformLocation,
   u_transform: WebGlUniformLocation,
   indices_size: usize
}

impl Program {

    pub fn new(context: WebGlRenderingContext) -> Self {

        let vert_shader = compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER, TRI_VERT).unwrap();
        let frag_shader = compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, TRI_FRAG).unwrap();
        let program = link_program(&context, &vert_shader, &frag_shader).unwrap();
        assert!(context.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
                    .as_bool()
                    .unwrap()
        );
            
        let vertices: [f32; 8] = [  0.7, 0.7, 
                                    -0.7, 0.7, 
                                    -0.7, -0.7,
                                    0.7, -0.7
        ];

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        let vert_array = unsafe { js_sys::Float32Array::view(&vertices) };
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER, 
            &vert_array, 
            WebGlRenderingContext::STATIC_DRAW
        ); 

        let buffer_indices = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices)); 
        let indices_array = unsafe { js_sys::Uint16Array::view(&indices) };
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            WebGlRenderingContext::STATIC_DRAW
        );

        Self {
            indices_size: indices.len(),
            u_transform: context.get_uniform_location(&program, "uTransform").unwrap(),
            u_opacity: context.get_uniform_location(&program, "uOpacity").unwrap(),
            color_buffer: context.create_buffer().unwrap(),
            gl: context,
            buffer,
            program,
        }
    }

    pub fn render(&self)  {
        let context = &self.gl;
        context.use_program(Some(&self.program));
        
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        context.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);

        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.color_buffer));
        context.vertex_attrib_pointer_with_i32(1, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(1);

        let colors: [f32; 16] = [
                    1.0, 0.0, 0.0, 1.0,
                    0.0, 1.0, 0.0, 1.0,
                    0.0, 0.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
        ];

        let colors_array = unsafe { js_sys::Float32Array::view(&colors) };
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &colors_array,
            WebGlRenderingContext::STATIC_DRAW,
        );


        context.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &U_TRANSFORM);
        context.uniform1f(Some(&self.u_opacity), 1.0);

        context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        context.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, self.indices_size as i32, WebGlRenderingContext::UNSIGNED_SHORT, 0);
    }
}
