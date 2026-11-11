#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 4-lane integer hash used by world generation (xxHash-style finalizer). */
void sc_noise_hash_u32x4(const uint32_t* input, uint32_t* output);

#ifdef __cplusplus
}
#endif
