use app_surface::AppSurface;

pub trait Example {
    fn resize(&mut self, _app_surface: &AppSurface) {}
    fn enter_frame(&mut self, app_surface: &AppSurface);
}

pub struct Empty;
impl Empty {
    pub fn new(_app_surface: &AppSurface) -> Self {
        Empty {}
    }
}
impl Example for Empty {
    fn enter_frame(&mut self, _app_surface: &AppSurface) {}
}

mod shapes;
pub use shapes::Shapes;
