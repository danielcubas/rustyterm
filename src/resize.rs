//! Window border resize functionality.
//!
//! This module provides mouse-based window resizing by detecting when the cursor
//! is near window borders and enabling drag-to-resize behavior.

use gdk4::{Cursor, SurfaceEdge};
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, EventControllerMotion, GestureDrag};
use std::cell::Cell;
use std::rc::Rc;

/// Margin in pixels from the window edge that triggers resize behavior.
const RESIZE_MARGIN: f64 = 8.0;

/// Represents which edge or corner of the window the cursor is near.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeEdge {
    None,
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl ResizeEdge {
    /// Converts the resize edge to a GDK SurfaceEdge for native resize operations.
    pub fn to_surface_edge(self) -> Option<SurfaceEdge> {
        match self {
            ResizeEdge::None => None,
            ResizeEdge::North => Some(SurfaceEdge::North),
            ResizeEdge::South => Some(SurfaceEdge::South),
            ResizeEdge::East => Some(SurfaceEdge::East),
            ResizeEdge::West => Some(SurfaceEdge::West),
            ResizeEdge::NorthEast => Some(SurfaceEdge::NorthEast),
            ResizeEdge::NorthWest => Some(SurfaceEdge::NorthWest),
            ResizeEdge::SouthEast => Some(SurfaceEdge::SouthEast),
            ResizeEdge::SouthWest => Some(SurfaceEdge::SouthWest),
        }
    }

    /// Returns the appropriate cursor name for this edge.
    pub fn cursor_name(self) -> &'static str {
        match self {
            ResizeEdge::None => "default",
            ResizeEdge::North | ResizeEdge::South => "ns-resize",
            ResizeEdge::East | ResizeEdge::West => "ew-resize",
            ResizeEdge::NorthEast | ResizeEdge::SouthWest => "nesw-resize",
            ResizeEdge::NorthWest | ResizeEdge::SouthEast => "nwse-resize",
        }
    }
}

/// Detects which edge of the window the cursor is near based on coordinates.
///
/// # Arguments
/// * `x` - X coordinate of the cursor relative to the window
/// * `y` - Y coordinate of the cursor relative to the window
/// * `width` - Current window width
/// * `height` - Current window height
///
/// # Returns
/// The `ResizeEdge` that the cursor is near, or `ResizeEdge::None` if not near any edge.
pub fn detect_edge(x: f64, y: f64, width: i32, height: i32) -> ResizeEdge {
    let near_left = x < RESIZE_MARGIN;
    let near_right = x > (width as f64 - RESIZE_MARGIN);
    let near_top = y < RESIZE_MARGIN;
    let near_bottom = y > (height as f64 - RESIZE_MARGIN);

    match (near_left, near_right, near_top, near_bottom) {
        (true, false, true, false) => ResizeEdge::NorthWest,
        (false, true, true, false) => ResizeEdge::NorthEast,
        (true, false, false, true) => ResizeEdge::SouthWest,
        (false, true, false, true) => ResizeEdge::SouthEast,
        (true, false, false, false) => ResizeEdge::West,
        (false, true, false, false) => ResizeEdge::East,
        (false, false, true, false) => ResizeEdge::North,
        (false, false, false, true) => ResizeEdge::South,
        _ => ResizeEdge::None,
    }
}

