


















use std::slice;

extern "C" {
    
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::zlib::adler32::adler32;
pub use crate::src::zlib::inffast::inflate_fast;
pub use crate::src::zlib::inftrees::inflate_table;
pub use crate::src::zlib::zutil::zcalloc;
pub use crate::src::zlib::zutil::zcfree;
pub use crate::src::zlib::deflate::internal_state;
pub type Byte = crate::src::libpng::png::Byte;
pub type uInt = crate::src::libpng::png::uInt;
pub type uLong = crate::src::libpng::png::uLong;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type free_func = crate::src::libpng::png::free_func;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::pngerror::z_stream_s;
pub type z_stream = crate::src::libpng::png::z_stream;
pub type z_streamp = crate::src::libpng::png::z_streamp;
// #[derive(Copy, Clone)]

pub type gz_header_s = crate::src::zlib::infback::gz_header_s;
pub type gz_header = crate::src::zlib::deflate::gz_header;
pub type gz_headerp = crate::src::zlib::deflate::gz_headerp;
pub const COPY_: inflate_mode = 16194;
pub type inflate_mode = crate::src::zlib::infback::inflate_mode;
pub const SYNC: inflate_mode = 16211;
pub const MEM: inflate_mode = 16210;
pub const BAD: inflate_mode = 16209;
pub const DONE: inflate_mode = 16208;
pub const LENGTH: inflate_mode = 16207;
pub const CHECK: inflate_mode = 16206;
pub const LIT: inflate_mode = 16205;
pub const MATCH: inflate_mode = 16204;
pub const DISTEXT: inflate_mode = 16203;
pub const DIST: inflate_mode = 16202;
pub const LENEXT: inflate_mode = 16201;
pub const LEN: inflate_mode = 16200;
pub const LEN_: inflate_mode = 16199;
pub const CODELENS: inflate_mode = 16198;
pub const LENLENS: inflate_mode = 16197;
pub const TABLE: inflate_mode = 16196;
pub const COPY: inflate_mode = 16195;
pub const STORED: inflate_mode = 16193;
pub const TYPEDO: inflate_mode = 16192;
pub const TYPE: inflate_mode = 16191;
pub const DICT: inflate_mode = 16190;
pub const DICTID: inflate_mode = 16189;
pub const HCRC: inflate_mode = 16188;
pub const COMMENT: inflate_mode = 16187;
pub const NAME: inflate_mode = 16186;
pub const EXTRA: inflate_mode = 16185;
pub const EXLEN: inflate_mode = 16184;
pub const OS: inflate_mode = 16183;
pub const TIME: inflate_mode = 16182;
pub const FLAGS: inflate_mode = 16181;
pub const HEAD: inflate_mode = 16180;
// #[derive(Copy, Clone)]

pub type inflate_state = crate::src::zlib::inffast::inflate_state;
// #[derive(Copy, Clone)]

pub type code = crate::src::zlib::inffast::code;
pub type codetype = crate::src::zlib::infback::codetype;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
/* inflate.c -- zlib decompression
 * Copyright (C) 1995-2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
 * Change history:
 *
 * 1.2.beta0    24 Nov 2002
 * - First version -- complete rewrite of inflate to simplify code, avoid
 *   creation of window when not needed, minimize use of window when it is
 *   needed, make inffast.c even faster, implement gzip decoding, and to
 *   improve code readability and style over the previous zlib inflate code
 *
 * 1.2.beta1    25 Nov 2002
 * - Use pointers for available input and output checking in inffast.c
 * - Remove input and output counters in inffast.c
 * - Change inffast.c entry and loop from avail_in >= 7 to >= 6
 * - Remove unnecessary second byte pull from length extra in inffast.c
 * - Unroll direct copy to three copies per loop in inffast.c
 *
 * 1.2.beta2    4 Dec 2002
 * - Change external routine names to reduce potential conflicts
 * - Correct filename to inffixed.h for fixed tables in inflate.c
 * - Make hbuf[] unsigned char to match parameter type in inflate.c
 * - Change strm->next_out[-state->offset] to *(strm->next_out - state->offset)
 *   to avoid negation problem on Alphas (64 bit) in inflate.c
 *
 * 1.2.beta3    22 Dec 2002
 * - Add comments on state->bits assertion in inffast.c
 * - Add comments on op field in inftrees.h
 * - Fix bug in reuse of allocated window after inflateReset()
 * - Remove bit fields--back to byte structure for speed
 * - Remove distance extra == 0 check in inflate_fast()--only helps for lengths
 * - Change post-increments to pre-increments in inflate_fast(), PPC biased?
 * - Add compile time option, POSTINC, to use post-increments instead (Intel?)
 * - Make MATCH copy in inflate() much faster for when inflate_fast() not used
 * - Use local copies of stream next and avail values, as well as local bit
 *   buffer and bit count in inflate()--for speed when inflate_fast() not used
 *
 * 1.2.beta4    1 Jan 2003
 * - Split ptr - 257 statements in inflate_table() to avoid compiler warnings
 * - Move a comment on output buffer sizes from inffast.c to inflate.c
 * - Add comments in inffast.c to introduce the inflate_fast() routine
 * - Rearrange window copies in inflate_fast() for speed and simplification
 * - Unroll last copy for window match in inflate_fast()
 * - Use local copies of window variables in inflate_fast() for speed
 * - Pull out common wnext == 0 case for speed in inflate_fast()
 * - Make op and len in inflate_fast() unsigned for consistency
 * - Add FAR to lcode and dcode declarations in inflate_fast()
 * - Simplified bad distance check in inflate_fast()
 * - Added inflateBackInit(), inflateBack(), and inflateBackEnd() in new
 *   source file infback.c to provide a call-back interface to inflate for
 *   programs like gzip and unzip -- uses window as output buffer to avoid
 *   window copying
 *
 * 1.2.beta5    1 Jan 2003
 * - Improved inflateBack() interface to allow the caller to provide initial
 *   input in strm.
 * - Fixed stored blocks bug in inflateBack()
 *
 * 1.2.beta6    4 Jan 2003
 * - Added comments in inffast.c on effectiveness of POSTINC
 * - Typecasting all around to reduce compiler warnings
 * - Changed loops from while (1) or do {} while (1) to for (;;), again to
 *   make compilers happy
 * - Changed type of window in inflateBackInit() to unsigned char *
 *
 * 1.2.beta7    27 Jan 2003
 * - Changed many types to unsigned or unsigned short to avoid warnings
 * - Added inflateCopy() function
 *
 * 1.2.0        9 Mar 2003
 * - Changed inflateBack() interface to provide separate opaque descriptors
 *   for the in() and out() functions
 * - Changed inflateBack() argument and in_func typedef to swap the length
 *   and buffer address return values for the input function
 * - Check next_in and next_out for Z_NULL on entry to inflate()
 *
 * The history for versions after 1.2.0 are in ChangeLog in zlib distribution.
 */
/* function prototypes */
unsafe extern "C" fn inflateStateCheck(mut strm: z_streamp) -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as std::os::raw::c_int
    }
    state = (*strm).state as *mut inflate_state;
    if state.is_null() || (*state).strm != strm ||
           ((*state).mode as std::os::raw::c_uint) <
               HEAD as std::os::raw::c_int as std::os::raw::c_uint ||
           (*state).mode as std::os::raw::c_uint > SYNC as std::os::raw::c_int as std::os::raw::c_uint
       {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateResetKeep(mut strm: z_streamp)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    (*state).total = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*strm).total_out = (*state).total;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = 0 as *mut std::os::raw::c_char;
    if (*state).wrap != 0 {
        /* to support ill-conceived Java test suite */
        (*strm).adler = ((*state).wrap & 1 as std::os::raw::c_int) as uLong
    }
    (*state).mode = HEAD;
    (*state).last = 0 as std::os::raw::c_int;
    (*state).havedict = 0 as std::os::raw::c_int;
    (*state).dmax = 32768 as std::os::raw::c_uint;
    (*state).head = 0 as gz_headerp;
    (*state).hold = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*state).bits = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*state).next = (*state).codes.as_mut_ptr();
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    (*state).sane = 1 as std::os::raw::c_int;
    (*state).back = -(1 as std::os::raw::c_int);
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateReset(mut strm: z_streamp) -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    (*state).wsize = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*state).whave = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*state).wnext = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    return inflateResetKeep(strm);
}
#[no_mangle]
pub unsafe extern "C" fn inflateReset2(mut strm: z_streamp,
                                       mut windowBits: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut wrap: std::os::raw::c_int = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    /* get the state */
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    /* extract wrap request from windowBits parameter */
    if windowBits < 0 as std::os::raw::c_int {
        wrap = 0 as std::os::raw::c_int;
        windowBits = -windowBits
    } else { wrap = (windowBits >> 4 as std::os::raw::c_int) + 5 as std::os::raw::c_int }
    /* set number of window bits, free window if different */
    if windowBits != 0 &&
           (windowBits < 8 as std::os::raw::c_int || windowBits > 15 as std::os::raw::c_int) {
        return -(2 as std::os::raw::c_int)
    }
    if !(*state).window.is_null() &&
           (*state).wbits != windowBits as std::os::raw::c_uint {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                    (*state).window
                                                                                                        as
                                                                                                        voidpf);
        (*state).window = 0 as *mut std::os::raw::c_uchar
    }
    /* update state and reset the rest of it */
    (*state).wrap = wrap; /* in case we return an error */
    (*state).wbits =
        windowBits as
            std::os::raw::c_uint; /* to pass state test in inflateReset2() */
    return inflateReset(strm);
}
#[no_mangle]
pub unsafe extern "C" fn inflateInit2_(mut strm: z_streamp,
                                       mut windowBits: std::os::raw::c_int,
                                       mut version: *const std::os::raw::c_char,
                                       mut stream_size: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if version.is_null() ||
           *version.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               (*::std::mem::transmute::<&[u8; 15],
                                         &[std::os::raw::c_char; 15]>(b"1.2.11-optipng\x00"))[0
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          usize]
                   as std::os::raw::c_int ||
           stream_size !=
               ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                   std::os::raw::c_int {
        return -(6 as std::os::raw::c_int)
    }
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    (*strm).msg = 0 as *mut std::os::raw::c_char;
    if (*strm).zalloc.is_none() {
        (*strm).zalloc =
            Some(zcalloc as
                     unsafe extern "C" fn(_: voidpf, _: std::os::raw::c_uint,
                                          _: std::os::raw::c_uint) -> voidpf);
        (*strm).opaque = 0 as voidpf
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree =
            Some(zcfree as unsafe extern "C" fn(_: voidpf, _: voidpf) -> ())
    }
    state =
        Some((*strm).zalloc.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                     1
                                                                                                         as
                                                                                                         std::os::raw::c_int
                                                                                                         as
                                                                                                         uInt,
                                                                                                     ::std::mem::size_of::<inflate_state>()
                                                                                                         as
                                                                                                         std::os::raw::c_ulong
                                                                                                         as
                                                                                                         uInt)
            as *mut inflate_state;
    if state.is_null() { return -(4 as std::os::raw::c_int) }
    (*strm).state = state as *mut internal_state;
    (*state).strm = strm;
    (*state).window = 0 as *mut std::os::raw::c_uchar;
    (*state).mode = HEAD;
    ret = inflateReset2(strm, windowBits);
    if ret != 0 as std::os::raw::c_int {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                    state
                                                                                                        as
                                                                                                        voidpf);
        (*strm).state = 0 as *mut internal_state
    }
    return ret;
}
#[no_mangle]
pub fn inflateInit_(strm: &mut z_stream, version: &str, stream_size: i32) -> i32 {
    let version_cstr = std::ffi::CString::new(version).expect("CString::new failed");
    let version_ptr = version_cstr.as_ptr();
    let stream_size_c: i32 = stream_size;

    unsafe {
        inflateInit2_(strm, 15, version_ptr, stream_size_c)
    }
}

