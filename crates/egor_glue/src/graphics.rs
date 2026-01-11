use egor_render::{GeometryBatch, Renderer, math::Vec2};

use crate::{
    camera::Camera,
    color::Color,
    primitives::{PolygonBuilder, PrimitiveBatch, RectangleBuilder},
    text::{TextBuilder, TextRenderer},
};

/// High-level 2D drawing interface that simplifies the [`Renderer`]
pub struct Graphics<'a> {
    renderer: &'a mut Renderer,
    batch: PrimitiveBatch,
    camera: Camera,
    text_renderer: &'a mut TextRenderer,
}

impl<'a> Graphics<'a> {
    /// Upload camera matrix & extract batched geometry
    pub(crate) fn flush(&mut self) -> Vec<(usize, GeometryBatch)> {
        self.renderer
            .upload_camera_matrix(self.camera.view_proj(self.renderer.surface_size().into()));
        self.batch.take()
    }

    /// Create a new `Graphics` instance.
    ///
    /// # Arguments
    ///
    /// * `renderer` - Mutable reference to the underlying renderer
    /// * `text_renderer` - Mutable reference to the text rendering system
    ///
    /// # Returns
    ///
    /// A new `Graphics` instance with default camera and empty batch.
    ///
    /// # Note
    ///
    /// This is typically called internally by the framework. Users interact with `Graphics`
    /// through the closure passed to `App::run()`.
    pub fn new(renderer: &'a mut Renderer, text_renderer: &'a mut TextRenderer) -> Self {
        Self {
            renderer,
            batch: PrimitiveBatch::default(),
            camera: Camera::default(),
            text_renderer,
        }
    }

