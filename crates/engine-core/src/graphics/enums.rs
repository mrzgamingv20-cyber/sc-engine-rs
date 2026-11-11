/// Blend factors for alpha/color blending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum Blend {
    #[default]
    Zero = 0,
    One = 1,
    SourceColor = 2,
    InverseSourceColor = 3,
    DestinationColor = 4,
    InverseDestinationColor = 5,
    SourceAlpha = 6,
    InverseSourceAlpha = 7,
    DestinationAlpha = 8,
    InverseDestinationAlpha = 9,
    BlendFactor = 10,
    InverseBlendFactor = 11,
    SourceAlphaSaturation = 12,
}

/// Blend operation applied to source and destination values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum BlendFunction {
    #[default]
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
}

/// Depth/stencil comparison function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum CompareFunction {
    #[default]
    Always = 0,
    Never = 1,
    Less = 2,
    LessEqual = 3,
    Equal = 4,
    GreaterEqual = 5,
    Greater = 6,
    NotEqual = 7,
}

/// Face culling mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum CullMode {
    #[default]
    None = 0,
    CullClockwise = 1,
    CullCounterClockwise = 2,
}

/// Primitive topology used when drawing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum PrimitiveType {
    #[default]
    LineList = 0,
    LineStrip = 1,
    TriangleList = 2,
    TriangleStrip = 3,
    Points = 4,
    LineLoop = 5,
    TriangleFan = 6,
}

/// Texture color format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum ColorFormat {
    #[default]
    Rgba8888 = 0,
    Rgba8888Srgb = 9,
    Rgba5551 = 1,
    Rgb565 = 2,
    R8 = 3,
    R32f = 4,
    Rg32f = 5,
    Rgba32f = 6,
    Rgba16f = 10,
    LinearLdr = 7,
    SrgbLdr = 8,
}

impl ColorFormat {
    /// Bytes per pixel, or 0 for compressed formats.
    pub const fn bytes_per_pixel(self) -> i32 {
        match self {
            ColorFormat::Rgba8888 | ColorFormat::Rgba8888Srgb => 4,
            ColorFormat::Rgba5551 | ColorFormat::Rgb565 => 2,
            ColorFormat::R8 => 1,
            ColorFormat::R32f => 4,
            ColorFormat::Rg32f => 8,
            ColorFormat::Rgba32f => 16,
            ColorFormat::Rgba16f => 8,
            ColorFormat::LinearLdr | ColorFormat::SrgbLdr => 0,
        }
    }
}

/// Depth buffer format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum DepthFormat {
    #[default]
    None = 0,
    Depth16 = 1,
    Depth24Stencil8 = 2,
}

/// Index buffer element size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum IndexFormat {
    #[default]
    SixteenBits = 0,
    ThirtyTwoBits = 1,
}

impl IndexFormat {
    pub const fn size(self) -> usize {
        match self {
            IndexFormat::SixteenBits => 2,
            IndexFormat::ThirtyTwoBits => 4,
        }
    }
}

/// Texture coordinate wrapping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum TextureAddressMode {
    #[default]
    Clamp = 0,
    Wrap = 1,
    MirrorWrap = 2,
}

/// Texture filtering mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum TextureFilterMode {
    #[default]
    Point = 0,
    Linear = 1,
    Anisotropic = 2,
    PointMipLinear = 3,
    LinearMipPoint = 4,
    MinPointMagLinearMipPoint = 5,
    MinPointMagLinearMipLinear = 6,
    MinLinearMagPointMipPoint = 7,
    MinLinearMagPointMipLinear = 8,
}

/// Vertex element data type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum VertexElementFormat {
    #[default]
    Single = 0,
    Vector2 = 1,
    Vector3 = 2,
    Vector4 = 3,
    Byte4 = 4,
    NormalizedByte4 = 5,
    Short2 = 6,
    NormalizedShort2 = 7,
    Short4 = 8,
    NormalizedShort4 = 9,
}

impl VertexElementFormat {
    /// Size in bytes of one element of this format.
    pub const fn size(self) -> i32 {
        match self {
            VertexElementFormat::Single => 4,
            VertexElementFormat::Vector2 => 8,
            VertexElementFormat::Vector3 => 12,
            VertexElementFormat::Vector4 => 16,
            VertexElementFormat::Byte4 | VertexElementFormat::NormalizedByte4 => 4,
            VertexElementFormat::Short2 | VertexElementFormat::NormalizedShort2 => 4,
            VertexElementFormat::Short4 | VertexElementFormat::NormalizedShort4 => 8,
        }
    }
}

/// Vertex element semantic meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum VertexElementSemantic {
    #[default]
    Position = 0,
    Color = 1,
    Normal = 2,
    TextureCoordinate = 3,
    TextureCoordinate0 = 4,
    TextureCoordinate1 = 5,
    TextureCoordinate2 = 6,
    TextureCoordinate3 = 7,
    TextureCoordinate4 = 8,
    TextureCoordinate5 = 9,
    TextureCoordinate6 = 10,
    TextureCoordinate7 = 11,
    Instance = 12,
    BlendIndices = 13,
    BlendWeights = 14,
    Tangent = 15,
}

impl VertexElementSemantic {
    pub const fn semantic_string(self) -> &'static str {
        match self {
            VertexElementSemantic::Position => "POSITION",
            VertexElementSemantic::Color => "COLOR",
            VertexElementSemantic::Normal => "NORMAL",
            VertexElementSemantic::TextureCoordinate => "TEXCOORD",
            VertexElementSemantic::TextureCoordinate0 => "TEXCOORD0",
            VertexElementSemantic::TextureCoordinate1 => "TEXCOORD1",
            VertexElementSemantic::TextureCoordinate2 => "TEXCOORD2",
            VertexElementSemantic::TextureCoordinate3 => "TEXCOORD3",
            VertexElementSemantic::TextureCoordinate4 => "TEXCOORD4",
            VertexElementSemantic::TextureCoordinate5 => "TEXCOORD5",
            VertexElementSemantic::TextureCoordinate6 => "TEXCOORD6",
            VertexElementSemantic::TextureCoordinate7 => "TEXCOORD7",
            VertexElementSemantic::Instance => "INSTANCE",
            VertexElementSemantic::BlendIndices => "BLENDINDICES",
            VertexElementSemantic::BlendWeights => "BLENDWEIGHTS",
            VertexElementSemantic::Tangent => "TANGENT",
        }
    }
}