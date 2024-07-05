use crate::examples::*;
use app_surface::{AppSurface, SurfaceFrame};

pub struct WgpuCanvas {
    pub app_surface: AppSurface,
    example: Box<dyn Example>,
}

#[allow(dead_code)]
impl WgpuCanvas {
    pub fn new(app_surface: AppSurface, idx: i32) -> Self {
        let example = Box::new(Empty::new(&app_surface));
        log::info!("example created");
        let mut instance = WgpuCanvas {
            app_surface,
            example,
        };
        instance.example = Box::new(Shapes::new(&instance.app_surface));

        if let Some(callback) = instance.app_surface.callback_to_app {
            callback(0);
        }
        instance
    }

    pub fn enter_frame(&mut self) {
        self.example.enter_frame(&self.app_surface);

        if let Some(_callback) = self.app_surface.callback_to_app {
            // callback(1);
        }
    }

    pub fn resize(&mut self) {
        self.app_surface.resize_surface();
    }
}
