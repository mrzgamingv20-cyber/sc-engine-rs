//! Device-independent vertex and index buffer resource descriptors.

use super::enums::{IndexFormat, VertexElementFormat, VertexElementSemantic};

/// One attribute of a vertex layout.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexElement {
    pub offset: i32,
    pub format: VertexElementFormat,
    pub semantic: String,
    pub semantic_name: String,
    pub semantic_index: i32,
}

impl VertexElement {
    pub fn new(format: VertexElementFormat, semantic: &str) -> Self {
        Self::with_offset(-1, format, semantic)
    }

    pub fn with_offset(offset: i32, format: VertexElementFormat, semantic: &str) -> Self {
        assert!(!semantic.is_empty(), "semantic cannot be empty or null.");

        // Split trailing digits into semantic_index, mirroring the C# implementation.
        let mut split = semantic.len();
        while split > 0 && semantic.as_bytes()[split - 1].is_ascii_digit() {
            split -= 1;
        }
        assert!(split > 0, "semantic cannot start with a digit.");

        let semantic_name = semantic[..split].to_string();
        let semantic_index = if split < semantic.len() {
            semantic[split..].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        VertexElement {
            offset,
            format,
            semantic: semantic.to_string(),
            semantic_name,
            semantic_index,
        }
    }

    pub fn from_enum(offset: i32, format: VertexElementFormat, semantic: VertexElementSemantic) -> Self {
        Self::with_offset(offset, format, semantic.semantic_string())
    }

    pub fn size(&self) -> i32 {
        self.format.size()
    }
}

/// Describes the memory layout of a vertex buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexDeclaration {
    pub elements: Vec<VertexElement>,
    pub vertex_stride: i32,
}

impl VertexDeclaration {
    pub fn new(elements: Vec<VertexElement>) -> Self {
        assert!(!elements.is_empty(), "There must be at least one VertexElement.");

        let mut elements = elements;
        let mut stride = 0i32;
        for element in elements.iter_mut() {
            if element.offset < 0 {
                element.offset = stride;
            }
            stride = stride.max(element.offset + element.format.size());
        }

        VertexDeclaration { elements, vertex_stride: stride }
    }

    pub fn vertex_elements(&self) -> &[VertexElement] {
        &self.elements
    }
}

/// Position + packed RGBA color vertex.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VertexPositionColor {
    pub position: crate::math::Vector3,
    pub color: crate::math::Color,
}

impl VertexPositionColor {
    pub fn new(position: crate::math::Vector3, color: crate::math::Color) -> Self {
        VertexPositionColor { position, color }
    }

    pub fn declaration() -> VertexDeclaration {
        VertexDeclaration::new(vec![
            VertexElement::with_offset(0, VertexElementFormat::Vector3, "POSITION"),
            VertexElement::with_offset(12, VertexElementFormat::NormalizedByte4, "COLOR"),
        ])
    }
}

/// Position + packed RGBA color + texture coordinate vertex.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VertexPositionColorTexture {
    pub position: crate::math::Vector3,
    pub color: crate::math::Color,
    pub tex_coord: crate::math::Vector2,
}

impl VertexPositionColorTexture {
    pub fn new(
        position: crate::math::Vector3,
        color: crate::math::Color,
        tex_coord: crate::math::Vector2,
    ) -> Self {
        VertexPositionColorTexture { position, color, tex_coord }
    }

    pub fn declaration() -> VertexDeclaration {
        VertexDeclaration::new(vec![
            VertexElement::with_offset(0, VertexElementFormat::Vector3, "POSITION"),
            VertexElement::with_offset(12, VertexElementFormat::NormalizedByte4, "COLOR"),
            VertexElement::with_offset(16, VertexElementFormat::Vector2, "TEXCOORD"),
        ])
    }
}

/// Index buffer descriptor (GPU handles live in the backend layer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IndexBufferDescriptor {
    pub index_format: IndexFormat,
    pub indices_count: i32,
}

/// Vertex buffer descriptor (GPU handles live in the backend layer).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VertexBufferDescriptor {
    pub declaration: Option<VertexDeclaration>,
    pub vertices_count: i32,
}