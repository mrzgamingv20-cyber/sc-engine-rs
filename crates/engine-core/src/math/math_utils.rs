pub const PI: f32 = std::f32::consts::PI;
pub const E: f32 = std::f32::consts::E;

#[inline]
pub fn min_i32(x1: i32, x2: i32) -> i32 {
    if x1 >= x2 { x2 } else { x1 }
}

#[inline]
pub fn max_i32(x1: i32, x2: i32) -> i32 {
    if x1 <= x2 { x2 } else { x1 }
}

#[inline]
pub fn min_i32_3(x1: i32, x2: i32, x3: i32) -> i32 {
    min_i32(min_i32(x1, x2), x3)
}

#[inline]
pub fn max_i32_3(x1: i32, x2: i32, x3: i32) -> i32 {
    max_i32(max_i32(x1, x2), x3)
}

#[inline]
pub fn clamp_i32(x: i32, min: i32, max: i32) -> i32 {
    if x >= min {
        if x <= max { x } else { max }
    } else {
        min
    }
}

#[inline]
pub fn sign_i32(x: i32) -> i32 {
    x.signum()
}

#[inline]
pub fn abs_i32(x: i32) -> i32 {
    x.abs()
}

#[inline]
pub fn sqr_i32(x: i32) -> i32 {
    x * x
}

#[inline]
pub fn is_power_of2(x: u32) -> bool {
    x != 0 && (x & (x - 1)) == 0
}

#[inline]
pub fn next_power_of2(mut x: u32) -> u32 {
    x -= 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x += 1;
    x
}

#[inline]
pub fn hash_u32(mut key: u32) -> u32 {
    key ^= key >> 16;
    key = key.wrapping_mul(2146121005);
    key ^= key >> 15;
    key = key.wrapping_mul(2221713035);
    key ^= key >> 16;
    key
}

#[inline]
pub fn hash_i32(key: i32) -> i32 {
    hash_u32(key as u32) as i32
}

#[inline]
pub fn min_f32(x1: f32, x2: f32) -> f32 {
    if x1 >= x2 { x2 } else { x1 }
}

#[inline]
pub fn max_f32(x1: f32, x2: f32) -> f32 {
    if x1 <= x2 { x2 } else { x1 }
}

#[inline]
pub fn min_f32_3(x1: f32, x2: f32, x3: f32) -> f32 {
    min_f32(min_f32(x1, x2), x3)
}

#[inline]
pub fn max_f32_3(x1: f32, x2: f32, x3: f32) -> f32 {
    max_f32(max_f32(x1, x2), x3)
}

#[inline]
pub fn min_f32_4(x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
    min_f32(min_f32(min_f32(x1, x2), x3), x4)
}

#[inline]
pub fn max_f32_4(x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
    max_f32(max_f32(max_f32(x1, x2), x3), x4)
}

#[inline]
pub fn clamp_f32(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline]
pub fn saturate(x: f32) -> f32 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

#[inline]
pub fn sign_f32(x: f32) -> f32 {
    x.signum()
}

#[inline]
pub fn floor_f32(x: f32) -> f32 {
    x.floor()
}

#[inline]
pub fn ceiling_f32(x: f32) -> f32 {
    x.ceil()
}

#[inline]
pub fn round_f32(x: f32) -> f32 {
    x.round()
}

#[inline]
pub fn remainder_f32(x: f32, y: f32) -> f32 {
    x - floor_f32(x / y) * y
}

#[inline]
pub fn sqr_f32(x: f32) -> f32 {
    x * x
}

#[inline]
pub fn lerp(x1: f32, x2: f32, f: f32) -> f32 {
    x1 + (x2 - x1) * f
}

#[inline]
pub fn smooth_step(min: f32, max: f32, x: f32) -> f32 {
    let x = clamp_f32((x - min) / (max - min), 0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

#[inline]
pub fn catmull_rom(v1: f32, v2: f32, v3: f32, v4: f32, f: f32) -> f32 {
    let num = f * f;
    let num2 = num * f;
    0.5 * (2.0 * v2 + (v3 - v1) * f + (2.0 * v1 - 5.0 * v2 + 4.0 * v3 - v4) * num + (3.0 * v2 - v1 - 3.0 * v3 + v4) * num2)
}

#[inline]
pub fn normalize_angle(angle: f32) -> f32 {
    let two_pi = PI * 2.0;
    let mut angle = (angle / two_pi).fract() * two_pi;
    if angle > PI {
        angle -= two_pi;
    } else if angle <= -PI {
        angle += two_pi;
    }
    angle
}

#[inline]
pub fn sigmoid(x: f32, steepness: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let num = steepness.exp();
    let num2 = (2.0 * steepness * x).exp();
    num * (num2 - 1.0) / ((num - 1.0) * (num2 + num))
}

#[inline]
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees / 180.0 * PI
}

#[inline]
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / PI
}

#[inline]
pub fn linear_step(zero: f32, one: f32, f: f32) -> f32 {
    saturate((f - zero) / (one - zero))
}

#[inline]
pub fn pow_sign(x: f32, n: f32) -> f32 {
    sign_f32(x) * x.abs().powf(n)
}
