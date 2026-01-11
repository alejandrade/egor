use crate::graphics::Graphics;
use egor_app::{input::Input, EgorEvent};

/// Main interface for frame updates, containing graphics, input, and window events
///
/// `Egor` provides access to everything you need in a frame callback:
/// - `gfx` - Drawing operations (rectangles, textures, text, camera)
/// - `input` - Keyboard, mouse, and touch input state
/// - `events` - Window lifecycle events (focus, resize, close) accumulated since last frame
///
/// # Lifetime
///
/// The lifetime `'a` represents the frame scope. All borrows are valid for the
/// duration of the frame callback.
///
/// # Example
///
/// ```no_run
/// use egor::{
///     app::{App, EgorEvent},
///     egor::Egor,
///     input::KeyCode,
///     math::{vec2, Vec2},
///     render::Color,
/// };
///
/// let mut paused = false;
///
/// App::new().run(move |egor: &mut Egor, timer| {
///     // Check for window events
///     for event in &egor.events {
///         match event {
///             EgorEvent::Focused(focused) => {
///                 paused = !focused;
///                 println!("Window focus: {}", focused);
///             },
///             EgorEvent::Resized(w, h) => {
///                 println!("Window resized: {}x{}", w, h);
///             },
///             _ => {}
///         }
///     }
///
///     // Access input
///     if egor.input.key_pressed(KeyCode::Space) {
///         println!("Jump!");
///     }
///
///     // Draw
///     if !paused {
///         egor.gfx.clear(Color::BLACK);
///         egor.gfx.rect()
///             .at(vec2(100.0, 100.0))
///             .size(Vec2::splat(50.0))
///             .color(Color::RED);
///     }
/// });
/// ```
pub struct Egor<'a> {
    /// Graphics interface for 2D drawing operations
    ///
    /// Provides methods for drawing rectangles, textures, text, and managing the camera.
    /// All draw calls are batched and rendered efficiently at the end of the frame.
    pub gfx: Graphics<'a>,

    /// Input state for keyboard, mouse, and touch
    ///
    /// Provides methods to query key presses, mouse buttons, cursor position, and touch events.
    /// Input state is automatically updated between frames.
    pub input: &'a Input,

    /// Window events that occurred since the last frame
    ///
    /// This vector accumulates events as they occur and is cleared at the start of each frame.
    /// Events are processed in the order they occurred.
    ///
    /// Typically empty (most frames have no window events). Common events include:
    /// - `EgorEvent::Focused` - Window gained/lost focus
    /// - `EgorEvent::Resized` - Window dimensions changed
    pub events: Vec<EgorEvent>,
}
