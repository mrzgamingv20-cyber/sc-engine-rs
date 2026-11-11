#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* out[i] = a[i] + b[i] for 4 lanes. */
void sc_simd_add_f32x4(const float* a, const float* b, float* out);

/* out[i] = a[i] * b[i] for 4 lanes. */
void sc_simd_mul_f32x4(const float* a, const float* b, float* out);

/* Returns a[0]*b[0] + a[1]*b[1] + a[2]*b[2] + a[3]*b[3]. */
float sc_simd_dot_f32x4(const float* a, const float* b);

#ifdef __cplusplus
}
#endif
