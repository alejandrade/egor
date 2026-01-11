/// Clamp dimensions to max limits while maintaining aspect ratio.
///
/// Returns the clamped width and height. If the dimensions don't exceed the limits,
/// returns them unchanged. Otherwise, scales them down proportionally.
///
/// # Arguments
///
/// * `width` - Desired width (must be non-zero)
/// * `height` - Desired height (must be non-zero)
/// * `max_width` - Optional maximum width (from config)
/// * `max_height` - Optional maximum height (from config)
/// * `hw_max` - Hardware maximum texture size
///
/// # Returns
///
/// `(clamped_width, clamped_height)`
pub fn clamp_dimensions(
    width: u32,
    height: u32,
    max_width: Option<u32>,
    max_height: Option<u32>,
    hw_max: u32,
) -> (u32, u32) {
    let effective_max_width = max_width.unwrap_or(hw_max).min(hw_max);
    let effective_max_height = max_height.unwrap_or(hw_max).min(hw_max);

    if width > effective_max_width || height > effective_max_height {
        let scale = (effective_max_width as f32 / width as f32)
            .min(effective_max_height as f32 / height as f32);
        (
            (width as f32 * scale) as u32,
            (height as f32 * scale) as u32,
        )
    } else {
        (width, height)
    }
}

/// Query the maximum texture size supported by WebGL/GPU
#[cfg(target_arch = "wasm32")]
pub fn query_max_texture_size() -> u32 {
    use wasm_bindgen::JsCast;

    // Try to get the max texture size from WebGL
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Ok(Some(canvas)) = document.query_selector("canvas") {
                if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
                    // Try WebGL 2 first
                    if let Ok(Some(context)) = canvas.get_context("webgl2") {
                        if let Ok(gl) = context.dyn_into::<web_sys::WebGl2RenderingContext>() {
                            if let Ok(value) = gl.get_parameter(
                                web_sys::WebGl2RenderingContext::MAX_TEXTURE_SIZE,
                            ) {
                                if let Some(size) = value.as_f64() {
                                    return size as u32;
                                }
                            }
                        }
                    }
                    // Fall back to WebGL 1
                    if let Ok(Some(context)) = canvas.get_context("webgl") {
                        if let Ok(gl) = context.dyn_into::<web_sys::WebGlRenderingContext>() {
                            if let Ok(value) = gl
                                .get_parameter(web_sys::WebGlRenderingContext::MAX_TEXTURE_SIZE)
                            {
                                if let Some(size) = value.as_f64() {
                                    return size as u32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Safe fallback: WebGL 1.0 minimum spec
    2048
}

/// Query the maximum texture size supported by WebGL/GPU
#[cfg(not(target_arch = "wasm32"))]
pub fn query_max_texture_size() -> u32 {
    u32::MAX
}

