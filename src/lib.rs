extern crate nalgebra_glm as glm;

use std::{cell::RefCell, rc::Rc};

use camera::Camera;
use image::{GenericImageView, ImageReader};
use input_handler::InputHandler;
use shaders::Shaders;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{ImageData, WebGl2RenderingContext, WebGlProgram, WebGlShader};

mod shaders;
mod input_handler;
mod camera;

// Set log from console.log
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    log("Hello from Rust!");

    let canvas = window()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(600);
    canvas.set_height(600);

    let context: web_sys::WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    // Link the shaders into a program
    let frag_shader = include_str!("shaders/fragment.glsl");
    let vert_shader = include_str!("shaders/vertex.glsl");

    let shader = Shaders::new(&context, &frag_shader, &vert_shader);
    shader.activate(&context);

    init(&context, &shader)?;

    let mut input_handler = InputHandler::new();
    let mut camera = Camera::new(glm::vec3(0.0, 0.0, 3.0));

    // requestAnimationFrame
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut angle = 0.0;

    *g.borrow_mut() = Some(Closure::new(move || {
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        camera.process_keys(&input_handler.borrow());
        camera.process_mouse(&input_handler.borrow());

        // log(&format!("Camera position: {:?}", camera.position));
        // log(&format!("Keys: {:?}", input_handler.borrow().keys));

        let view_location = context.get_uniform_location(&shader.program, "view");
        context.uniform_matrix4fv_with_f32_array(
            view_location.as_ref(),
            false,
            &camera.view_matrix().as_ref(),
        );


        //context.bind_vertex_array(Some(&vao));

        /* context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES, // Mode
            0,                                // First
            3,                                // Count
        ); */

        // context.draw_elements_with_i32(
        //     WebGl2RenderingContext::TRIANGLES,
        //     36,
        //     WebGl2RenderingContext::UNSIGNED_SHORT,
        //     0,
        // );

        // Rotate the cube
        let model_location = context.get_uniform_location(&shader.program, "model");
        context.uniform_matrix4fv_with_f32_array(
            model_location.as_ref(),
            false,
            &glm::rotate(
                &glm::mat4(
                    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                ),
                f32::to_radians(angle),
                &glm::vec3(1.0, 0.6, 0.0),
            )
            .as_ref(),
        );

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES, // Mode
            0,                                 // First
            36,                                // Count
        );

        angle = angle + 0.1;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    log("Finished!");
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn init(context: &WebGl2RenderingContext, shader: &Shaders) -> Result<(), JsValue> {
    context.viewport(0, 0, 600, 600);
    context.viewport(0, 0, 600, 600);

    log(&format!(
        "GL_MAX_VERTEX_ATTRIBS: {}",
        context
            .get_parameter(WebGl2RenderingContext::MAX_VERTEX_ATTRIBS)
            .unwrap()
            .as_f64()
            .unwrap()
    ));

    // Set uniform vertexColor
    /* let vertex_color = context
        .get_uniform_location(&program, "vertexColor")
        .unwrap();
    context.uniform4f(Some(&vertex_color), 0.0, 1.0, 0.0, 1.0); */

    // Render a colored cube
    let vertices: [f32; 180] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, // 0
        0.5, -0.5, -0.5, 1.0, 0.0, // 1
        0.5, 0.5, -0.5, 1.0, 1.0, // 2
        0.5, 0.5, -0.5, 1.0, 1.0, // 3
        -0.5, 0.5, -0.5, 0.0, 1.0, // 4
        -0.5, -0.5, -0.5, 0.0, 0.0, // 5
        -0.5, -0.5, 0.5, 0.0, 0.0, // 6
        0.5, -0.5, 0.5, 1.0, 0.0, // 7
        0.5, 0.5, 0.5, 1.0, 1.0, // 8
        0.5, 0.5, 0.5, 1.0, 1.0, // 9
        -0.5, 0.5, 0.5, 0.0, 1.0, // 10
        -0.5, -0.5, 0.5, 0.0, 0.0, // 11
        -0.5, 0.5, 0.5, 1.0, 0.0, // 12
        -0.5, 0.5, -0.5, 1.0, 1.0, // 13
        -0.5, -0.5, -0.5, 0.0, 1.0, // 14
        -0.5, -0.5, -0.5, 0.0, 1.0, // 15
        -0.5, -0.5, 0.5, 0.0, 0.0, // 16
        -0.5, 0.5, 0.5, 1.0, 0.0, // 17
        0.5, 0.5, 0.5, 1.0, 0.0, // 18
        0.5, 0.5, -0.5, 1.0, 1.0, // 19
        0.5, -0.5, -0.5, 0.0, 1.0, // 20
        0.5, -0.5, -0.5, 0.0, 1.0, // 21
        0.5, -0.5, 0.5, 0.0, 0.0, // 22
        0.5, 0.5, 0.5, 1.0, 0.0, // 23
        -0.5, -0.5, -0.5, 0.0, 1.0, // 24
        0.5, -0.5, -0.5, 1.0, 1.0, // 25
        0.5, -0.5, 0.5, 1.0, 0.0, // 26
        0.5, -0.5, 0.5, 1.0, 0.0, // 27
        -0.5, -0.5, 0.5, 0.0, 0.0, // 28
        -0.5, -0.5, -0.5, 0.0, 1.0, // 29
        -0.5, 0.5, -0.5, 0.0, 1.0, // 30
        0.5, 0.5, -0.5, 1.0, 1.0, // 31
        0.5, 0.5, 0.5, 1.0, 0.0, // 32
        0.5, 0.5, 0.5, 1.0, 0.0, // 33
        -0.5, 0.5, 0.5, 0.0, 0.0, // 34
        -0.5, 0.5, -0.5, 0.0, 1.0, // 35
    ];

    let vao = context
        .create_vertex_array()
        .ok_or("failed to create vertex array")?;
    context.bind_vertex_array(Some(&vao));

    let vbo = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

    // Chargement des données des sommets dans le buffer
    unsafe {
        let vertex_array = js_sys::Float32Array::view(&vertices);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let ebo = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));

    // Position de l'attribut du sommet (aPos)
    let position_attrib_location = context.get_attrib_location(&shader.program, "aPos") as u32;
    context.vertex_attrib_pointer_with_i32(
        position_attrib_location,      // Location
        3,                             // Nombre de composantes par sommet
        WebGl2RenderingContext::FLOAT, // Type
        false,                         // Normalized
        5 * 4,                         // Taille du sommet en octets
        0,                             // Décalage à partir du début du tampon
    );
    context.enable_vertex_attrib_array(position_attrib_location);

    // Handler texture
    let texture_attrib_location = context.get_attrib_location(&shader.program, "aTexCoord") as u32;
    context.vertex_attrib_pointer_with_i32(
        texture_attrib_location,       // Location
        2,                             // Nombre de composantes par sommet
        WebGl2RenderingContext::FLOAT, // Type
        false,                         // Normalized
        5 * 4,                         // Taille du sommet en octets
        3 * 4,                         // Offset from the beginning of the buffer
    );
    context.enable_vertex_attrib_array(texture_attrib_location);

    let texture = context.create_texture().ok_or("failed to create texture")?;
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_S,
        WebGl2RenderingContext::REPEAT as i32,
    );

    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_T,
        WebGl2RenderingContext::REPEAT as i32,
    );

    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
        WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
    );

    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
        WebGl2RenderingContext::LINEAR as i32,
    );

    let image_data = include_bytes!("../textures/wall.jpg");

    let img = ImageReader::new(std::io::Cursor::new(image_data))
        .with_guessed_format() // Devine le format (JPEG dans ce cas)
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    // Obtenir les dimensions de l'image
    let (width, height) = img.dimensions();

    // Convertir l'image en tableau RGBA
    let rgba = img.to_rgba8();

    // Créer l'objet `ImageData` attendu par WebGL
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&rgba), width, height)?;

    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_data(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGBA as i32,
        width as i32,
        height as i32,
        0,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &image_data,
    )?;

    context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

    // Add translation - init as identity
    let mut model = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let mut view = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let mut projection = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    model = glm::rotate(&model, f32::to_radians(45.0), &glm::vec3(1.0, 0.0, 0.0));
    view = glm::translate(&view, &glm::vec3(0.0, 0.0, -3.0));
    projection = glm::perspective(f32::to_radians(45.0), 600.0 / 600.0, 0.1, 100.0);

    let model_location = context.get_uniform_location(&shader.program, "model");
    context.uniform_matrix4fv_with_f32_array(model_location.as_ref(), false, &model.as_ref());

    let view_location = context.get_uniform_location(&shader.program, "view");
    context.uniform_matrix4fv_with_f32_array(view_location.as_ref(), false, &view.as_ref());

    let projection_location = context.get_uniform_location(&shader.program, "projection");
    context.uniform_matrix4fv_with_f32_array(
        projection_location.as_ref(),
        false,
        &projection.as_ref(),
    );

    // Enable depth test
    context.enable(WebGl2RenderingContext::DEPTH_TEST);

    return Ok(());
}
