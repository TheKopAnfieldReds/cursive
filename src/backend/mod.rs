//! Define backends using common libraries.
//!
//! Cursive doesn't print anything by itself: it delegates this job to a
//! backend library, which handles all actual input and output.
//!
//! This module defines the `Backend` trait, as well as a few implementations
//! using some common libraries. Each of those included backends needs a 
//! corresonding feature to be enabled.

use event;
use theme;

use vec::Vec2;

pub mod dummy;

pub mod termion;
pub mod blt;
pub mod curses;

/// Trait defining the required methods to be a backend.
pub trait Backend {
    // TODO: take `self` by value?
    // Or implement Drop?
    /// Prepares to close the backend.
    ///
    /// This should clear any state in the terminal.
    fn finish(&mut self);

    /// Refresh the screen.
    fn refresh(&mut self);

    /// Should return `true` if this backend supports colors.
    fn has_colors(&self) -> bool;

    /// Returns the screen size.
    fn screen_size(&self) -> Vec2;

    /// Main input method
    fn poll_event(&mut self) -> event::Event;

    /// Main method used for printing
    fn print_at(&self, pos: Vec2, text: &str);

    /// Clears the screen with the given color.
    fn clear(&self, color: theme::Color);

    /// Sets the refresh rate for the backend.
    ///
    /// If no event is detected in the interval, send an `Event::Refresh`.
    fn set_refresh_rate(&mut self, fps: u32);

    /// Starts using a new color.
    ///
    /// This should return the previously active color.
    fn set_color(&self, colors: theme::ColorPair) -> theme::ColorPair;

    /// Enables the given effect.
    fn set_effect(&self, effect: theme::Effect);


    /// Disables the given effect.
    fn unset_effect(&self, effect: theme::Effect);
}
