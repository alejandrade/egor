/// Information about the canvas for coordinate conversion
#[derive(Debug, Clone, Copy)]
pub struct CanvasInfo {
    pub css_width: f32,
    pub css_height: f32,
    pub buffer_width: f32,
    pub buffer_height: f32,
}

/// Converts window coordinates (from winit) to buffer coordinates
/// Handles DPI scaling, canvas offset, and CSS-to-buffer scaling
#[derive(Clone, Copy, PartialEq)]
pub struct CoordinateConverter {
    css_to_buffer_scale_x: f32,
    css_to_buffer_scale_y: f32,
    dpr: f32,
}

impl CoordinateConverter {
    /// Create a new coordinate converter with canvas information
    pub fn new(canvas_info: CanvasInfo, dpr: f32) -> Self {
        Self {
            css_to_buffer_scale_x: canvas_info.buffer_width / canvas_info.css_width,
            css_to_buffer_scale_y: canvas_info.buffer_height / canvas_info.css_height,
            dpr,
        }
    }

    /// Convert window coordinates (from winit) to buffer coordinates
    pub fn window_to_buffer(&self, window_x: f32, window_y: f32) -> (f32, f32) {
        if self.dpr == 1.0 {
            // No DPI scaling needed - coordinates are already correct
            (window_x, window_y)
        } else {
            // Convert physical pixels to CSS pixels
            let css_x = window_x / self.dpr;
            let css_y = window_y / self.dpr;

            // Scale to buffer coordinates
            (
                css_x * self.css_to_buffer_scale_x,
                css_y * self.css_to_buffer_scale_y,
            )
        }
    }
}

impl Default for CoordinateConverter {
    /// Default converter that does no conversion (pass-through)
    fn default() -> Self {
        Self {
            css_to_buffer_scale_x: 1.0,
            css_to_buffer_scale_y: 1.0,
            dpr: 1.0,
        }
    }
}
