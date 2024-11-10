use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct Shaders {
    pub program: WebGlProgram,
}

impl Shaders {
    pub fn new(ctx: &WebGl2RenderingContext, frag_shader: &str, vert_shader: &str) -> Shaders {
        let vert_shader =
            Shaders::compile_shader(&ctx, WebGl2RenderingContext::VERTEX_SHADER, vert_shader)
                .unwrap();
        let frag_shader =
            Shaders::compile_shader(&ctx, WebGl2RenderingContext::FRAGMENT_SHADER, frag_shader)
                .unwrap();

        let program = Shaders::link_program(&ctx, &vert_shader, &frag_shader).unwrap();

        Shaders { program }
    }

    pub fn activate(&self, ctx: &WebGl2RenderingContext) {
        ctx.use_program(Some(&self.program));
    }

    fn link_program(
        ctx: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = ctx
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        ctx.attach_shader(&program, vert_shader);
        ctx.attach_shader(&program, frag_shader);
        ctx.link_program(&program);

        // Delete the shaders as they're linked into our program now and no longer necessary
        ctx.delete_shader(Some(vert_shader));
        ctx.delete_shader(Some(frag_shader));

        if ctx
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(ctx
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
}
