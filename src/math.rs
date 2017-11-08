pub fn next_pow2_u32(value: u32) -> u32 {
    2u32.pow(((value as f32).log2().ceil()) as u32)
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
}

/// 2D Affine Transform Matrix
///
///     | a c x |    | x y o |
///     | b d y | or | x y o |
///     | 0 0 1 |    | x y o |
///
/// This matrix is used to multiply by column vector:
///
///     | a c x |   | x |
///     | b d y | * | y |
///     | 0 0 1 |   | 1 |
///
/// This matrix use column-major order to store elements
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Trans2 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub x: f32,
    pub y: f32,
}

impl Trans2 {
    pub fn identity() -> Trans2 {
        Trans2 {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            x: 0.0,
            y: 0.0,
        }
    }
}

#[repr(C)]
pub struct GlMat3 {
    pub e: [f32; 9],
}

impl From<Trans2> for GlMat3 {
    fn from(trans: Trans2) -> Self {
        GlMat3 {
            e: [
                trans.a,
                trans.b,
                0.0,
                trans.c,
                trans.d,
                0.0,
                trans.x,
                trans.y,
                1.0,
            ],
        }
    }
}
