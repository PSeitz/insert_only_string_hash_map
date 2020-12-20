use std::ptr::copy_nonoverlapping;

#[inline]
fn rotl32(x: u32, r: u8) -> u32 {
    (x << r) | (x >> (32 - r))
}

#[inline]
fn read_u32_p(p: *const u8) -> u32 {
    let mut out: u32 = 0;
    unsafe {
        copy_nonoverlapping(p, &mut out as *mut u32 as *mut u8, 4);
    }
    out
}

#[inline]
fn read_u16_p(p: *const u8) -> u16 {
    let mut out: u16 = 0;
    unsafe {
        copy_nonoverlapping(p, &mut out as *mut u16 as *mut u8, 2);
    }
    out
}

#[inline(never)]
pub fn fnv32a_yoshimitsu_hasher(bytes: &[u8]) -> u32 {
    fnv32a_yoshimitsu_triad(0xD8AF_FD71, bytes)
}

#[inline]
pub fn fnv32a_yoshimitsu_triad(seed: u32, bytes: &[u8]) -> u32 {
    let mut len: u32 = bytes.len() as u32;
    let mut p: *const u8 = bytes.as_ptr();

    const PRIME: u32 = 709_607;
    let mut hash32_a: u32 = seed ^ 2_166_136_261;
    let mut hash32_b: u32 = 2_166_136_261 + len;
    let mut hash32_c: u32 = 2_166_136_261;
    while len >= 24 {
        hash32_a = (hash32_a ^ (rotl32(read_u32_p(p), 5) ^ read_u32_p(unsafe { p.offset(4) })))
            .wrapping_mul(PRIME);
        hash32_b = (hash32_b
            ^ (rotl32(read_u32_p(unsafe { p.offset(8) }), 5)
                ^ read_u32_p(unsafe { p.offset(12) })))
        .wrapping_mul(PRIME);
        hash32_c = (hash32_c
            ^ (rotl32(read_u32_p(unsafe { p.offset(16) }), 5)
                ^ read_u32_p(unsafe { p.offset(20) })))
        .wrapping_mul(PRIME);
        len -= 24;
        p = unsafe { p.offset(24) };
    }

    if p == bytes.as_ptr() {
        hash32_a = (hash32_a ^ rotl32(hash32_c, 5)).wrapping_mul(PRIME);
    }
    //Cases 0. .31
    if (len & 16) != 0 {
        hash32_a = (hash32_a ^ (rotl32(read_u32_p(p), 5) ^ read_u32_p(unsafe { p.offset(4) })))
            .wrapping_mul(PRIME);
        hash32_b = (hash32_b
            ^ (rotl32(read_u32_p(unsafe { p.offset(8) }), 5)
                ^ read_u32_p(unsafe { p.offset(12) })))
        .wrapping_mul(PRIME);
        p = unsafe { p.offset(16) };
    }
    //Cases 0. .15
    if (len & 8) != 0 {
        hash32_a = (hash32_a ^ read_u32_p(p)).wrapping_mul(PRIME);
        hash32_b = (hash32_b ^ read_u32_p(unsafe { p.offset(4) })).wrapping_mul(PRIME);
        p = unsafe { p.offset(8) };
    }
    //Cases:0. .7
    if (len & 4) != 0 {
        hash32_a = (hash32_a ^ u32::from(read_u16_p(p))).wrapping_mul(PRIME);
        hash32_b = (hash32_b ^ u32::from(read_u16_p(unsafe { p.offset(2) }))).wrapping_mul(PRIME);
        p = unsafe { p.offset(4) };
    }
    // //Cases:0. .3
    if (len & 2) != 0 {
        hash32_a = (hash32_a ^ u32::from(read_u16_p(p))).wrapping_mul(PRIME);
        p = unsafe { p.offset(2) };
    }
    if (len & 1) != 0 {
        hash32_a = (hash32_a ^ u32::from(unsafe { *p })).wrapping_mul(PRIME);
    }
    hash32_a = (hash32_a ^ rotl32(hash32_b, 5)).wrapping_mul(PRIME);
    hash32_a ^ (hash32_a >> 16)
}
