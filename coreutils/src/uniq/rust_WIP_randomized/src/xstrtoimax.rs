



use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> intmax_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __intmax_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
unsafe extern "C" fn bkm_scale(
    mut x: *mut intmax_t,
    mut scale_factor: libc::c_int,
) -> strtol_error {
    let mut scaled: intmax_t = 0;
    if if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { *x })
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
            - 1 as libc::c_int) < 0 as libc::c_int
        && (if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        -(1 as libc::c_int) as intmax_t
                    }) + scale_factor as libc::c_long
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (*x < -(1 as libc::c_int) as intmax_t / scale_factor as libc::c_long)
                        as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_long + -(1 as libc::c_int) as intmax_t
                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) as intmax_t / -scale_factor as libc::c_long
                    }) <= -(1 as libc::c_int) as libc::c_long - *x) as libc::c_int
                }
            } else {
                if {
    let scale = if 1 != 0 {
        0
    } else {
        scale_factor
    } as i64;

    let condition1 = (scale - 1) < 0;

    if condition1 {
        let adjusted_scale = (scale + 1) << (std::mem::size_of::<i64>() * 8 - 2) - 1;
        let result = adjusted_scale * 2 + 1;
        !result < 0
    } else {
        scale < 0
    }
} {
    let scale = if 1 != 0 {
        0
    } else {
        scale_factor
    } as i64;

    let condition2 = scale < -((scale - 1) as i64);

    if condition2 {
        let adjusted_scale = (scale + 1) << (std::mem::size_of::<i64>() * 8 - 2) - 1;
        (adjusted_scale * 2 + 1) as i32
    } else {
        (scale - 1) as i32
    }
} else {
    if (0 / scale_factor as i64) < *x {
        1
    } else {
        0
    }
}

            }
        } else {
            if scale_factor == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                if *x < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
                        }) + 0 as libc::c_int as intmax_t
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                *x
                            }) + 0 as libc::c_int as intmax_t
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                *x
                            }) + 0 as libc::c_int as intmax_t
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            *x
                        }) + 0 as libc::c_int as intmax_t)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    *x
                                }) + 0 as libc::c_int as intmax_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        *x
                                    }) + 0 as libc::c_int as intmax_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        *x
                                    }) + 0 as libc::c_int as intmax_t
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                *x
                            }) + 0 as libc::c_int as intmax_t) as libc::c_int
                    }) != 0 && *x == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < scale_factor as libc::c_long
                                    + 0 as libc::c_int as intmax_t) as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) as libc::c_long
                                - 0 as libc::c_int as intmax_t)
                                < (scale_factor - 1 as libc::c_int) as libc::c_long)
                                as libc::c_int
                        }
                    } else {
                        (0 as libc::c_int as intmax_t / *x
                            < scale_factor as libc::c_long) as libc::c_int
                    }
                } else {
                    ((-(1 as libc::c_int) as intmax_t / scale_factor as libc::c_long)
                        < *x) as libc::c_int
                }
            }
        }) != 0
    {
        let (fresh4, _fresh5) = (*x).overflowing_mul(scale_factor.into());
        *(&mut scaled as *mut intmax_t) = fresh4;
        1 as libc::c_int
    } else {
        let (fresh6, fresh7) = (*x).overflowing_mul(scale_factor.into());
        *(&mut scaled as *mut intmax_t) = fresh6;
        fresh7 as libc::c_int
    } != 0
    {
        *x = if *x < 0 as libc::c_int as libc::c_long {
            !if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t {
                -(1 as libc::c_int) as intmax_t
            } else {
                (((1 as libc::c_int as intmax_t)
                    << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }
        } else if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t {
            -(1 as libc::c_int) as intmax_t
        } else {
            (((1 as libc::c_int as intmax_t)
                << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        };
        return LONGINT_OVERFLOW;
    }
    *x = scaled;
    return LONGINT_OK;
}
fn bkm_scale_by_power(
    x: &mut i64, // Assuming intmax_t is equivalent to i64 in Rust
    base: i32,
    mut power: i32,
) -> strtol_error {
    let mut err: strtol_error = LONGINT_OK;
    while power > 0 {
        power -= 1;
        // Call to the unsafe function bkm_scale wrapped in an unsafe block
        err |= unsafe { bkm_scale(x, base) };
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn xstrtoimax(
    mut s: *const libc::c_char,
    mut ptr: *mut *mut libc::c_char,
    mut strtol_base: libc::c_int,
    mut val: *mut intmax_t,
    mut valid_suffixes: *const libc::c_char,
) -> strtol_error {
    use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;

let mut t_ptr: *mut c_char = std::ptr::null_mut();
let mut p: *mut *mut c_char = if let Some(ptr) = ptr.as_mut() { ptr } else { &mut t_ptr };
let mut tmp: i64 = 0;
let mut err: strtol_error = LONGINT_OK;

if !(0 <= strtol_base && strtol_base <= 36) {
    panic!("0 <= strtol_base && strtol_base <= 36");
}

let mut q: *const c_char = s;
let mut ch: u8 = unsafe { *q as u8 };

if (0 as i64) < -(1 as i64) {
    while unsafe { *(*__ctype_b_loc()).offset(ch as i32 as isize) as i32 & _ISspace as u16 as i32 != 0 } {
        q = unsafe { q.offset(1) };
        ch = unsafe { *q as u8 };
    }
    if ch as i32 == '-' as i32 {
        return LONGINT_INVALID;
    }
}

tmp = strtoimax(s, p, strtol_base);

if *p == s as *mut c_char {
    if !valid_suffixes.is_null() && **p as i32 != 0 && !strchr(valid_suffixes, **p as i32).is_null() {
        tmp = 1;
    } else {
        return LONGINT_INVALID;
    }
} else if unsafe { *__errno_location() } != 0 {
    if unsafe { *__errno_location() } != 34 {
        return LONGINT_INVALID;
    }
    err = LONGINT_OVERFLOW;
}

if valid_suffixes.is_null() {
    *val = tmp;
    return err;
}

    if !(*p).is_null() && **p != 0 {
    let mut base: i32 = 1024;
    let mut suffixes: i32 = 1;
    let mut overflow: strtol_error = LONGINT_OK;

    let valid_suffixes_str = unsafe { std::ffi::CStr::from_ptr(valid_suffixes).to_string_lossy() };
    if !valid_suffixes_str.contains(**p as u8 as char) {
        *val = tmp;
        return (err | LONGINT_INVALID_SUFFIX_CHAR) as strtol_error;
    }

    match **p as u8 {
        b'E' | b'G' | b'g' | b'k' | b'K' | b'M' | b'm' | b'P' | b'Q' | b'R' | b'T' | b't' | b'Y' | b'Z' => {
            if valid_suffixes_str.contains('0') {
                match unsafe { *(*p).offset(1) } as u8 {
                    b'i' => {
                        if unsafe { *(*p).offset(2) } as u8 == b'B' {
                            suffixes += 2;
                        }
                    }
                    b'B' | b'D' => {
                        base = 1000;
                        suffixes += 1;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    match **p as u8 {
        b'b' => {
            overflow = bkm_scale(&mut tmp, 512);
        }
        b'B' => {
            overflow = bkm_scale(&mut tmp, 1024);
        }
        b'c' => {
            overflow = LONGINT_OK;
        }
        b'E' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 6);
        }
        b'G' | b'g' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 3);
        }
        b'k' | b'K' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 1);
        }
        b'M' | b'm' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 2);
        }
        b'P' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 5);
        }
        b'Q' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 10);
        }
        b'R' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 9);
        }
        b'T' | b't' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 4);
        }
        b'w' => {
            overflow = bkm_scale(&mut tmp, 2);
        }
        b'Y' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 8);
        }
        b'Z' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 7);
        }
        _ => {
            *val = tmp;
            return (err | LONGINT_INVALID_SUFFIX_CHAR) as strtol_error;
        }
    }

    err |= overflow as u32;
    *p = unsafe { (*p).offset(suffixes as isize) };
    if **p != 0 {
        err |= LONGINT_INVALID_SUFFIX_CHAR;
    }
}
*val = tmp;
return err;

}
