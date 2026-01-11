/// Window lifecycle events that occur between frames
///
/// Events accumulate in a `Vec<Event>` between frames and are passed to the
/// frame callback via `Egor::events`. The event vector is cleared after each
/// frame completes.
///
/// # Example
///
/// ```no_run
/// use egor::{app::App, egor::Egor, app::EgorEvent};
/// use egor::input::KeyCode;
///
/// let mut paused = false;
///
/// App::new().run(move |egor: &mut Egor, timer| {
///     // Handle window events
///     for event in &egor.events {
///         match event {
///             EgorEgorEvent::Focused(focused) => {
///                 paused = !focused;
///             },
///             EgorEgorEvent::Resized(w, h) => {
///                 println!("Window resized to {}x{}", w, h);
///             },
///             EgorEgorEvent::CloseRequested => {
///                 // Note: This event is added for API consistency but
///                 // you'll never see it because the app exits immediately.
///                 // Use App::on_quit() for cleanup instead.
///             }
///         }
///     }
///
///     if !paused {
///         // Game logic...
///     }
/// });
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EgorEvent {
    /// Window gained or lost focus
    ///
    /// Contains `true` when the window gains focus, `false` when it loses focus.
    /// Use this to pause gameplay, mute audio, or reduce rendering quality when unfocused.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::app::Event;
    /// # let events = vec![EgorEvent::Focused(true)];
    /// for event in &events {
    ///     if let EgorEvent::Focused(focused) = event {
    ///         if *focused {
    ///             println!("Window gained focus - resume game");
    ///         } else {
    ///             println!("Window lost focus - pause game");
    ///         }
    ///     }
    /// }
    /// ```
    Focused(bool),

    /// Window was resized
    ///
    /// Contains the new window dimensions in physical pixels (width, height).
    /// The renderer automatically handles resizing, but you may want to adjust
    /// UI layouts, camera bounds, or game state based on the new size.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use egor::app::Event;
    /// # let events = vec![EgorEvent::Resized(1920, 1080)];
    /// for event in &events {
    ///     if let EgorEvent::Resized(width, height) = event {
    ///         println!("Window resized to {}x{}", width, height);
    ///         // Adjust UI scale, camera bounds, etc.
    ///     }
    /// }
    /// ```
    Resized(u32, u32),

    /// Window close was requested (user clicked X button or pressed Alt+F4)
    ///
    /// **Note:** When this event occurs, you have one final frame to handle it
    /// before the application exits. This is your chance to save state, show a
    /// confirmation dialog, or perform cleanup. The application will exit
    /// automatically after this frame completes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use egor::{app::{App, EgorEvent}, egor::Egor};
    ///
    /// App::new().run(|egor: &mut Egor, timer| {
    ///     // Check for close request
    ///     for event in &egor.events {
    ///         if let EgorEvent::CloseRequested = event {
    ///             println!("Saving game state before exit...");
    ///             // Save game state, close connections, etc.
    ///             // App will exit after this frame
    ///         }
    ///     }
    /// });
    /// ```
    CloseRequested,
}
