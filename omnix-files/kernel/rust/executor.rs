use crate::omxapk::OmxApp;

pub unsafe fn run_omx_app(app: &OmxApp<'static>) {
    let code_ptr = app.payload.as_ptr();
    let app_function: extern "C" fn() = core::mem::transmute(code_ptr);
    app_function();
}
