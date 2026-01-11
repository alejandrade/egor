use egor_render::math::{Mat4, Rect, Vec2};

/// A basic camera for controlling view & projection
///
/// Useful for culling & rendering transformations
pub struct Camera {
    position: Vec2,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Camera {
    /// Returns the orthographic view-projection matrix for the current camera state
    pub(crate) fn view_proj(&self, screen_size: Vec2) -> Mat4 {
        let half_width = screen_size.x / 2.0 / self.zoom;
        let half_height = screen_size.y / 2.0 / self.zoom;

        let left = self.position.x - half_width;
        let right = self.position.x + half_width;
        let bottom = self.position.y - half_height;
        let top = self.position.y + half_height;

        Mat4::orthographic_lh(left, right, top, bottom, -1.0, 1.0)
    }

    /// Set the camera's target position (center of view).
    ///
    /// # Arguments
    ///
    /// * `position` - World-space coordinates for the camera to focus on
    ///
    /// # Note
    ///
    /// The camera's position determines what part of the world is visible on screen.
    /// The position represents the center of the view, not the top-left corner.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// // Follow a player character
    /// let player_pos = vec2(500.0, 300.0);
    /// gfx.camera().target(player_pos);
    /// # }
    /// ```
    pub fn target(&mut self, position: Vec2) {
        self.position = position;
    }

    /// Set the camera's zoom level.
    ///
    /// # Arguments
    ///
    /// * `zoom` - Zoom factor (1.0 = normal, 2.0 = zoomed in 2×, 0.5 = zoomed out 2×)
    ///
    /// # Note
    ///
    /// The zoom level is automatically clamped between 0.1 and 10.0 to prevent
    /// extreme values that could cause rendering issues or performance problems.
    ///
    /// Higher zoom values show less of the world (zoomed in), while lower values
    /// show more of the world (zoomed out).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # fn example(gfx: &mut Graphics) {
    /// // Zoom in 2x for a close-up view
    /// gfx.camera().set_zoom(2.0);
    ///
    /// // Zoom out to see more of the world
    /// gfx.camera().set_zoom(0.5);
    ///
    /// // Smooth zoom animation
    /// let target_zoom = 1.5;
    /// let current_zoom = 1.0;
    /// let new_zoom = current_zoom + (target_zoom - current_zoom) * 0.1;
    /// gfx.camera().set_zoom(new_zoom);
    /// # }
    /// ```
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 10.0);
    }

    /// Get the visible area in world coordinates.
    ///
    /// # Arguments
    ///
    /// * `screen_size` - The rendering surface size (from `gfx.screen_size()`)
    ///
    /// # Returns
    ///
    /// A [`Rect`] representing the visible area in world space.
    ///
    /// # Note
    ///
    /// This is useful for:
    /// - **Culling**: Skip drawing objects outside the viewport
    /// - **Visibility checks**: Determine if an object is on-screen
    /// - **UI bounds**: Position UI elements relative to visible area
    ///
    /// The returned rectangle's position is the top-left corner in world space,
    /// and the size accounts for the current zoom level.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// let screen_size = gfx.screen_size();
    /// let viewport = gfx.camera().viewport(screen_size);
    ///
    /// // Only draw objects within the viewport
    /// let objects = get_all_objects();
    /// for obj in objects {
    ///     if viewport.contains(obj.position) {
    ///         // Draw object
    ///     }
    /// }
    /// # }
    /// # fn get_all_objects() -> Vec<Object> { vec![] }
    /// # struct Object { position: egor::math::Vec2 }
    /// ```
    pub fn viewport(&self, screen_size: Vec2) -> Rect {
        let size = screen_size / self.zoom;
        // convert centre → top‑left
        let top_left = self.position - size * 0.5;

        Rect::new(top_left, size)
    }

    /// Convert a position from world space to screen space.
    ///
    /// # Arguments
    ///
    /// * `world` - A position in world coordinates
    /// * `screen_size` - The rendering surface size (from `gfx.screen_size()`)
    ///
    /// # Returns
    ///
    /// The corresponding position in screen space (pixel coordinates).
    ///
    /// # Note
    ///
    /// This is useful for:
    /// - **UI positioning**: Place UI elements above world objects
    /// - **Debug rendering**: Draw debug info at object positions
    /// - **Mouse interaction**: Convert world positions for cursor checks
    ///
    /// Screen space has (0,0) at the top-left corner of the window.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics) {
    /// // Draw health bar above a character
    /// let character_world_pos = vec2(500.0, 300.0);
    /// let screen_size = gfx.screen_size();
    /// let screen_pos = gfx.camera().world_to_screen(character_world_pos, screen_size);
    ///
    /// gfx.text("❤️ 100%")
    ///     .at(screen_pos + vec2(-20.0, -30.0))
    ///     .size(16.0);
    /// # }
    /// ```
    pub fn world_to_screen(&self, world: Vec2, screen_size: Vec2) -> Vec2 {
        (world - self.position) * self.zoom + (screen_size / 2.0)
    }

    /// Convert a position from screen space to world space.
    ///
    /// # Arguments
    ///
    /// * `screen` - A position in screen coordinates (pixels)
    /// * `screen_size` - The rendering surface size (from `gfx.screen_size()`)
    ///
    /// # Returns
    ///
    /// The corresponding position in world space.
    ///
    /// # Note
    ///
    /// This is useful for:
    /// - **Mouse/touch interaction**: Convert cursor position to world coordinates
    /// - **Click detection**: Determine what object was clicked
    /// - **Camera controls**: Pan the camera based on mouse drag
    ///
    /// Screen space has (0,0) at the top-left corner of the window.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::render::Graphics;
    /// # use egor::input::Input;
    /// # use egor::math::vec2;
    /// # fn example(gfx: &mut Graphics, input: &Input) {
    /// // Get world position of mouse cursor
    /// let mouse_screen = vec2(input.mouse_position().0, input.mouse_position().1);
    /// let screen_size = gfx.screen_size();
    /// let mouse_world = gfx.camera().screen_to_world(mouse_screen, screen_size);
    ///
    /// // Check if mouse is over an object
    /// let object_pos = vec2(500.0, 300.0);
    /// let distance = (mouse_world - object_pos).length();
    /// if distance < 50.0 {
    ///     // Mouse is hovering over object
    /// }
    /// # }
    /// ```
    pub fn screen_to_world(&self, screen: Vec2, screen_size: Vec2) -> Vec2 {
        (screen - screen_size / 2.0) / self.zoom + self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egor_render::math::vec2;

    #[test]
    fn view_proj_matrix() {
        // check that the camera's view-projection matrix matches expected ortho math
        let mut cam = Camera::default();
        cam.target(vec2(0.0, 0.0));
        cam.set_zoom(1.0);

        let mat = cam.view_proj(vec2(800.0, 600.0));
        let expected = Mat4::orthographic_lh(-400.0, 400.0, 300.0, -300.0, -1.0, 1.0);
        assert_eq!(mat, expected);
    }

    #[test]
    fn viewport_rect() {
        // check that viewport is centered on camera & scales correctly with zoom
        let mut cam = Camera::default();
        cam.target(vec2(50.0, 50.0));
        cam.set_zoom(2.0);

        let rect = cam.viewport(vec2(200.0, 100.0));
        assert_eq!(rect.position, vec2(0.0, 25.0));
        assert!((rect.size - vec2(100.0, 50.0)).length() < 0.001); // allow for float fuzz
    }

    #[test]
    fn world_screen_round_trip() {
        // converting world -> screen -> world should come back to where we started
        let mut cam = Camera::default();
        cam.target(vec2(100.0, 50.0));
        cam.set_zoom(2.0);

        let screen_size = vec2(800.0, 600.0);
        let world = vec2(110.0, 55.0);
        let screen = cam.world_to_screen(world, screen_size);
        let world2 = cam.screen_to_world(screen, screen_size);

        assert!((world - world2).length() < 0.001);
    }
}
