use crate::wgpu_canvas::WgpuCanvas;
use app_surface::{AppSurface, IOSViewObj};

#[no_mangle]
pub fn create_wgpu_canvas(ios_obj: IOSViewObj) -> *mut libc::c_void {
    println!(
        "create_wgpu_canvas, maximum frames: {}",
        ios_obj.maximum_frames
    );
    let obj = WgpuCanvas::new(AppSurface::new(ios_obj));
    let box_obj = Box::new(obj);
    Box::into_raw(box_obj) as *mut libc::c_void
}

#[no_mangle]
pub fn enter_frame(obj: *mut libc::c_void) {
    let obj = unsafe { &mut *(obj as *mut WgpuCanvas) };
    obj.enter_frame();
}
