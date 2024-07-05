use app_surface::AppSurface;
use std::{num::NonZeroUsize, sync::Arc};
use vello::{
    kurbo::{Affine, Circle, Ellipse, Line, RoundedRect, Stroke, Vec2},
    peniko::{Blob, Color, Format, Image},
    util::{RenderContext, RenderSurface},
    AaConfig, Renderer, RendererOptions, Scene,
};

use super::Example;

pub struct Shapes {
    scene: Scene,
}

impl Shapes {
    pub fn new(app_surface: &AppSurface) -> Self {
        // let mut render_cx = RenderContext::new();

        // // An array of renderers, one per wgpu device
        // let mut renderers: Vec<Option<Renderer>> = vec![];

        //         let (width, height) = ...;
        // let device: wgpu::Device = ...;
        let device = &app_surface.device;
        let queue = &app_surface.queue;
        let surface = &app_surface.surface;
        // let queue: wgpu::Queue = ...;
        // let surface: wgpu::Surface<'_> = ...;
        // let texture_format: wgpu::TextureFormat = ...;
        let texture_format = app_surface.config.format;

        let mut renderer = Renderer::new(
            &device,
            RendererOptions {
                surface_format: Some(texture_format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: NonZeroUsize::new(1),
            },
        )
        .expect("Failed to create renderer");

        // // State for our example where we store the winit Window and the wgpu Surface
        // let mut render_state = RenderState::Suspended(None);

        // A vello Scene which is a data structure which allows one to build up a description a scene to be drawn
        // (with paths, fills, images, text, etc) which is then passed to a renderer for rendering
        let mut scene = Scene::new();

        // let window = unsafe { app_surface.view.as_ref().unwrap() };
        // let size = window.inner_size();
        // let surface_future = render_cx.create_surface(
        //     window.clone(),
        //     size.width,
        //     size.height,
        //     wgpu::PresentMode::AutoVsync,
        // );
        // let surface = pollster::block_on(surface_future).expect("Error creating surface");

        // Create a vello Renderer for the surface (using its device id)
        // renderers.resize_with(render_cx.devices.len(), || None);
        // renderers[surface.dev_id]
        //     .get_or_insert_with(|| create_vello_renderer(&render_cx, &surface));

        // Empty the scene of objects to draw. You could create a new Scene each time, but in this case
        // the same Scene is reused so that the underlying memory allocation can also be reused.
        scene.reset();

        // Re-add the objects to draw to the scene.
        add_shapes_to_scene(&mut scene);

        // Get the RenderSurface (surface + config)
        // let surface = &render_state.surface;

        // // Get the window size
        // let width = surface.config.width;
        // let height = surface.config.height;

        // // Get a handle to the device
        // let device_handle = &render_cx.devices[surface.dev_id];

        // Get the surface's texture
        let surface_texture = surface
            .get_current_texture()
            .expect("failed to get surface texture");

        let width = app_surface.config.width;
        let height = app_surface.config.height;

        renderer
            .render_to_surface(
                device,
                queue,
                &scene,
                &surface_texture,
                &vello::RenderParams {
                    base_color: Color::BLACK, // Background color
                    width,
                    height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("Failed to render to surface");
        surface_texture.present();

        // // Render to the surface's texture
        // renderers[surface.dev_id]
        //     .as_mut()
        //     .unwrap()
        //     .render_to_surface(
        //         &device_handle.device,
        //         &device_handle.queue,
        //         &scene,
        //         &surface_texture,
        //         &vello::RenderParams {
        //             base_color: Color::BLACK, // Background color
        //             width,
        //             height,
        //             antialiasing_method: AaConfig::Msaa16,
        //         },
        //     )
        //     .expect("failed to render to surface");

        // // Queue the texture to be presented on the surface
        // surface_texture.present();

        // device_handle.device.poll(wgpu::Maintain::Poll);

        // canvas.enter_frame();
        // canvas.app_surface.view.as_ref().unwrap().request_redraw();
        Shapes { scene }
    }
}
impl Example for Shapes {
    fn enter_frame(&mut self, _app_surface: &AppSurface) {}
}

/// Add shapes to a vello scene. This does not actually render the shapes, but adds them
/// to the Scene data structure which represents a set of objects to draw.
fn add_shapes_to_scene(scene: &mut Scene) {
    // Draw an outlined rectangle
    let stroke = Stroke::new(6.0);
    let rect = RoundedRect::new(10.0, 10.0, 240.0, 240.0, 20.0);
    let rect_stroke_color = Color::rgb(0.9804, 0.702, 0.5294);
    scene.stroke(&stroke, Affine::IDENTITY, rect_stroke_color, None, &rect);

    // Draw a filled circle
    let circle = Circle::new((420.0, 200.0), 120.0);
    let circle_fill_color = Color::rgb(0.9529, 0.5451, 0.6588);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        circle_fill_color,
        None,
        &circle,
    );

    // Draw a filled ellipse
    let ellipse = Ellipse::new((250.0, 420.0), (100.0, 160.0), -90.0);
    let ellipse_fill_color = Color::rgb(0.7961, 0.651, 0.9686);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        ellipse_fill_color,
        None,
        &ellipse,
    );

    // Draw a straight line
    let line = Line::new((260.0, 20.0), (620.0, 100.0));
    let line_stroke_color = Color::rgb(0.5373, 0.7059, 0.9804);
    scene.stroke(&stroke, Affine::IDENTITY, line_stroke_color, None, &line);

    let data = vec![0xff; 400];
    let data = Arc::new(data);
    let image = Image::new(Blob::new(data), Format::Rgba8, 10, 10);
    scene.draw_image(&image, Affine::translate(Vec2::new(128.0, 128.0)));
}

/// Helper function that creates a vello `Renderer` for a given `RenderContext` and `RenderSurface`
fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions {
            surface_format: Some(surface.format),
            use_cpu: false,
            antialiasing_support: vello::AaSupport::all(),
            num_init_threads: NonZeroUsize::new(1),
        },
    )
    .expect("Couldn't create renderer")
}