    /// Start building a rectangle.
    ///
    /// # Returns
    ///
    /// A [`RectangleBuilder`] that allows chaining methods to configure the rectangle's
    /// position, size, color, texture, and other properties.
    ///
    /// # Note
    ///
    /// The rectangle is automatically drawn when the builder is dropped (no explicit `build()` needed).
    /// All rectangles are batched and rendered efficiently in a single draw call per texture.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// // Draw a simple rectangle at origin
    /// gfx.rect();
    ///
    /// // Draw a red 100×50 rectangle at (200, 100)
    /// gfx.rect()
    ///     .at(vec2(200.0, 100.0))
    ///     .size(vec2(100.0, 50.0))
    ///     .color([1.0, 0.0, 0.0, 1.0]);
    /// # }
    /// ```
    pub fn rect(&mut self) -> RectangleBuilder<'_> {
        RectangleBuilder::new(&mut self.batch)
    }

    /// Start building an arbitrary polygon primitive, capable of triangles, circles, n-gons
    pub fn polygon(&mut self) -> PolygonBuilder<'_> {
        PolygonBuilder::new(&mut self.batch)
    }


    /// Set the screen clear color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to clear the screen to each frame
    ///
    /// # Note
    ///
    /// This affects the background color visible when no geometry is drawn.
    /// The clear happens automatically at the start of each frame.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::{Graphics, Color};
    /// # fn example(gfx: &mut Graphics) {
    /// // Clear to black
    /// gfx.clear(Color::BLACK);
    ///
    /// // Clear to dark blue
    /// gfx.clear(Color::new([0.1, 0.1, 0.3, 1.0]));
    /// # }
    /// ```
    pub fn clear(&mut self, color: Color) {
        self.renderer.set_clear_color(color.into());
    }

    /// Get the current rendering surface size in pixels.
    ///
    /// # Returns
    ///
    /// A [`Vec2`] containing the width (x) and height (y) of the rendering surface.
    ///
    /// # Note
    ///
    /// On WASM builds with `max_surface_size` configured, this may be smaller than
    /// the actual window size. The surface is automatically scaled to fit the window.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// let screen = gfx.screen_size();
    /// println!("Screen: {}×{}", screen.x, screen.y);
    ///
    /// // Center a rectangle on screen
    /// gfx.rect()
    ///     .at(screen / 2.0)
    ///     .size(vec2(100.0, 100.0));
    /// # }
    /// ```
    pub fn screen_size(&self) -> Vec2 {
        self.renderer.surface_size().into()
    }

    /// Get mutable access to the camera.
    ///
    /// # Returns
    ///
    /// A mutable reference to the [`Camera`], allowing you to control the view.
    ///
    /// # Note
    ///
    /// The camera controls the view-projection matrix used for rendering.
    /// Changes to the camera (position, zoom) affect all subsequently drawn geometry.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// // Center camera on world position (500, 300)
    /// gfx.camera().target(vec2(500.0, 300.0));
    ///
    /// // Zoom in 2x
    /// gfx.camera().set_zoom(2.0);
    /// # }
    /// ```
    pub fn camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Start building a text string to draw.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content to display
    ///
    /// # Returns
    ///
    /// A [`TextBuilder`] that allows chaining methods to configure the text's
    /// position, size, color, and alignment.
    ///
    /// # Note
    ///
    /// Text rendering uses signed distance fields (SDF) for crisp rendering at any scale.
    /// The text is automatically drawn when the builder is dropped.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::{Graphics, Color};
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// // Draw "Hello World" at the top-left
    /// gfx.text("Hello World")
    ///     .at(vec2(10.0, 10.0))
    ///     .size(24.0);
    ///
    /// // Draw red score text
    /// gfx.text("Score: 1000")
    ///     .at(vec2(100.0, 50.0))
    ///     .size(32.0)
    ///     .color(Color::new([1.0, 0.0, 0.0, 1.0]));
    /// # }
    /// ```
    pub fn text(&mut self, text: &str) -> TextBuilder<'_> {
        TextBuilder::new(self.text_renderer, text.to_string())
    }

    /// Load a texture from image data.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw image bytes (PNG, JPEG, etc.)
    ///
    /// # Returns
    ///
    /// A texture ID (usize) that can be used with `.texture(id)` on primitives.
    ///
    /// # Note
    ///
    /// Texture loading is relatively expensive, so it's best to load textures once
    /// during initialization (e.g., when `timer.frame == 0`) and reuse the IDs.
    ///
    /// The texture format is automatically detected from the image data.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// // Load texture once (typically at startup)
    /// let sprite_texture = gfx.load_texture(include_bytes!("sprite.png"));
    ///
    /// // Use texture when drawing
    /// gfx.rect()
    ///     .at(vec2(100.0, 100.0))
    ///     .size(vec2(64.0, 64.0))
    ///     .texture(sprite_texture);
    /// # }
    /// ```
    pub fn load_texture(&mut self, data: &[u8]) -> usize {
        self.renderer.add_texture(data)
    }

    /// Update an existing texture with new image data.
    ///
    /// # Arguments
    ///
    /// * `index` - The texture ID returned from [`load_texture`](Self::load_texture)
    /// * `data` - New image bytes (PNG, JPEG, etc.)
    ///
    /// # Note
    ///
    /// The new image data must have the same dimensions as the original texture,
    /// or the texture will be recreated with the new dimensions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # fn example(gfx: &mut Graphics, texture_id: usize) {
    /// // Update animated texture
    /// let frame_data = get_animation_frame();
    /// gfx.update_texture(texture_id, &frame_data);
    /// # }
    /// # fn get_animation_frame() -> Vec<u8> { vec![] }
    /// ```
    pub fn update_texture(&mut self, index: usize, data: &[u8]) {
        self.renderer.update_texture(index, data);
    }

    /// Update an existing texture with raw RGBA pixel data.
    ///
    /// # Arguments
    ///
    /// * `index` - The texture ID returned from [`load_texture`](Self::load_texture)
    /// * `w` - Width of the texture in pixels
    /// * `h` - Height of the texture in pixels
    /// * `data` - Raw RGBA8 pixel data (4 bytes per pixel: red, green, blue, alpha)
    ///
    /// # Note
    ///
    /// The data buffer must contain exactly `w × h × 4` bytes.
    /// Pixels are in row-major order, starting from the top-left.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # fn example(gfx: &mut Graphics, texture_id: usize) {
    /// // Create 2×2 red texture
    /// let pixels: Vec<u8> = vec![
    ///     255, 0, 0, 255,  // top-left: red
    ///     255, 0, 0, 255,  // top-right: red
    ///     255, 0, 0, 255,  // bottom-left: red
    ///     255, 0, 0, 255,  // bottom-right: red
    /// ];
    /// gfx.update_texture_raw(texture_id, 2, 2, &pixels);
    /// # }
    /// ```
    pub fn update_texture_raw(&mut self, index: usize, w: u32, h: u32, data: &[u8]) {
        self.renderer.update_texture_raw(index, w, h, data);
    }
}
