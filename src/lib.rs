//! # Rael OBJ
//! 
//! A simple OBJ file loader and renderer for the terminal.
//! 
//! This crate provides the core functionality to load a `.obj` model, and render it to a list of pixels
//! that can be drawn to a `rael::Canvas`.

// Re-export key types for easier use
pub use obj::{load_obj, Obj, TexturedVertex};
pub use rael::{Canvas, Color};

pub use crate::obj_load::draw_obj;
pub use crate::obj_load::light::{Light, LightKind};
pub use crate::obj_load::texture::Texture;

pub mod obj_load;