#[no_mangle]
pub unsafe extern "C" fn inflatePrime(mut strm: z_streamp,
                                      mut bits: std::os::raw::c_int,
                                      mut value: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    if bits < 0 as std::os::raw::c_int {
        (*state).hold = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
        (*state).bits = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        return 0 as std::os::raw::c_int
    }
    if bits > 16 as std::os::raw::c_int ||
           (*state).bits.wrapping_add(bits as uInt) >
               32 as std::os::raw::c_int as std::os::raw::c_uint {
        return -(2 as std::os::raw::c_int)
    }
    value =
        (value as std::os::raw::c_long &
             ((1 as std::os::raw::c_long) << bits) - 1 as std::os::raw::c_int as std::os::raw::c_long)
            as std::os::raw::c_int;
    (*state).hold =
        (*state).hold.wrapping_add(((value as std::os::raw::c_uint) << (*state).bits)
                                       as std::os::raw::c_ulong);
    (*state).bits = (*state).bits.wrapping_add(bits as uInt);
    return 0 as std::os::raw::c_int;
}
/*
   Return state with length and distance decoding tables and index sizes set to
   fixed code decoding.  Normally this returns fixed tables from inffixed.h.
   If BUILDFIXED is defined, then instead this routine builds the tables the
   first time it's called, and returns those tables the first time and
   thereafter.  This reduces the size of the code by about 2K bytes, in
   exchange for a little execution time.  However, BUILDFIXED should not be
   used for threaded applications, since the rewriting of the tables and virgin
   may not be thread-safe.
 */
fn fixedtables(state: *mut inflate_state) {
    let state = unsafe { &mut *state };
     static mut lenfix: [code; 512] =
        [{
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 80 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 16 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 112 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 48 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 192 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 96 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 32 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 160 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 128 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 64 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 224 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 88 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 24 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 144 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 120 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 56 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 208 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 104 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 40 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 176 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 136 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 72 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 240 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 84 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 20 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 227 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 116 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 52 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 200 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 100 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 36 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 168 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 132 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 68 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 232 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 92 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 28 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 152 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 124 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 60 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 216 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 108 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 44 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 184 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 12 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 140 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 76 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 248 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 82 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 18 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 163 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 114 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 50 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 196 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 98 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 34 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 164 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 130 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 66 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 228 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 90 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 26 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 148 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 122 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 58 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 212 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 106 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 42 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 180 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 138 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 74 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 244 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 86 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 22 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 118 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 54 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 204 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 102 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 38 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 172 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 134 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 70 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 236 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 94 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 30 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 156 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 126 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 62 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 220 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 110 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 46 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 188 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 14 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 142 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 78 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 252 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 81 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 113 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 49 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 194 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 97 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 33 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 162 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 129 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 65 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 226 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 89 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 25 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 146 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 121 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 57 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 210 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 105 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 41 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 178 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 137 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 73 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 242 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 85 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 21 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 258 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 117 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 53 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 202 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 101 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 37 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 170 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 133 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 69 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 234 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 93 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 29 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 154 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 125 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 61 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 218 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 109 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 45 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 186 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 141 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 77 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 250 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 195 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 198 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 166 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 230 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 91 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 150 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 123 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 214 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 107 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 182 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 139 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 75 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 246 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 87 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 119 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 55 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 206 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 103 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 39 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 174 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 135 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 71 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 238 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 95 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 158 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 127 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 63 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 222 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 111 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 47 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 190 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 143 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 79 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 254 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 80 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 16 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 112 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 48 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 193 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 96 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 32 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 161 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 128 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 64 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 225 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 88 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 24 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 145 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 120 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 56 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 209 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 104 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 40 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 177 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 136 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 72 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 241 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 84 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 20 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 227 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 116 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 52 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 201 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 100 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 36 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 169 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 132 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 68 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 233 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 92 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 28 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 153 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 124 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 60 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 217 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 108 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 44 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 185 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 12 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 140 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 76 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 249 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 82 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 18 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 163 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 114 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 50 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 197 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 98 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 34 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 165 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 130 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 66 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 229 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 90 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 26 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 149 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 122 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 58 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 213 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 106 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 42 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 181 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 138 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 74 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 245 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 86 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 22 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 118 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 54 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 205 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 102 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 38 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 173 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 134 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 70 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 237 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 94 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 30 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 157 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 126 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 62 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 221 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 110 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 46 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 189 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 14 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 142 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 78 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 253 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 81 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 113 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 49 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 195 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 97 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 33 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 163 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 129 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 65 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 227 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 89 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 25 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 147 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 121 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 57 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 211 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 105 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 41 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 179 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 137 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 73 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 243 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 85 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 21 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 258 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 117 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 53 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 203 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 101 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 37 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 171 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 133 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 69 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 235 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 93 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 29 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 155 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 125 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 61 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 219 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 109 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 45 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 187 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 141 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 77 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 251 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 195 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 199 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 167 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 231 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 91 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 151 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 123 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 215 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 107 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 183 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 139 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 75 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 247 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 87 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 119 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 55 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 207 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 103 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 39 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 175 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 135 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 71 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 239 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 95 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 159 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 127 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 63 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 223 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 111 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 47 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 191 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 143 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 79 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 255 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         }];
    static mut distfix: [code; 32] =
        [{
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 23 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 257 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 27 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4097 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 25 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1025 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 65 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 29 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 16385 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 24 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 513 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 33 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 28 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8193 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 26 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2049 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 22 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 129 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 23 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 385 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 25 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 27 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6145 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 25 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1537 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 97 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 29 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 24577 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 24 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 769 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 49 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 28 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 12289 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 26 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3073 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 22 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 193 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         }];
    /* !BUILDFIXED */
    /* BUILDFIXED */
    state.lencode = unsafe { lenfix.as_ptr() };
    state.lenbits = 9u32;
    state.distcode = unsafe { distfix.as_ptr() };
    state.distbits = 5u32;
}

/* MAKEFIXED */
/*
   Update the window with the last wsize (normally 32K) bytes written before
   returning.  If window does not exist yet, create it.  This is only called
   when a window is already in use, or when output has been written during this
   inflate call, but the end of the deflate stream has not been reached yet.
   It is also called to create a window for dictionary data when a dictionary
   is loaded.

   Providing output buffers larger than 32K to inflate() should provide a speed
   advantage, since only the last 32K of output is copied to the sliding window
   upon return from inflate(), and since all distances after the first 32K of
   output will fall in the output data, making match copies simpler and faster.
   The advantage may be dependent on the size of the processor's data caches.
 */
