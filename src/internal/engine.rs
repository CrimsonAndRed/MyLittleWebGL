use wasm_bindgen::JsCast;
use web_sys::*;
use js_sys::WebAssembly;

use crate::shader::*;
use crate::webgl_utils::*;

pub struct Program {
   gl: WebGlRenderingContext,
   program: WebGlProgram,
   buffer: WebGlBuffer,
   u_color: WebGlUniformLocation,
   u_opacity: WebGlUniformLocation,
   u_transform: WebGlUniformLocation,
   rect_vert_size: usize
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
            

        let vertices: [f32; 6] = [0.0, 0.7, -0.7, -0.7, 0.7, -0.7];
        
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vert_ref = vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(vert_ref, vert_ref + vertices.len() as u32);
        let buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        context.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vert_array, WebGlRenderingContext::STATIC_DRAW); 
        Self {
            u_color: context.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: context.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: context.get_uniform_location(&program, "uTransform").unwrap(),
            gl: context,
            buffer,
            rect_vert_size: vertices.len(),
            program
        }
    }

    pub fn render(&self)  {
        let context = &self.gl;
        context.use_program(Some(&self.program));
        context.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );


        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        context.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);

        context.uniform4f(Some(&self.u_color), 
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
            js_sys::Math::random() as f32,
            1.0
        );

        let mut u_transform: [f32; 16] = [0.0; 16];
        u_transform[0] = 1.0;
        u_transform[5] = 1.0;
        u_transform[10] = 1.0;
        u_transform[15] = 1.0;

        context.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &u_transform);
        context.uniform1f(Some(&self.u_opacity), 1.0);


        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (self.rect_vert_size / 2) as i32);
        //context.clear_color(
        //    js_sys::Math::random() as f32,
        //    js_sys::Math::random() as f32,
        //    js_sys::Math::random() as f32,
        //    1.0,
        //);
    }
}
