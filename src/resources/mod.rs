#[path = "mouse_movement.resource.rs"]
mod mouse_movement_resource;
#[path = "window_focused.resource.rs"]
mod window_focused_resource;

pub use self::{
    mouse_movement_resource::MouseMovementResource,
    window_focused_resource::WindowFocusedResource
};