unsafe extern "C" fn updatewindow(mut strm: z_streamp, mut end: *const Bytef,
                                  mut copy: std::os::raw::c_uint) -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut dist: std::os::raw::c_uint = 0;
    state = (*strm).state as *mut inflate_state;
    /* if it hasn't been done already, allocate space for the window */
    if (*state).window.is_null() {
        (*state).window =
            Some((*strm).zalloc.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                         (1
                                                                                                              as
                                                                                                              std::os::raw::c_uint)
                                                                                                             <<
                                                                                                             (*state).wbits,
                                                                                                         ::std::mem::size_of::<std::os::raw::c_uchar>()
                                                                                                             as
                                                                                                             std::os::raw::c_ulong
                                                                                                             as
                                                                                                             uInt)
                as *mut std::os::raw::c_uchar;
        if (*state).window.is_null() { return 1 as std::os::raw::c_int }
    }
    /* if window not in use yet, initialize */
    if (*state).wsize == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).wsize = (1 as std::os::raw::c_uint) << (*state).wbits;
        (*state).wnext = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        (*state).whave = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* copy state->wsize or less output bytes into the circular window */
    if copy >= (*state).wsize {
        memcpy((*state).window as *mut std::os::raw::c_void,
               end.offset(-((*state).wsize as isize)) as *const std::os::raw::c_void,
               (*state).wsize as std::os::raw::c_ulong);
        (*state).wnext = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        (*state).whave = (*state).wsize
    } else {
        dist = (*state).wsize.wrapping_sub((*state).wnext);
        if dist > copy { dist = copy }
        memcpy((*state).window.offset((*state).wnext as isize) as
                   *mut std::os::raw::c_void,
               end.offset(-(copy as isize)) as *const std::os::raw::c_void,
               dist as std::os::raw::c_ulong);
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            memcpy((*state).window as *mut std::os::raw::c_void,
                   end.offset(-(copy as isize)) as *const std::os::raw::c_void,
                   copy as std::os::raw::c_ulong);
            (*state).wnext = copy;
            (*state).whave = (*state).wsize
        } else {
            (*state).wnext = (*state).wnext.wrapping_add(dist);
            if (*state).wnext == (*state).wsize {
                (*state).wnext = 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
            if (*state).whave < (*state).wsize {
                (*state).whave = (*state).whave.wrapping_add(dist)
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* Remove n bits from the bit accumulator */
/* Remove zero to seven bits as needed to go to a byte boundary */
/*
   inflate() uses a state machine to process as much input data and generate as
   much output data as possible before returning.  The state machine is
   structured roughly as follows:

    for (;;) switch (state) {
    ...
    case STATEn:
        if (not enough input data or output space to make progress)
            return;
        ... make progress ...
        state = STATEm;
        break;
    ...
    }

   so when inflate() is called again, the same case is attempted again, and
   if the appropriate resources are provided, the machine proceeds to the
   next state.  The NEEDBITS() macro is usually the way the state evaluates
   whether it can proceed or should return.  NEEDBITS() does the return if
   the requested bits are not available.  The typical use of the BITS macros
   is:

        NEEDBITS(n);
        ... do something with BITS(n) ...
        DROPBITS(n);

   where NEEDBITS(n) either returns from inflate() if there isn't enough
   input left to load n bits into the accumulator, or it continues.  BITS(n)
   gives the low n bits in the accumulator.  When done, DROPBITS(n) drops
   the low n bits off the accumulator.  INITBITS() clears the accumulator
   and sets the number of available bits to zero.  BYTEBITS() discards just
   enough bits to put the accumulator on a byte boundary.  After BYTEBITS()
   and a NEEDBITS(8), then BITS(8) would return the next byte in the stream.

   NEEDBITS(n) uses PULLBYTE() to get an available byte of input, or to return
   if there is no input available.  The decoding of variable length codes uses
   PULLBYTE() directly in order to pull just enough bytes to decode the next
   code, and no more.

   Some states loop until they get enough input, making sure that enough
   state information is maintained to continue the loop where it left off
   if NEEDBITS() returns in the loop.  For example, want, need, and keep
   would all have to actually be part of the saved state in case NEEDBITS()
   returns:

    case STATEw:
        while (want < need) {
            NEEDBITS(n);
            keep[want++] = BITS(n);
            DROPBITS(n);
        }
        state = STATEx;
    case STATEx:

   As shown above, if the next state is also the next case, then the break
   is omitted.

   A state may also return if there is not enough output space available to
   complete that state.  Those states are copying stored data, writing a
   literal byte, and copying a matching string.

   When returning, a "goto inf_leave" is used to update the total counters,
   update the check value, and determine whether any progress has been made
   during that inflate() call in order to return the proper return code.
   Progress is defined as a change in either strm->avail_in or strm->avail_out.
   When there is a window, goto inf_leave will update the window with the last
   output written.  If a goto inf_leave occurs in the middle of decompression
   and there is no window currently, goto inf_leave will create one and copy
   output to the window for the next call of inflate().

   In this implementation, the flush parameter of inflate() only affects the
   return code (per zlib.h).  inflate() always writes as much as possible to
   strm->next_out, given the space available and the provided input--the effect
   documented in zlib.h of Z_SYNC_FLUSH.  Furthermore, inflate() always defers
   the allocation of and copying into a sliding window until necessary, which
   provides the effect documented in zlib.h for Z_FINISH when the entire input
   stream available.  So the only thing the flush parameter actually does is:
   when flush is set to Z_FINISH, inflate() cannot return Z_OK.  Instead it
   will return Z_BUF_ERROR if it has not reached the end of the stream.
 */
#[no_mangle]
pub unsafe extern "C" fn inflate(mut strm: z_streamp, mut flush: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64; /* next input */
    let mut state: *mut inflate_state =
        0 as *mut inflate_state; /* next output */
    let mut next: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* available input and output */
    let mut put: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* bit buffer */
    let mut have: std::os::raw::c_uint = 0; /* bits in bit buffer */
    let mut left: std::os::raw::c_uint =
        0; /* save starting available input and output */
    let mut hold: std::os::raw::c_ulong =
        0; /* number of stored or match bytes to copy */
    let mut bits: std::os::raw::c_uint = 0; /* where to copy match bytes from */
    let mut in_0: std::os::raw::c_uint = 0; /* current decoding table entry */
    let mut out: std::os::raw::c_uint = 0; /* parent table entry */
    let mut copy: std::os::raw::c_uint =
        0; /* length to copy for repeats, bits to drop */
    let mut from: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* return code */
    let mut here: code = code{op: 0, bits: 0, val: 0,}; /* skip check */
    let mut last: code =
        code{op: 0, bits: 0, val: 0,}; /* go to byte boundary */
    let mut len: std::os::raw::c_uint = 0;
    let mut ret: std::os::raw::c_int = 0;
    static mut order: [std::os::raw::c_ushort; 19] =
        [16 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         0 as std::os::raw::c_int as std::os::raw::c_ushort,
         8 as std::os::raw::c_int as std::os::raw::c_ushort,
         7 as std::os::raw::c_int as std::os::raw::c_ushort,
         9 as std::os::raw::c_int as std::os::raw::c_ushort,
         6 as std::os::raw::c_int as std::os::raw::c_ushort,
         10 as std::os::raw::c_int as std::os::raw::c_ushort,
         5 as std::os::raw::c_int as std::os::raw::c_ushort,
         11 as std::os::raw::c_int as std::os::raw::c_ushort,
         4 as std::os::raw::c_int as std::os::raw::c_ushort,
         12 as std::os::raw::c_int as std::os::raw::c_ushort,
         3 as std::os::raw::c_int as std::os::raw::c_ushort,
         13 as std::os::raw::c_int as std::os::raw::c_ushort,
         2 as std::os::raw::c_int as std::os::raw::c_ushort,
         14 as std::os::raw::c_int as std::os::raw::c_ushort,
         1 as std::os::raw::c_int as std::os::raw::c_ushort,
         15 as std::os::raw::c_int as std::os::raw::c_ushort];
    if inflateStateCheck(strm) != 0 || (*strm).next_out.is_null() ||
           (*strm).next_in.is_null() &&
               (*strm).avail_in != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return -(2 as std::os::raw::c_int)
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).mode as std::os::raw::c_uint == TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).mode = TYPEDO
    }
    put = (*strm).next_out;
    left = (*strm).avail_out;
    next = (*strm).next_in as *mut u8;
    have = (*strm).avail_in;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = 0 as std::os::raw::c_int;
    's_114:
        loop  {
            match (*state).mode as std::os::raw::c_uint {
                16180 => {
                    if unsafe { (*state).wrap } == 0 {
    unsafe { (*state).mode = TYPEDO };
    continue;
} else {
    while bits < 16 {
        if have == 0 {
            break 's_114;
        }
        have -= 1;
        let fresh0 = next;
        next = next.add(1);
        hold += (unsafe { *fresh0 } as u64) << bits;
        bits += 8;
    }
    if ((hold & ((1 << 8) - 1) << 8) + (hold >> 8) % 31) != 0 {
        unsafe { (*strm).msg = b"incorrect header check\x00".as_ptr() as *const i8 as *mut i8 };
        unsafe { (*state).mode = BAD };
        continue;
    } else if hold & ((1 << 4) - 1) != 8 {
        unsafe { (*strm).msg = b"unknown compression method\x00".as_ptr() as *const i8 as *mut i8 };
        unsafe { (*state).mode = BAD };
        continue;
    } else {
        hold >>= 4;
        bits -= 4;
        let len: u32 = (hold as u32 & ((1 << 4) - 1)) + 8;
        if unsafe { (*state).wbits } == 0 {
            unsafe { (*state).wbits = len };
        }
        if len > 15 || len > unsafe { (*state).wbits } {
            unsafe { (*strm).msg = b"invalid window size\x00".as_ptr() as *const i8 as *mut i8 };
            unsafe { (*state).mode = BAD };
            continue;
        } else {
            unsafe { (*state).dmax = 1 << len };
            unsafe { (*state).check = adler32(0, std::ptr::null(), 0) };
            unsafe { (*strm).adler = (*state).check };
            unsafe { (*state).mode = if hold & 0x200 != 0 { DICTID } else { TYPE } };
            hold = 0;
            bits = 0;
            continue;
        }
    }
}

                }
                16189 => {
                    while bits < 32 {
    if have == 0 {
        break 's_114;
    }
    have -= 1;
    let fresh1 = next;
    next = next.add(1);
    hold += (*fresh1 as u64) << bits;
    bits += 8;
}
(*state).check = (hold >> 24 & 0xff)
    .wrapping_add((hold >> 8 & 0xff00))
    .wrapping_add((hold & 0xff00) << 8)
    .wrapping_add((hold & 0xff) << 24);
(*strm).adler = (*state).check;
hold = 0;
bits = 0;
(*state).mode = DICT;
current_block = 14331593601693766846;

                }
                16190 => { current_block = 14331593601693766846; }
                16191 => { current_block = 210528378685203046; }
                16192 => { current_block = 6853415491165023619; }
                16193 => {
                    let mut hold = hold >> (bits & 7);
bits = bits.wrapping_sub(bits & 7);

while bits < 32 {
    if have == 0 {
        break 's_114;
    }
    have = have.wrapping_sub(1);
    let fresh3 = next;
    next = next.add(1);
    hold = hold.wrapping_add((fresh3 as u64) << bits);
    bits = bits.wrapping_add(8);
}

if hold & 0xffff != (hold >> 16) ^ 0xffff {
    (*strm).msg = "invalid stored block lengths\0".as_ptr() as *mut i8;
    (*state).mode = BAD;
    continue;
} else {
    (*state).length = (hold & 0xffff) as u32;
    hold = 0;
    bits = 0;
    (*state).mode = COPY_;
    if flush == 6 {
        break;
    }
}
current_block = 16345656043779050082;

                }
                16194 => { current_block = 16345656043779050082; }
                16195 => { current_block = 7368450687049571364; }
                16196 => {
                    while bits < 14 {
    if have == 0 {
        break 's_114;
    }
    have -= 1;
    let fresh4 = next;
    next = next.add(1);
    hold += (*fresh4 as u64) << bits;
    bits += 8;
}

let nlen = (hold as u32 & ((1 << 5) - 1)).wrapping_add(257);
hold >>= 5;
bits -= 5;

let ndist = (hold as u32 & ((1 << 5) - 1)).wrapping_add(1);
hold >>= 5;
bits -= 5;

let ncode = (hold as u32 & ((1 << 4) - 1)).wrapping_add(4);
hold >>= 4;
bits -= 4;

if nlen > 286 || ndist > 30 {
    (*strm).msg = "too many length or distance symbols\0".as_ptr() as *mut i8;
    (*state).mode = BAD;
    continue;
} else {
    (*state).have = 0;
    (*state).mode = LENLENS;
}

(*state).nlen = nlen;
(*state).ndist = ndist;
(*state).ncode = ncode;

current_block = 9974864727789713748;

                }
                16197 => { current_block = 9974864727789713748; }
                16198 => { current_block = 1995330570110937187; }
                16199 => { current_block = 1692384543052803397; }
                16200 => { current_block = 7212245502802746503; }
                16201 => { current_block = 7259693266422973518; }
                16202 => { current_block = 14213006019499569531; }
                16203 => { current_block = 13874176978779739039; }
                16204 => { current_block = 3519113796920557345; }
                16205 => {
                    if left == 0 as std::os::raw::c_int as std::os::raw::c_uint { break ; }
                    let fresh23 = put;
                    put = put.offset(1);
                    *fresh23 = (*state).length as std::os::raw::c_uchar;
                    left = left.wrapping_sub(1);
                    (*state).mode = LEN;
                    continue ;
                }
                16206 => {
                    if unsafe { (*state).wrap } != 0 {
    while bits < 32 {
        if have == 0 {
            break 's_114;
        }
        have -= 1;
        let fresh24 = unsafe { *next };
        next = unsafe { next.add(1) }; // Move to the next byte
        hold += (fresh24 as u64) << bits;
        bits += 8;
    }
    out -= left;
    unsafe { (*strm).total_out += out as u64 };
    unsafe { (*state).total += out as u64 };

    if (unsafe { (*state).wrap } & 4) != 0 && out != 0 {
        let put_offset = unsafe { put.offset(-(out as isize)) };
        unsafe { (*state).check = adler32(unsafe { (*state).check }, put_offset, out) };
        unsafe { (*strm).adler = (*state).check };
    }
    out = left;

    if (unsafe { (*state).wrap } & 4) != 0 && 
       ((hold >> 24) & 0xff).wrapping_add((hold >> 8) & 0xff00).wrapping_add((hold & 0xff00) << 8).wrapping_add((hold & 0xff) << 24) != unsafe { (*state).check } {
        unsafe { (*strm).msg = b"incorrect data check\x00".as_ptr() as *mut std::os::raw::c_char };
        unsafe { (*state).mode = BAD };
        continue;
    } else {
        hold = 0;
        bits = 0;
    }
}
unsafe { (*state).mode = DONE };
current_block = 4955812790560533212;

                }
                16208 => { current_block = 4955812790560533212; }
                16209 => { ret = -(3 as std::os::raw::c_int); break ; }
                16210 => { return -(4 as std::os::raw::c_int) }
                16211 | _ => { return -(2 as std::os::raw::c_int) }
            }
            match current_block {
                9974864727789713748 => {
                    while (*state).have < (*state).ncode {
                        while bits < 3 as std::os::raw::c_int as std::os::raw::c_uint {
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                break 's_114 ;
                            }
                            have = have.wrapping_sub(1);
                            let fresh5 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*fresh5 as std::os::raw::c_ulong)
                                                      << bits);
                            bits =
                                bits.wrapping_add(8 as std::os::raw::c_int as
                                                      std::os::raw::c_uint)
                        }
                        let fresh6 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[fresh6 as usize] as usize] =
                            (hold as std::os::raw::c_uint &
                                 ((1 as std::os::raw::c_uint) <<
                                      3 as
                                          std::os::raw::c_int).wrapping_sub(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_uint))
                                as std::os::raw::c_ushort;
                        hold >>= 3 as std::os::raw::c_int;
                        bits =
                            bits.wrapping_sub(3 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    while (*state).have < 19 as std::os::raw::c_int as std::os::raw::c_uint {
                        let fresh7 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[fresh7 as usize] as usize] =
                            0 as std::os::raw::c_int as std::os::raw::c_ushort
                    }
                    (*state).next = (*state).codes.as_mut_ptr();
                    (*state).lencode = (*state).next as *const code;
                    (*state).lenbits = 7 as std::os::raw::c_int as std::os::raw::c_uint;
                    ret =
                        inflate_table(CODES, (*state).lens.as_mut_ptr(),
                                      19 as std::os::raw::c_int as std::os::raw::c_uint,
                                      &mut (*state).next,
                                      &mut (*state).lenbits,
                                      (*state).work.as_mut_ptr());
                    if ret != 0 {
                        (*strm).msg =
                            b"invalid code lengths set\x00" as *const u8 as
                                *const std::os::raw::c_char as *mut std::os::raw::c_char;
                        (*state).mode = BAD;
                        continue ;
                    } else {
                        (*state).have = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        (*state).mode = CODELENS
                    }
                    current_block = 1995330570110937187;
                }
                14331593601693766846 => {
                    if (*state).havedict == 0 as std::os::raw::c_int {
                        (*strm).next_out = put;
                        (*strm).avail_out = left;
                        (*strm).next_in = next;
                        (*strm).avail_in = have;
                        (*state).hold = hold;
                        (*state).bits = bits;
                        return 2 as std::os::raw::c_int
                    }
                    (*state).check =
                        adler32(0 as std::os::raw::c_long as uLong, 0 as *const Bytef,
                                0 as std::os::raw::c_int as uInt);
                    (*strm).adler = (*state).check;
                    (*state).mode = TYPE;
                    current_block = 210528378685203046;
                }
                16345656043779050082 => {
                    (*state).mode = COPY;
                    current_block = 7368450687049571364;
                }
                4955812790560533212 => { ret = 1 as std::os::raw::c_int; break ; }
                _ => { }
            }
            match current_block {
                1995330570110937187 => {
                    while unsafe { (*state).have } < unsafe { (*state).nlen + (*state).ndist } {
     loop  {
                            here =
                                *(*state).lencode.offset((hold as std::os::raw::c_uint
                                                              &
                                                              ((1 as
                                                                    std::os::raw::c_uint)
                                                                   <<
                                                                   (*state).lenbits).wrapping_sub(1
                                                                                                      as
                                                                                                      std::os::raw::c_int
                                                                                                      as
                                                                                                      std::os::raw::c_uint))
                                                             as isize);
                            if here.bits as std::os::raw::c_uint <= bits { break ; }
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                break 's_114 ;
                            }
                            have = have.wrapping_sub(1);
                            let fresh8 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*fresh8 as std::os::raw::c_ulong)
                                                      << bits);
                            bits =
                                bits.wrapping_add(8 as std::os::raw::c_int as
                                                      std::os::raw::c_uint)
                        }
                        if (here.val as std::os::raw::c_int) < 16 as std::os::raw::c_int {
                            hold >>= here.bits as std::os::raw::c_int;
                            bits =
                                bits.wrapping_sub(here.bits as std::os::raw::c_uint);
                            let fresh9 = (*state).have;
                            (*state).have = (*state).have.wrapping_add(1);
                            (*state).lens[fresh9 as usize] = here.val
                        } else {
                            if here.val == 16 {
     while bits < (here.bits as u32 + 2) {
    if have == 0 {
        break 's_114;
    }
    have -= 1;
    let fresh10 = unsafe { *next }; // Dereference the raw pointer to get the byte
    next = unsafe { next.add(1) }; // Move to the next byte
    hold += (fresh10 as u64) << bits;
    bits += 8;
}
hold >>= here.bits as u32;
bits -= here.bits as u32;

if unsafe { (*state).have } == 0 {
    unsafe { (*strm).msg = "invalid bit length repeat\0".as_ptr() as *mut std::os::raw::c_char };
    unsafe { (*state).mode = BAD };
    break;
} else {
    len = unsafe { (*state).lens[(unsafe { (*state).have } - 1) as usize] } as u32;
    copy = 3 + (hold as u32 & ((1 << 2) - 1));
    hold >>= 2;
    bits -= 2;
}


} else if here.val == 17 {
     while bits < (here.bits as u32 + 3) {
    if have == 0 {
        break 's_114;
    }
    have -= 1;
    let fresh11 = next;
    next = next.add(1);
    hold += (*fresh11 as u64) << bits;
    bits += 8;
}
hold >>= here.bits as u32;
bits -= here.bits as u32;
len = 0;
copy = 3 + (hold & ((1u64 << 3) - 1)) as u32;
hold >>= 3;
bits -= 3;


} else {
     while bits < (here.bits as u32 + 7) {
    if have == 0 {
        break 's_114;
    }
    have -= 1;
    let fresh12 = unsafe { *next }; // Dereference the raw pointer
    next = unsafe { next.add(1) }; // Move to the next byte
    hold += (fresh12 as u64) << bits;
    bits += 8;
}
hold >>= here.bits as u32;
bits -= here.bits as u32;
len = 0;
copy = 11 + (hold & ((1 << 7) - 1)) as u32;
hold >>= 7;
bits -= 7;


}

if (*state).have.wrapping_add(copy) > (*state).nlen.wrapping_add((*state).ndist) {
    (*strm).msg = "invalid bit length repeat\0".as_ptr() as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    (*state).mode = BAD;
    break;
} else {
    loop {
        let fresh13 = copy;
        copy = copy.wrapping_sub(1);
        if fresh13 == 0 { break; }
        let fresh14 = (*state).have;
        (*state).have = (*state).have.wrapping_add(1);
        (*state).lens[fresh14 as usize] = len as u16;
    }
}
/*
The variables live at this point are:
(mut strm: &mut src::libpng::png::z_stream_s, mut state: &mut src::zlib::infback::inflate_state, mut next: &mut u8, mut have: u32, mut hold: u64, mut bits: u32, mut copy: u32, mut here: src::zlib::infback::code, mut len: u32)
*/

                        }

}

