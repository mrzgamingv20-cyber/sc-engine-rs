//! Device-level render state and draw-call validation.
//!
//! The actual GPU work is delegated to a [`RenderBackend`], so the core stays
//! platform independent (an OpenGL ES backend is added in a later step).

use crate::math::{Color, Point2, Rectangle, Vector4};
use crate::graphics::enums::{IndexFormat, PrimitiveType};
use crate::graphics::states::{BlendState, DepthStencilState, RasterizerState};
use crate::graphics::vertex::{IndexBufferDescriptor, VertexBufferDescriptor, VertexDeclaration};
use crate::graphics::viewport::Viewport;

/// Backend that issues real GPU commands. Implemented by the platform layer.
pub trait RenderBackend: Send + Sync {
    fn clear(&mut self, color: Option<Vector4>, depth: Option<f32>, stencil: Option<i32>);
    fn draw_arrays(&mut self, primitive: PrimitiveType, start_vertex: i32, vertices_count: i32);
    fn draw_elements(
        &mut self,
        primitive: PrimitiveType,
        index_format: IndexFormat,
        start_index: i32,
        indices_count: i32,
    );
    fn apply_viewport(&mut self, viewport: &Viewport, scissor: &Rectangle, scissor_enabled: bool);
    fn apply_rasterizer(&mut self, state: &RasterizerState);
    fn apply_depth_stencil(&mut self, state: &DepthStencilState);
    fn apply_blend(&mut self, state: &BlendState);
    fn max_texture_size(&self) -> i32;
    fn device_description(&self) -> String;
}

/// Mirror of the C# `Display` static class: owns the current render state.
#[derive(Debug, Clone)]
pub struct Display {
    pub backbuffer_size: Point2,
    pub viewport: Viewport,
    pub scissor_rectangle: Rectangle,
    pub rasterizer_state: RasterizerState,
    pub depth_stencil_state: DepthStencilState,
    pub blend_state: BlendState,
    pub render_target_size: Option<Point2>,
    pub use_reduced_z_range: bool,
    pub device_description: String,
}

impl Default for Display {
    fn default() -> Self {
        Display {
            backbuffer_size: Point2::new(0, 0),
            viewport: Viewport::default(),
            scissor_rectangle: Rectangle::EMPTY,
            rasterizer_state: RasterizerState::cull_counter_clockwise(),
            depth_stencil_state: DepthStencilState::locked_default(),
            blend_state: BlendState::opaque(),
            render_target_size: None,
            use_reduced_z_range: false,
            device_description: String::new(),
        }
    }
}

impl Display {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resize backbuffer-derived viewport and scissor to the given size.
    pub fn resize(&mut self, size: Point2) {
        self.backbuffer_size = size;
        if self.render_target_size.is_none() {
            self.viewport = Viewport::new(0, 0, size.x, size.y);
            self.scissor_rectangle = Rectangle::new(0, 0, size.x, size.y);
        }
    }

    /// Bind a render target; `None` returns to the backbuffer.
    pub fn set_render_target(&mut self, size: Option<Point2>) {
        self.render_target_size = size;
        match size {
            Some(s) => {
                self.viewport = Viewport::new(0, 0, s.x, s.y);
                self.scissor_rectangle = Rectangle::new(0, 0, s.x, s.y);
            }
            None => {
                let s = self.backbuffer_size;
                self.viewport = Viewport::new(0, 0, s.x, s.y);
                self.scissor_rectangle = Rectangle::new(0, 0, s.x, s.y);
            }
        }
    }

    /// Total GPU memory in bytes: backbuffer (8 bytes/pixel) plus nothing else tracked here.
    pub fn gpu_memory_usage(&self) -> i64 {
        8 * self.backbuffer_size.x as i64 * self.backbuffer_size.y as i64
    }

    // --- Validation helpers -------------------------------------------------

    pub fn verify_draw(
        &self,
        vertex_buffer: &VertexBufferDescriptor,
        start_vertex: i32,
        vertices_count: i32,
    ) -> Result<(), &'static str> {
        if start_vertex < 0 || vertices_count < 0 {
            return Err("Vertices range is out of bounds.");
        }
        if start_vertex + vertices_count > vertex_buffer.vertices_count {
            return Err("Vertices range is out of bounds.");
        }
        Ok(())
    }

    pub fn verify_draw_indexed(
        &self,
        vertex_buffer: &VertexBufferDescriptor,
        index_buffer: &IndexBufferDescriptor,
        start_index: i32,
        indices_count: i32,
    ) -> Result<(), &'static str> {
        if start_index < 0 || indices_count < 0 {
            return Err("Indices range is out of bounds.");
        }
        if start_index + indices_count > index_buffer.indices_count {
            return Err("Indices range is out of bounds.");
        }
        self.verify_draw(vertex_buffer, 0, vertex_buffer.vertices_count)
    }

    /// Ensures the vertex stride is a whole multiple of the vertex struct size.
    pub fn verify_vertex_stride(declaration: &VertexDeclaration, element_size: i32) -> Result<(), String> {
        assert!(element_size > 0, "element size must be positive");
        let stride = declaration.vertex_stride;
        if stride / element_size * element_size != stride {
            return Err(format!(
                "Vertex is not an integer multiple of array element, vertex stride is {stride}, array element is {element_size}."
            ));
        }
        Ok(())
    }

    pub fn clear_color(&self, color: Color) -> Vector4 {
        color.to_vector4()
    }
}