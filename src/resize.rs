//! Window border resize functionality.
//!
//! This module provides mouse-based window resizing by creating invisible
//! border widgets that capture mouse events for resize operations.

use gdk4::{Cursor, SurfaceEdge};
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, DrawingArea, EventControllerMotion, GestureDrag, Overlay, Widget};

/// Border size in pixels for resize detection areas.
const BORDER_SIZE: i32 = 6;

/// Edge type for resize operations.
#[derive(Debug, Clone, Copy)]
pub enum Edge {
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Edge {
    fn to_surface_edge(self) -> SurfaceEdge {
        match self {
            Edge::South => SurfaceEdge::South,
            Edge::East => SurfaceEdge::East,
            Edge::West => SurfaceEdge::West,
            Edge::NorthEast => SurfaceEdge::NorthEast,
            Edge::NorthWest => SurfaceEdge::NorthWest,
            Edge::SouthEast => SurfaceEdge::SouthEast,
            Edge::SouthWest => SurfaceEdge::SouthWest,
        }
    }

    fn cursor_name(self) -> &'static str {
        match self {
            Edge::South => "ns-resize",
            Edge::East | Edge::West => "ew-resize",
            Edge::NorthEast | Edge::SouthWest => "nesw-resize",
            Edge::NorthWest | Edge::SouthEast => "nwse-resize",
        }
    }
}

/// Creates a resize border widget for the specified edge.
fn create_border_widget(window: &ApplicationWindow, edge: Edge) -> DrawingArea {
    let area = DrawingArea::new();

    // Configure size and alignment based on edge
    match edge {
        Edge::South => {
            area.set_height_request(BORDER_SIZE);
            area.set_hexpand(true);
            area.set_vexpand(false);
            area.set_valign(gtk4::Align::End);
        }
        Edge::East | Edge::West => {
            area.set_width_request(BORDER_SIZE);
            area.set_hexpand(false);
            area.set_vexpand(true);
            match edge {
                Edge::East => area.set_halign(gtk4::Align::End),
                Edge::West => area.set_halign(gtk4::Align::Start),
                _ => {}
            }
        }
        Edge::NorthWest | Edge::NorthEast | Edge::SouthWest | Edge::SouthEast => {
            area.set_width_request(BORDER_SIZE * 2);
            area.set_height_request(BORDER_SIZE * 2);
            area.set_hexpand(false);
            area.set_vexpand(false);
            match edge {
                Edge::NorthWest => {
                    area.set_halign(gtk4::Align::Start);
                    area.set_valign(gtk4::Align::Start);
                }
                Edge::NorthEast => {
                    area.set_halign(gtk4::Align::End);
                    area.set_valign(gtk4::Align::Start);
                }
                Edge::SouthWest => {
                    area.set_halign(gtk4::Align::Start);
                    area.set_valign(gtk4::Align::End);
                }
                Edge::SouthEast => {
                    area.set_halign(gtk4::Align::End);
                    area.set_valign(gtk4::Align::End);
                }
                _ => {}
            }
        }
    }

    // Setup cursor change on hover
    let motion = EventControllerMotion::new();
    let cursor_name = edge.cursor_name();

    motion.connect_enter(move |controller, _, _| {
        if let Some(widget) = controller.widget() {
            let cursor = Cursor::from_name(cursor_name, None);
            widget.set_cursor(cursor.as_ref());
        }
    });

    motion.connect_leave(move |controller| {
        if let Some(widget) = controller.widget() {
            widget.set_cursor(None);
        }
    });

    area.add_controller(motion);

    // Setup drag gesture for resize
    let drag = GestureDrag::new();
    drag.set_button(1);

    let window_weak = window.downgrade();
    drag.connect_drag_begin(move |gesture, x, y| {
        if let Some(window) = window_weak.upgrade() {
            if let Some(native) = window.native() {
                if let Some(surface) = native.surface() {
                    if let Ok(toplevel) = surface.clone().downcast::<gdk4::Toplevel>() {
                        // Get absolute coordinates
                        if let Some(widget) = gesture.widget() {
                            let (abs_x, abs_y) = widget.translate_coordinates(&window, x, y)
                                .unwrap_or((x, y));

                            toplevel.begin_resize(
                                edge.to_surface_edge(),
                                gesture.device().as_ref(),
                                1,
                                abs_x,
                                abs_y,
                                gdk4::CURRENT_TIME,
                            );

                            gesture.set_state(gtk4::EventSequenceState::Claimed);
                        }
                    }
                }
            }
        }
    });

    area.add_controller(drag);

    area
}

/// Creates an overlay with resize borders around the main content.
///
/// # Arguments
/// * `window` - The ApplicationWindow for resize operations
/// * `content` - The main content widget to wrap
///
/// # Returns
/// An Overlay widget containing the content with resize borders
pub fn create_resize_overlay(window: &ApplicationWindow, content: &impl IsA<Widget>) -> Overlay {
    let overlay = Overlay::new();
    overlay.set_child(Some(content));

    // Add border widgets for edges and corners
    // Note: North (top) edge resize is handled natively by the HeaderBar
    let edges = [
        Edge::South,
        Edge::East,
        Edge::West,
        Edge::NorthWest,
        Edge::NorthEast,
        Edge::SouthWest,
        Edge::SouthEast,
    ];

    for edge in edges {
        let border = create_border_widget(window, edge);
        overlay.add_overlay(&border);
    }

    overlay
}