/* handle error breaks in while */
if unsafe { (*state).mode } == BAD {
    continue;
}

/* check for end-of-block code (better have one) */
if unsafe { (*state).lens[256] } == 0 {
    unsafe { (*strm).msg = "invalid code -- missing end-of-block".as_ptr() as *const i8 as *mut i8 };
    unsafe { (*state).mode = BAD };
    continue;
} else {
    /* build code tables -- note: do not change the lenbits or distbits
    values here (9 and 6) without reading the comments in inftrees.h
    concerning the ENOUGH constants, which depend on those values */
    unsafe { (*state).next = (*state).codes.as_mut_ptr() };
    unsafe { (*state).lencode = (*state).next as *const code };
    unsafe { (*state).lenbits = 9 };
    
    ret = inflate_table(LENS, unsafe { (*state).lens.as_mut_ptr() }, unsafe { (*state).nlen }, &mut unsafe { (*state).next }, &mut unsafe { (*state).lenbits }, unsafe { (*state).work.as_mut_ptr() });
    if ret != 0 {
        unsafe { (*strm).msg = "invalid literal/lengths set".as_ptr() as *const i8 as *mut i8 };
        unsafe { (*state).mode = BAD };
        continue;
    } else {
        unsafe { (*state).distcode = (*state).next as *const code };
        unsafe { (*state).distbits = 6 };
        
        ret = inflate_table(DISTS, unsafe { (*state).lens.as_mut_ptr().offset(unsafe { (*state).nlen } as isize) }, unsafe { (*state).ndist }, &mut unsafe { (*state).next }, &mut unsafe { (*state).distbits }, unsafe { (*state).work.as_mut_ptr() });
        if ret != 0 {
            unsafe { (*strm).msg = "invalid distances set".as_ptr() as *const i8 as *mut i8 };
            unsafe { (*state).mode = BAD };
            continue;
        } else {
            unsafe { (*state).mode = LEN_ };
            if flush == 6 { break; }
        }
    }
}
current_block = 1692384543052803397;
/*
The variables live at this point are:
(mut strm: *mut src::libpng::png::z_stream_s, mut flush: i32, mut current_block: u64, mut state: *mut src::zlib::infback::inflate_state, mut next: *mut u8, mut have: u32, mut hold: u64, mut bits: u32, mut copy: u32, mut here: src::zlib::infback::code, mut len: u32, mut ret: i32)
*/

                }
                7368450687049571364 => {
                    let copy = unsafe { (*state).length };
if copy != 0 {
    let copy = if copy > have { have } else { copy };
    let copy = if copy > left { left } else { copy };
    if copy == 0 {
        break;
    }
    let next_slice = unsafe { std::slice::from_raw_parts(next, copy as usize) };
    let put_slice = unsafe { std::slice::from_raw_parts_mut(put, copy as usize) };
    put_slice.copy_from_slice(next_slice);
    have = have.wrapping_sub(copy);
    next = next.add(copy as usize);
    left = left.wrapping_sub(copy);
    put = put.add(copy as usize);
    unsafe { (*state).length = (*state).length.wrapping_sub(copy); }
    continue;
} else {
    unsafe { (*state).mode = TYPE; }
    continue;
}

                }
                210528378685203046 => {
                    if flush == 5 as std::os::raw::c_int || flush == 6 as std::os::raw::c_int
                       {
                        break ;
                    }
                    current_block = 6853415491165023619;
                }
                _ => { }
            }
            match current_block {
    6853415491165023619 => {
        if unsafe { (*state).last } != 0 {
            hold >>= bits & 7;
            bits = bits.wrapping_sub(bits & 7);
            unsafe { (*state).mode = CHECK };
            continue;
        } else {
            while bits < 3 {
                if have == 0 {
                    break 's_114;
                }
                have = have.wrapping_sub(1);
                let fresh2 = next;
                next = next.add(1);
                hold = hold.wrapping_add((fresh2 as u64) << bits);
                bits = bits.wrapping_add(8);
            }
            unsafe {
                (*state).last = (hold as u32 & ((1u32 << 1) - 1)) as i32;
                hold >>= 1;
                bits = bits.wrapping_sub(1);
                match hold as u32 & ((1u32 << 2) - 1) {
                    0 => {
                        // stored block
                        (*state).mode = STORED;
                    }
                    1 => {
                        // fixed block
                        fixedtables(state); // decode codes
                        (*state).mode = LEN_;
                        if flush == 6 {
                            hold >>= 2;
                            bits = bits.wrapping_sub(2);
                            break;
                        }
                    }
                    2 => {
                        // dynamic block
                        (*state).mode = TABLE;
                    }
                    3 => {
                        (*strm).msg = "invalid block type\0".as_ptr() as *mut i8;
                        (*state).mode = BAD;
                    }
                    _ => {}
                }
                hold >>= 2;
                bits = bits.wrapping_sub(2);
            }
            continue;
        }
    }
    1692384543052803397 => {
        unsafe { (*state).mode = LEN };
        current_block = 7212245502802746503;
    }
    _ => {}
}

            match current_block {
                7212245502802746503 => {
                    if have >= 6 as std::os::raw::c_int as std::os::raw::c_uint &&
                           left >= 258 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*strm).next_out = put;
                        (*strm).avail_out = left;
                        (*strm).next_in = next;
                        (*strm).avail_in = have;
                        (*state).hold = hold;
                        (*state).bits = bits;
                        inflate_fast(strm, out);
                        put = (*strm).next_out;
                        left = (*strm).avail_out;
                        next = (*strm).next_in as *mut u8;
                        have = (*strm).avail_in;
                        hold = (*state).hold;
                        bits = (*state).bits;
                        if (*state).mode as std::os::raw::c_uint ==
                               TYPE as std::os::raw::c_int as std::os::raw::c_uint {
                            (*state).back = -(1 as std::os::raw::c_int)
                        }
                        continue ;
                    } else {
                        (*state).back = 0 as std::os::raw::c_int;
                        loop  {
                            here =
                                *(*state).lencode.offset((hold as std::os::raw::c_uint
                                                              &
                                                              ((1 as
                                                                    std::os::raw::c_uint)
                                                                   <<
                                                                   (*state).lenbits).wrapping_sub(1
                                                                                                      as
                                                                                                      std::os::raw::c_int
                                                                                                      as
                                                                                                      std::os::raw::c_uint))
                                                             as isize);
                            if here.bits as std::os::raw::c_uint <= bits { break ; }
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                break 's_114 ;
                            }
                            have = have.wrapping_sub(1);
                            let fresh15 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*fresh15 as std::os::raw::c_ulong)
                                                      << bits);
                            bits =
                                bits.wrapping_add(8 as std::os::raw::c_int as
                                                      std::os::raw::c_uint)
                        }
                        if here.op as std::os::raw::c_int != 0 &&
                               here.op as std::os::raw::c_int & 0xf0 as std::os::raw::c_int ==
                                   0 as std::os::raw::c_int {
                            last = here;
                            loop  {
                                here =
                                    *(*state).lencode.offset((last.val as
                                                                  std::os::raw::c_uint).wrapping_add((hold
                                                                                                  as
                                                                                                  std::os::raw::c_uint
                                                                                                  &
                                                                                                  ((1
                                                                                                        as
                                                                                                        std::os::raw::c_uint)
                                                                                                       <<
                                                                                                       last.bits
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           +
                                                                                                           last.op
                                                                                                               as
                                                                                                               std::os::raw::c_int).wrapping_sub(1
                                                                                                                                             as
                                                                                                                                             std::os::raw::c_int
                                                                                                                                             as
                                                                                                                                             std::os::raw::c_uint))
                                                                                                 >>
                                                                                                 last.bits
                                                                                                     as
                                                                                                     std::os::raw::c_int)
                                                                 as isize);
                                if (last.bits as std::os::raw::c_int +
                                        here.bits as std::os::raw::c_int) as
                                       std::os::raw::c_uint <= bits {
                                    break ;
                                }
                                if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    break 's_114 ;
                                }
                                have = have.wrapping_sub(1);
                                let fresh16 = next;
                                next = next.offset(1);
                                hold =
                                    hold.wrapping_add((*fresh16 as
                                                           std::os::raw::c_ulong) <<
                                                          bits);
                                bits =
                                    bits.wrapping_add(8 as std::os::raw::c_int as
                                                          std::os::raw::c_uint)
                            }
                            hold >>= last.bits as std::os::raw::c_int;
                            bits =
                                bits.wrapping_sub(last.bits as std::os::raw::c_uint);
                            (*state).back += last.bits as std::os::raw::c_int
                        }
                        hold >>= here.bits as std::os::raw::c_int;
                        bits = bits.wrapping_sub(here.bits as std::os::raw::c_uint);
                        (*state).back += here.bits as std::os::raw::c_int;
                        (*state).length = here.val as std::os::raw::c_uint;
                        if here.op as std::os::raw::c_int == 0 as std::os::raw::c_int {
                            (*state).mode = LIT;
                            continue ;
                        } else if here.op as std::os::raw::c_int & 32 as std::os::raw::c_int
                                      != 0 {
                            (*state).back = -(1 as std::os::raw::c_int);
                            (*state).mode = TYPE;
                            continue ;
                        } else if here.op as std::os::raw::c_int & 64 as std::os::raw::c_int
                                      != 0 {
                            (*strm).msg =
                                b"invalid literal/length code\x00" as
                                    *const u8 as *const std::os::raw::c_char as
                                    *mut std::os::raw::c_char;
                            (*state).mode = BAD;
                            continue ;
                        } else {
                            (*state).extra =
                                here.op as std::os::raw::c_uint &
                                    15 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*state).mode = LENEXT
                        }
                    }
                    current_block = 7259693266422973518;
                }
                _ => { }
            }
            match current_block {
    7259693266422973518 => {
        if unsafe { (*state).extra } != 0 {
            while bits < unsafe { (*state).extra } {
                if have == 0 {
                    break 's_114;
                }
                have -= 1;
                let fresh17 = next;
                next = unsafe { next.add(1) };
                hold += (unsafe { *fresh17 } as u64) << bits;
                bits += 8;
            }
            unsafe {
                (*state).length += (hold as u32) & ((1u32 << (*state).extra) - 1);
                hold >>= (*state).extra;
                bits -= (*state).extra;
                (*state).back += (*state).extra as i32;
            }
        }
        unsafe {
            (*state).was = (*state).length;
            (*state).mode = DIST;
        }
        current_block = 14213006019499569531;
    }
    _ => {}
}
loop {
    match current_block {
        13874176978779739039 => {
             if (*state).extra != 0 {
    while bits < (*state).extra {
        if have == 0 {
            break 's_114;
        }
        have -= 1;
        let fresh20 = next;
        next = next.add(1);
        hold += (*fresh20 as u64) << bits;
        bits += 8;
    }
    (*state).offset += (hold as u32) & ((1u32 << (*state).extra) - 1);
    hold >>= (*state).extra;
    bits -= (*state).extra;
    (*state).back += (*state).extra as i32;
}
(*state).mode = MATCH;
current_block = 3519113796920557345;


        }
        14213006019499569531 => {
             here =
                            *(*state).distcode.offset((hold as std::os::raw::c_uint &
                                                           ((1 as
                                                                 std::os::raw::c_uint)
                                                                <<
                                                                (*state).distbits).wrapping_sub(1
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_uint))
                                                          as isize);
                        if here.bits as std::os::raw::c_uint <= bits {
                            if here.op as std::os::raw::c_int & 0xf0 as std::os::raw::c_int ==
                                   0 as std::os::raw::c_int {
                                last = here;
                                loop  {
                                    here =
                                        *(*state).distcode.offset((last.val as
                                                                       std::os::raw::c_uint).wrapping_add((hold
                                                                                                       as
                                                                                                       std::os::raw::c_uint
                                                                                                       &
                                                                                                       ((1
                                                                                                             as
                                                                                                             std::os::raw::c_uint)
                                                                                                            <<
                                                                                                            last.bits
                                                                                                                as
                                                                                                                std::os::raw::c_int
                                                                                                                +
                                                                                                                last.op
                                                                                                                    as
                                                                                                                    std::os::raw::c_int).wrapping_sub(1
                                                                                                                                                  as
                                                                                                                                                  std::os::raw::c_int
                                                                                                                                                  as
                                                                                                                                                  std::os::raw::c_uint))
                                                                                                      >>
                                                                                                      last.bits
                                                                                                          as
                                                                                                          std::os::raw::c_int)
                                                                      as
                                                                      isize);
                                    if (last.bits as std::os::raw::c_int +
                                            here.bits as std::os::raw::c_int) as
                                           std::os::raw::c_uint <= bits {
                                        break ;
                                    }
                                    if have ==
                                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                                        break 's_114 ;
                                    }
                                    have = have.wrapping_sub(1);
                                    let fresh19 = next;
                                    next = next.offset(1);
                                    hold =
                                        hold.wrapping_add((*fresh19 as
                                                               std::os::raw::c_ulong)
                                                              << bits);
                                    bits =
                                        bits.wrapping_add(8 as std::os::raw::c_int as
                                                              std::os::raw::c_uint)
                                }
                                hold >>= last.bits as std::os::raw::c_int;
                                bits =
                                    bits.wrapping_sub(last.bits as
                                                          std::os::raw::c_uint);
                                (*state).back += last.bits as std::os::raw::c_int
                            }
                            hold >>= here.bits as std::os::raw::c_int;
                            bits =
                                bits.wrapping_sub(here.bits as std::os::raw::c_uint);
                            (*state).back += here.bits as std::os::raw::c_int;
                            if here.op as std::os::raw::c_int & 64 as std::os::raw::c_int != 0
                               {
                                (*strm).msg =
                                    b"invalid distance code\x00" as *const u8
                                        as *const std::os::raw::c_char as
                                        *mut std::os::raw::c_char;
                                (*state).mode = BAD;
                                continue 's_114 ;
                            } else {
                                (*state).offset = here.val as std::os::raw::c_uint;
                                (*state).extra =
                                    here.op as std::os::raw::c_uint &
                                        15 as std::os::raw::c_int as std::os::raw::c_uint;
                                (*state).mode = DISTEXT;
                                current_block = 13874176978779739039;
                            }
                        } else {
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                break 's_114 ;
                            }
                            have = have.wrapping_sub(1);
                            let fresh18 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*fresh18 as std::os::raw::c_ulong)
                                                      << bits);
                            bits =
                                bits.wrapping_add(8 as std::os::raw::c_int as
                                                      std::os::raw::c_uint);
                            current_block = 14213006019499569531;
                        }

        }
        _ => {
             if left == 0 {
    break 's_114;
}
copy = out.wrapping_sub(left);
if state as *const _ as usize > copy as usize { // Compare the pointer value with copy
    current_block = 937269053449267811;
    break;
} else {
    current_block = 1863993923982565458;
    break;
}


        }
    }
}
match current_block {
    1863993923982565458 => {
        /* copy from output */
        from = unsafe { put.offset(-(unsafe { (*state).offset } as isize)) };
        copy = unsafe { (*state).length };
    }
    _ => {
        /* copy from window */
        copy = unsafe { (*state).offset.wrapping_sub(copy) };
        if copy > unsafe { (*state).whave } {
            if unsafe { (*state).sane } != 0 {
                unsafe {
                    (*strm).msg = b"invalid distance too far back\x00" as *const u8 as *const i8 as *mut i8;
                    (*state).mode = BAD;
                }
                continue;
            }
        }
        if copy > unsafe { (*state).wnext } {
            copy -= unsafe { (*state).wnext };
            from = unsafe { (*state).window.offset(((*state).wsize.wrapping_sub(copy)) as isize) };
        } else {
            from = unsafe { (*state).window.offset(((*state).wnext.wrapping_sub(copy)) as isize) };
        }
        if copy > unsafe { (*state).length } {
            copy = unsafe { (*state).length };
        }
    }
}
if copy > left {
    copy = left;
}
left -= copy;
unsafe { (*state).length -= copy; }
loop {
    let fresh21 = from;
    from = unsafe { from.add(1) };
    let fresh22 = put;
    put = unsafe { put.add(1) };
    unsafe { *fresh22 = *fresh21 };
    copy -= 1;
    if copy == 0 {
        break;
    }
}
if unsafe { (*state).length } == 0 {
    unsafe { (*state).mode = LEN; }
}
/*
The variables live at this point are:
(mut strm: *mut src::libpng::png::z_stream_s, mut current_block: u64, mut state: *mut src::zlib::infback::inflate_state, mut next: *mut u8, mut put: *mut u8, mut have: u32, mut left: u32, mut hold: u64, mut bits: u32, mut out: u32, mut copy: u32, mut from: *mut u8, mut here: src::zlib::infback::code, mut last: src::zlib::infback::code)
*/

        }
    /*
       Return from inflate(), updating the total counts and the check value.
       If there was no progress during the inflate() call, return a buffer
       error.  Call updatewindow() to create and/or update the window state.
       Note: a memory error from inflate() is non-recoverable.
     */
    (*strm).next_out = put;
    (*strm).avail_out = left;
    (*strm).next_in = next;
    (*strm).avail_in = have;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0 ||
           out != (*strm).avail_out &&
               ((*state).mode as std::os::raw::c_uint) <
                   BAD as std::os::raw::c_int as std::os::raw::c_uint &&
               (((*state).mode as std::os::raw::c_uint) <
                    CHECK as std::os::raw::c_int as std::os::raw::c_uint ||
                    flush != 4 as std::os::raw::c_int) {
        if updatewindow(strm, (*strm).next_out,
                        out.wrapping_sub((*strm).avail_out)) != 0 {
            (*state).mode = MEM;
            return -(4 as std::os::raw::c_int)
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in);
    out = out.wrapping_sub((*strm).avail_out);
    (*strm).total_in =
        ((*strm).total_in as
             std::os::raw::c_ulong).wrapping_add(in_0 as std::os::raw::c_ulong) as uLong as
            uLong;
    (*strm).total_out =
        ((*strm).total_out as
             std::os::raw::c_ulong).wrapping_add(out as std::os::raw::c_ulong) as uLong as
            uLong;
    (*state).total = (*state).total.wrapping_add(out as std::os::raw::c_ulong);
    if (*state).wrap & 4 as std::os::raw::c_int != 0 && out != 0 {
        (*state).check =
            adler32((*state).check, (*strm).next_out.offset(-(out as isize)),
                    out);
        (*strm).adler = (*state).check
    }
    (*strm).data_type =
        (*state).bits as std::os::raw::c_int +
            (if (*state).last != 0 {
                 64 as std::os::raw::c_int
             } else { 0 as std::os::raw::c_int }) +
            (if (*state).mode as std::os::raw::c_uint ==
                    TYPE as std::os::raw::c_int as std::os::raw::c_uint {
                 128 as std::os::raw::c_int
             } else { 0 as std::os::raw::c_int }) +
            (if (*state).mode as std::os::raw::c_uint ==
                    LEN_ as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*state).mode as std::os::raw::c_uint ==
                        COPY_ as std::os::raw::c_int as std::os::raw::c_uint {
                 256 as std::os::raw::c_int
             } else { 0 as std::os::raw::c_int });
    if (in_0 == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
            out == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
            flush == 4 as std::os::raw::c_int) && ret == 0 as std::os::raw::c_int {
        ret = -(5 as std::os::raw::c_int)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn inflateEnd(mut strm: z_streamp) -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    if !(*state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                    (*state).window
                                                                                                        as
                                                                                                        voidpf);
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                (*strm).state
                                                                                                    as
                                                                                                    voidpf);
    (*strm).state = 0 as *mut internal_state;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub fn inflateGetDictionary(strm: z_streamp,
                             dictionary: &mut [u8],
                             dictLength: Option<&mut uInt>)
                             -> std::os::raw::c_int {
    unsafe {
        if inflateStateCheck(strm) != 0 {
            return -(2 as std::os::raw::c_int);
        }

        let state = (*strm).state as *mut inflate_state;
        let state_ref = &*state;

        if state_ref.whave != 0 {
            let dict_len = dictionary.len() as uInt;
            let copy_len = state_ref.whave.wrapping_sub(state_ref.wnext) as usize;

            if dict_len >= copy_len as uInt {
                let window_slice = std::slice::from_raw_parts(
                    state_ref.window.offset(state_ref.wnext as isize) as *const u8,
                    copy_len
                );
                dictionary[..copy_len].copy_from_slice(window_slice);
                if state_ref.wnext > 0 {
                    let wrap_around_slice = std::slice::from_raw_parts(
                        state_ref.window as *const u8,
                        state_ref.wnext as usize
                    );
                    dictionary[copy_len..copy_len + state_ref.wnext as usize].copy_from_slice(wrap_around_slice);
                }
            }
        }

        if let Some(length) = dictLength {
            *length = state_ref.whave;
        }
    }

    0 as std::os::raw::c_int
}