/// Sets up event controllers for window border resize functionality.
///
/// This function attaches motion and drag gesture controllers to the window
/// to enable resize behavior when the user hovers over or drags window borders.
///
/// # Arguments
/// * `window` - The ApplicationWindow to add resize handlers to
pub fn setup_resize_handlers(window: &ApplicationWindow) {
    let current_edge: Rc<Cell<ResizeEdge>> = Rc::new(Cell::new(ResizeEdge::None));

    // Motion controller to detect edge proximity and change cursor
    let motion_controller = EventControllerMotion::new();
    let window_weak = window.downgrade();
    let edge_clone = current_edge.clone();

    motion_controller.connect_motion(move |_, x, y| {
        if let Some(window) = window_weak.upgrade() {
            let width = window.width();
            let height = window.height();
            let edge = detect_edge(x, y, width, height);
            edge_clone.set(edge);

            // Update cursor based on edge
            let cursor_name = edge.cursor_name();
            let cursor = Cursor::from_name(cursor_name, None);
            window.set_cursor(cursor.as_ref());
        }
    });

    // Reset cursor when leaving window
    let window_weak = window.downgrade();
    motion_controller.connect_leave(move |_| {
        if let Some(window) = window_weak.upgrade() {
            window.set_cursor(None);
        }
    });

    // Drag gesture for resize
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(1); // Left mouse button

    let window_weak = window.downgrade();
    let edge_clone = current_edge.clone();

    drag_gesture.connect_drag_begin(move |gesture, start_x, start_y| {
        if let Some(window) = window_weak.upgrade() {
            let edge = edge_clone.get();
            if let Some(surface_edge) = edge.to_surface_edge() {
                // Get the native surface and initiate resize
                if let Some(native) = window.native() {
                    if let Some(surface) = native.surface() {
                        if let Ok(toplevel) = surface.clone().downcast::<gdk4::Toplevel>() {
                            if let Some(device) = gesture.device() {
                                toplevel.begin_resize(
                                    surface_edge,
                                    Some(&device),
                                    1, // button
                                    start_x,
                                    start_y,
                                    gdk4::CURRENT_TIME,
                                );

                                // Claim the gesture sequence
                                gesture.set_state(gtk4::EventSequenceState::Claimed);
                            }
                        }
                    }
                }
            }
        }
    });

    window.add_controller(motion_controller);
    window.add_controller(drag_gesture);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_edge_corners() {
        // Top-left corner
        assert_eq!(detect_edge(2.0, 2.0, 800, 600), ResizeEdge::NorthWest);

        // Top-right corner
        assert_eq!(detect_edge(798.0, 2.0, 800, 600), ResizeEdge::NorthEast);

        // Bottom-left corner
        assert_eq!(detect_edge(2.0, 598.0, 800, 600), ResizeEdge::SouthWest);

        // Bottom-right corner
        assert_eq!(detect_edge(798.0, 598.0, 800, 600), ResizeEdge::SouthEast);
    }

    #[test]
    fn test_detect_edge_sides() {
        // Left edge
        assert_eq!(detect_edge(2.0, 300.0, 800, 600), ResizeEdge::West);

        // Right edge
        assert_eq!(detect_edge(798.0, 300.0, 800, 600), ResizeEdge::East);

        // Top edge
        assert_eq!(detect_edge(400.0, 2.0, 800, 600), ResizeEdge::North);

        // Bottom edge
        assert_eq!(detect_edge(400.0, 598.0, 800, 600), ResizeEdge::South);
    }

    #[test]
    fn test_detect_edge_center() {
        assert_eq!(detect_edge(400.0, 300.0, 800, 600), ResizeEdge::None);
    }

    #[test]
    fn test_cursor_names() {
        assert_eq!(ResizeEdge::None.cursor_name(), "default");
        assert_eq!(ResizeEdge::North.cursor_name(), "ns-resize");
        assert_eq!(ResizeEdge::South.cursor_name(), "ns-resize");
        assert_eq!(ResizeEdge::East.cursor_name(), "ew-resize");
        assert_eq!(ResizeEdge::West.cursor_name(), "ew-resize");
        assert_eq!(ResizeEdge::NorthEast.cursor_name(), "nesw-resize");
        assert_eq!(ResizeEdge::SouthWest.cursor_name(), "nesw-resize");
        assert_eq!(ResizeEdge::NorthWest.cursor_name(), "nwse-resize");
        assert_eq!(ResizeEdge::SouthEast.cursor_name(), "nwse-resize");
    }
}
