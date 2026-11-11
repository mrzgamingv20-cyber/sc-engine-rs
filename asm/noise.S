    .text
    .align 2

    .global sc_noise_hash_u32x4
    .type sc_noise_hash_u32x4, %function
sc_noise_hash_u32x4:
    ld1 {v0.4s}, [x0]
    mov w2, #0x79B1
    movk w2, #0x9E37, lsl #16
    dup v1.4s, w2
    mul v0.4s, v0.4s, v1.4s
    ushr v2.4s, v0.4s, #15
    eor v0.16b, v0.16b, v2.16b
    mov w2, #0xCA77
    movk w2, #0x85EB, lsl #16
    dup v1.4s, w2
    mul v0.4s, v0.4s, v1.4s
    ushr v2.4s, v0.4s, #13
    eor v0.16b, v0.16b, v2.16b
    st1 {v0.4s}, [x1]
    ret
    .size sc_noise_hash_u32x4, .-sc_noise_hash_u32x4