#[no_mangle]
pub unsafe extern "C" fn inflateSetDictionary(mut strm: z_streamp,
                                              mut dictionary: *const Bytef,
                                              mut dictLength: uInt)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut dictid: std::os::raw::c_ulong = 0;
    let mut ret: std::os::raw::c_int = 0;
    /* check state */
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap != 0 as std::os::raw::c_int &&
           (*state).mode as std::os::raw::c_uint !=
               DICT as std::os::raw::c_int as std::os::raw::c_uint {
        return -(2 as std::os::raw::c_int)
    }
    /* check for correct dictionary identifier */
    if (*state).mode as std::os::raw::c_uint == DICT as std::os::raw::c_int as std::os::raw::c_uint {
        dictid =
            adler32(0 as std::os::raw::c_long as uLong, 0 as *const Bytef,
                    0 as std::os::raw::c_int as uInt);
        dictid = adler32(dictid, dictionary, dictLength);
        if dictid != (*state).check { return -(3 as std::os::raw::c_int) }
    }
    /* copy dictionary to window using updatewindow(), which will amend the
       existing dictionary if appropriate */
    ret =
        updatewindow(strm, dictionary.offset(dictLength as isize),
                     dictLength);
    if ret != 0 { (*state).mode = MEM; return -(4 as std::os::raw::c_int) }
    (*state).havedict = 1 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateGetHeader(mut strm: z_streamp,
                                          mut head: gz_headerp)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    /* check state */
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap & 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    /* save header structure */
    (*state).head = head;
    (*head).done = 0 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
