    .text
    .align 2

    .global sc_simd_add_f32x4
    .type sc_simd_add_f32x4, %function
sc_simd_add_f32x4:
    ld1 {v0.4s}, [x0]
    ld1 {v1.4s}, [x1]
    fadd v0.4s, v0.4s, v1.4s
    st1 {v0.4s}, [x2]
    ret
    .size sc_simd_add_f32x4, .-sc_simd_add_f32x4

    .global sc_simd_mul_f32x4
    .type sc_simd_mul_f32x4, %function
sc_simd_mul_f32x4:
    ld1 {v0.4s}, [x0]
    ld1 {v1.4s}, [x1]
    fmul v0.4s, v0.4s, v1.4s
    st1 {v0.4s}, [x2]
    ret
    .size sc_simd_mul_f32x4, .-sc_simd_mul_f32x4

    .global sc_simd_dot_f32x4
    .type sc_simd_dot_f32x4, %function
sc_simd_dot_f32x4:
    ld1 {v0.4s}, [x0]
    ld1 {v1.4s}, [x1]
    fmul v0.4s, v0.4s, v1.4s
    faddp v0.2s, v0.2s, v0.2s
    faddp s0, v0.2s
    ret
    .size sc_simd_dot_f32x4, .-sc_simd_dot_f32x4
