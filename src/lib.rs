//! The static site genrator.

mod layout;
mod manifest;
mod post;

pub use self::{
    layout::Layout,
    manifest::Manifest,
    post::{Meta, Post},
};