/*
   Search buf[0..len-1] for the pattern: 0, 0, 0xff, 0xff.  Return when found
   or when out of input.  When called, *have is the number of pattern bytes
   found in order so far, in 0..3.  On return *have is updated to the new
   state.  If on return *have equals four, then the pattern was found and the
   return value is how many bytes were read including the last byte of the
   pattern.  If *have is less than four, then the pattern has not been found
   yet and the return value is len.  In the latter case, syncsearch() can be
   called again with more data and the *have state.  *have is initialized to
   zero for the first call.
 */
fn syncsearch(have: &mut u32, buf: &[u8], len: u32) -> u32 {
    let mut got: u32 = *have; // number of bytes to look at or looked at
    let mut next: u32 = 0; // temporary to save total_in and total_out
    while next < len && got < 4 {
        if buf[next as usize] as i32 == if got < 2 { 0 } else { 0xff } {
            got = got.wrapping_add(1);
        } else if buf[next as usize] != 0 {
            got = 0;
        } else {
            got = 4_u32.wrapping_sub(got);
        }
        next = next.wrapping_add(1);
    }
    *have = got;
    next
}

#[no_mangle]
pub unsafe extern "C" fn inflateSync(mut strm: z_streamp) -> std::os::raw::c_int {
    let mut len: std::os::raw::c_uint = 0;
    let mut in_0: std::os::raw::c_ulong = 0;
    let mut out: std::os::raw::c_ulong = 0;
    let mut buf: [std::os::raw::c_uchar; 4] = [0; 4];
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    /* check parameters */
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    if (*strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*state).bits < 8 as std::os::raw::c_int as std::os::raw::c_uint {
        return -(5 as std::os::raw::c_int)
    }
    /* if first time, start search in bit buffer */
    if (*state).mode as std::os::raw::c_uint != SYNC as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).mode = SYNC;
        (*state).hold <<= (*state).bits & 7 as std::os::raw::c_int as std::os::raw::c_uint;
        (*state).bits =
            (*state).bits.wrapping_sub((*state).bits &
                                           7 as std::os::raw::c_int as std::os::raw::c_uint);
        len = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while (*state).bits >= 8 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh25 = len;
            len = len.wrapping_add(1);
            buf[fresh25 as usize] = (*state).hold as std::os::raw::c_uchar;
            (*state).hold >>= 8 as std::os::raw::c_int;
            (*state).bits =
                (*state).bits.wrapping_sub(8 as std::os::raw::c_int as std::os::raw::c_uint)
        }
        (*state).have = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        syncsearch(&mut (*state).have, &buf[..], len);
    }
    /* search available input */
    let len = syncsearch(&mut (*state).have, std::slice::from_raw_parts((*strm).next_in, (*strm).avail_in as usize), (*strm).avail_in);
    (*strm).avail_in =
        ((*strm).avail_in as std::os::raw::c_uint).wrapping_sub(len) as uInt as uInt;
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in =
        ((*strm).total_in as std::os::raw::c_ulong).wrapping_add(len as std::os::raw::c_ulong)
            as uLong as uLong;
    /* return no joy or set up to restart inflate() on a new block */
    if (*state).have != 4 as std::os::raw::c_int as std::os::raw::c_uint {
        return -(3 as std::os::raw::c_int)
    }
    in_0 = (*strm).total_in;
    out = (*strm).total_out;
    inflateReset(strm);
    (*strm).total_in = in_0;
    (*strm).total_out = out;
    (*state).mode = TYPE;
    return 0 as std::os::raw::c_int;
}
/*
   Returns true if inflate is currently at the end of a block generated by
   Z_SYNC_FLUSH or Z_FULL_FLUSH. This function is used by one PPP
   implementation to provide an additional safety check. PPP uses
   Z_SYNC_FLUSH but removes the length bytes of the resulting empty stored
   block. When decompressing, PPP checks that at the end of input packet,
   inflate is waiting for these length bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn inflateSyncPoint(mut strm: z_streamp)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    return ((*state).mode as std::os::raw::c_uint ==
                STORED as std::os::raw::c_int as std::os::raw::c_uint &&
                (*state).bits == 0 as std::os::raw::c_int as std::os::raw::c_uint) as
               std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateCopy(mut dest: z_streamp,
                                     mut source: z_streamp) -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut copy: *mut inflate_state = 0 as *mut inflate_state;
    let mut window: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut wsize: std::os::raw::c_uint = 0;
    /* check input */
    if inflateStateCheck(source) != 0 || dest.is_null() {
        return -(2 as std::os::raw::c_int)
    }
    state = (*source).state as *mut inflate_state;
    /* allocate space */
    copy =
        Some((*source).zalloc.expect("non-null function pointer")).expect("non-null function pointer")((*source).opaque,
                                                                                                       1
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           uInt,
                                                                                                       ::std::mem::size_of::<inflate_state>()
                                                                                                           as
                                                                                                           std::os::raw::c_ulong
                                                                                                           as
                                                                                                           uInt)
            as *mut inflate_state;
    if copy.is_null() { return -(4 as std::os::raw::c_int) }
    window = 0 as *mut std::os::raw::c_uchar;
    if !(*state).window.is_null() {
        window =
            Some((*source).zalloc.expect("non-null function pointer")).expect("non-null function pointer")((*source).opaque,
                                                                                                           (1
                                                                                                                as
                                                                                                                std::os::raw::c_uint)
                                                                                                               <<
                                                                                                               (*state).wbits,
                                                                                                           ::std::mem::size_of::<std::os::raw::c_uchar>()
                                                                                                               as
                                                                                                               std::os::raw::c_ulong
                                                                                                               as
                                                                                                               uInt)
                as *mut std::os::raw::c_uchar;
        if window.is_null() {
            Some((*source).zfree.expect("non-null function pointer")).expect("non-null function pointer")((*source).opaque,
                                                                                                          copy
                                                                                                              as
                                                                                                              voidpf);
            return -(4 as std::os::raw::c_int)
        }
    }
    /* copy state */
    memcpy(dest as voidpf, source as voidpf as *const std::os::raw::c_void,
           ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong);
    memcpy(copy as voidpf, state as voidpf as *const std::os::raw::c_void,
           ::std::mem::size_of::<inflate_state>() as std::os::raw::c_ulong);
    (*copy).strm = dest;
    if (*state).lencode >= (*state).codes.as_mut_ptr() as *const code &&
           (*state).lencode <=
               (*state).codes.as_mut_ptr().offset((852 as std::os::raw::c_int +
                                                       592 as std::os::raw::c_int) as
                                                      isize).offset(-(1 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize))
                   as *const code {
        (*copy).lencode =
            (*copy).codes.as_mut_ptr().offset((*state).lencode.offset_from((*state).codes.as_mut_ptr())
                                                  as std::os::raw::c_long as isize);
        (*copy).distcode =
            (*copy).codes.as_mut_ptr().offset((*state).distcode.offset_from((*state).codes.as_mut_ptr())
                                                  as std::os::raw::c_long as isize)
    }
    (*copy).next =
        (*copy).codes.as_mut_ptr().offset((*state).next.offset_from((*state).codes.as_mut_ptr())
                                              as std::os::raw::c_long as isize);
    if !window.is_null() {
        wsize = (1 as std::os::raw::c_uint) << (*state).wbits;
        memcpy(window as *mut std::os::raw::c_void,
               (*state).window as *const std::os::raw::c_void,
               wsize as std::os::raw::c_ulong);
    }
    (*copy).window = window;
    (*dest).state = copy as *mut internal_state;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateUndermine(mut strm: z_streamp,
                                          mut subvert: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    (*state).sane = 1 as std::os::raw::c_int;
    return -(3 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn inflateValidate(mut strm: z_streamp,
                                         mut check: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 { return -(2 as std::os::raw::c_int) }
    state = (*strm).state as *mut inflate_state;
    if check != 0 {
        (*state).wrap |= 4 as std::os::raw::c_int
    } else { (*state).wrap &= !(4 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateMark(mut strm: z_streamp) -> std::os::raw::c_long {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return -((1 as std::os::raw::c_long) << 16 as std::os::raw::c_int)
    }
    state = (*strm).state as *mut inflate_state;
    return (((*state).back as std::os::raw::c_long as std::os::raw::c_ulong) <<
                16 as std::os::raw::c_int) as std::os::raw::c_long +
               (if (*state).mode as std::os::raw::c_uint ==
                       COPY as std::os::raw::c_int as std::os::raw::c_uint {
                    (*state).length
                } else {
                    (if (*state).mode as std::os::raw::c_uint ==
                            MATCH as std::os::raw::c_int as std::os::raw::c_uint {
                         (*state).was.wrapping_sub((*state).length)
                     } else { 0 as std::os::raw::c_int as std::os::raw::c_uint })
                }) as std::os::raw::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn inflateCodesUsed(mut strm: z_streamp)
 -> std::os::raw::c_ulong {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return -(1 as std::os::raw::c_int) as std::os::raw::c_ulong
    }
    state = (*strm).state as *mut inflate_state;
    return (*state).next.offset_from((*state).codes.as_mut_ptr()) as
               std::os::raw::c_long as std::os::raw::c_ulong;
}
