
extern "C" {
    
    
    
    
    
    
    
    #[no_mangle]
    fn getc(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn ungetc(__c: std::os::raw::c_int, __stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fseek(__stream: * mut crate::src::gifread::gifread::_IO_FILE, __off: std::os::raw::c_long, __whence: std::os::raw::c_int)
     -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn memset(_: * mut core::ffi::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
}
pub use crate::src::libpng::png::png_set_sig_bytes;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngread::png_read_png;
pub use crate::src::libpng::pngrio::png_set_read_fn;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::pngset::png_set_PLTE;
pub use crate::src::libpng::pngset::png_set_sBIT;
pub use crate::src::pngxtern::pngxmem::pngx_malloc_rows_extended;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
pub type png_byte = std::os::raw::c_uchar;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_size_t = std::os::raw::c_ulong;
pub type png_alloc_size_t = std::os::raw::c_ulong;
pub type png_voidp = * mut core::ffi::c_void;
pub type png_bytep = * mut std::os::raw::c_uchar;
pub type png_const_charp = * const std::os::raw::c_char;
pub type png_bytepp = * mut * mut std::os::raw::c_uchar;
pub type png_const_charpp = * mut * const std::os::raw::c_char;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::png::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color_struct;
pub type png_const_colorp = * const crate::src::libpng::png::png_color_struct;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::png::png_color_8_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8_struct;
pub type png_const_color_8p = * const crate::src::libpng::png::png_color_8_struct;
pub type png_rw_ptr<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 mut std::os::raw::c_uchar>,_: std::os::raw::c_ulong,) -> ()>;
pub type pngx_alloc_size_t = std::os::raw::c_ulong;
/* PNG format                */
/* ****************************************************************************/
/* BMP memory utilities                                                      */
/* ****************************************************************************/
unsafe extern "C" fn bmp_get_word(mut ptr: * mut std::os::raw::c_uchar) -> std::os::raw::c_uint {
    return (*ptr.offset(0 as std::os::raw::c_int as isize) as
                std::os::raw::c_uint).wrapping_add((*ptr.offset(1 as std::os::raw::c_int as
                                                            isize) as
                                                std::os::raw::c_uint) <<
                                               8 as std::os::raw::c_int);
}
unsafe extern "C" fn bmp_get_dword(mut ptr: * mut std::os::raw::c_uchar) -> std::os::raw::c_uint {
    return (*ptr.offset(0 as std::os::raw::c_int as isize) as
                png_uint_32).wrapping_add((*ptr.offset(1 as std::os::raw::c_int as
                                                           isize) as
                                               png_uint_32) <<
                                              8 as
                                                  std::os::raw::c_int).wrapping_add((*ptr.offset(2
                                                                                             as
                                                                                             std::os::raw::c_int
                                                                                             as
                                                                                             isize)
                                                                                 as
                                                                                 png_uint_32)
                                                                                <<
                                                                                16
                                                                                    as
                                                                                    std::os::raw::c_int).wrapping_add((*ptr.offset(3
                                                                                                                               as
                                                                                                                               std::os::raw::c_int
                                                                                                                               as
                                                                                                                               isize)
                                                                                                                   as
                                                                                                                   png_uint_32)
                                                                                                                  <<
                                                                                                                  24
                                                                                                                      as
                                                                                                                      std::os::raw::c_int);
}
/* ****************************************************************************/
/* BMP helpers                                                               */
/* ****************************************************************************/
unsafe extern "C" fn bmp_memset_bytes(mut ptr: * mut std::os::raw::c_uchar, mut offset: std::os::raw::c_ulong,
                                      mut ch: std::os::raw::c_int, mut len: std::os::raw::c_ulong) {
    memset(ptr.offset(offset as isize) as *mut std::os::raw::c_void, ch, len);
}
unsafe extern "C" fn bmp_memset_halfbytes(mut ptr: * mut std::os::raw::c_uchar,
                                          mut offset: std::os::raw::c_ulong,
                                          mut ch: std::os::raw::c_int,
                                          mut len: std::os::raw::c_ulong) {
    if len == 0 as std::os::raw::c_int as std::os::raw::c_ulong { return }
    ptr =
        ptr.offset(offset.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                       isize);
    if offset & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
        /* use half-byte operations at odd offset */
        *ptr =
            (*ptr as std::os::raw::c_int & 0xf0 as std::os::raw::c_int |
                 ch & 0xf as std::os::raw::c_int) as png_byte; /* skip padding */
        ch =
            (ch & 0xf0 as std::os::raw::c_int) >> 4 as std::os::raw::c_int |
                (ch & 0xf as std::os::raw::c_int) << 4 as std::os::raw::c_int;
        ptr = ptr.offset(1);
        len = len.wrapping_sub(1)
    }
    memset(ptr as *mut std::os::raw::c_void, ch,
           len.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong));
    if len & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
        *ptr.offset(len.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                        isize) = (ch & 0xf0 as std::os::raw::c_int) as png_byte
    };
}
unsafe extern "C" fn bmp_fread_bytes(mut ptr: * mut std::os::raw::c_uchar, mut offset: std::os::raw::c_ulong,
                                     mut len: std::os::raw::c_ulong, mut stream: * mut crate::src::gifread::gifread::_IO_FILE)
 -> std::os::raw::c_ulong {
    let mut result: u64 = 0;
    result =
        fread(ptr.offset(offset as isize) as *mut std::os::raw::c_void,
              1 as std::os::raw::c_int as std::os::raw::c_ulong, len, stream);
    if len & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 { getc(stream); }
    return result;
}
unsafe extern "C" fn bmp_fread_halfbytes(mut ptr: * mut std::os::raw::c_uchar,
                                         mut offset: std::os::raw::c_ulong, mut len: std::os::raw::c_ulong,
                                         mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong {
    let mut result: u64 = 0;
    let mut ch: i32 = 0;
    if len == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as size_t
    }
    ptr =
        ptr.offset(offset.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                       isize);
    if offset & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
        /* use half-byte operations at odd offset */
        result = 0 as std::os::raw::c_int as size_t; /* skip padding */
        while result < len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
            ch = getc(stream);
            if ch == -(1 as std::os::raw::c_int) { break ; }
            *ptr =
                (*ptr as std::os::raw::c_int & 0xf0 as std::os::raw::c_int |
                     (ch & 0xf0 as std::os::raw::c_int) >> 4 as std::os::raw::c_int) as
                    png_byte;
            ptr = ptr.offset(1);
            *ptr =
                ((ch & 0xf as std::os::raw::c_int) << 4 as std::os::raw::c_int) as png_byte;
            result =
                (result as
                     std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t
        }
    } else {
        result =
            fread(ptr as *mut std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
                  len.wrapping_add(1 as std::os::raw::c_int as
                                       std::os::raw::c_ulong).wrapping_div(2 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong),
                  stream).wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    if len & 2 as std::os::raw::c_int as std::os::raw::c_ulong != 0 { getc(stream); }
    return if result <= len { result } else { len };
}
/* ****************************************************************************/
/* BMP packbit (BITFIELDS) helpers                                           */
/* ****************************************************************************/
unsafe extern "C" fn bmp_process_mask<'a1, 'a2>(mut bmp_mask: std::os::raw::c_uint,
                                      mut sig_bit: Option<&'a1 mut std::os::raw::c_uchar>,
                                      mut shift_bit: Option<&'a2 mut std::os::raw::c_uchar>) {
    *(borrow_mut(&mut shift_bit)).unwrap() = 0 as std::os::raw::c_int as png_byte;
    *(borrow_mut(&mut sig_bit)).unwrap() = *(borrow_mut(&mut shift_bit)).unwrap();
    if bmp_mask == 0 as std::os::raw::c_int as std::os::raw::c_uint { return }
    while bmp_mask & 1 as std::os::raw::c_int as std::os::raw::c_uint ==
              0 as std::os::raw::c_int as std::os::raw::c_uint {
        bmp_mask >>= 1 as std::os::raw::c_int;
        *(borrow_mut(&mut shift_bit)).unwrap() = (*(borrow(& shift_bit)).unwrap()).wrapping_add(1)
    }
    while bmp_mask != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        if bmp_mask & 1 as std::os::raw::c_int as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint ||
               *(borrow(& sig_bit)).unwrap() as std::os::raw::c_int >= 8 as std::os::raw::c_int {
            *(borrow_mut(&mut sig_bit)).unwrap() = 0 as std::os::raw::c_int as png_byte;
            return
        }
        bmp_mask >>= 1 as std::os::raw::c_int;
        *(borrow_mut(&mut sig_bit)).unwrap() = (*(borrow(& sig_bit)).unwrap()).wrapping_add(1)
    };
}
/* ****************************************************************************/
/* BMP I/O utilities                                                         */
/* ****************************************************************************/
unsafe extern "C" fn bmp_read_rows(mut begin_row: * mut * mut std::os::raw::c_uchar,
                                   mut end_row: * mut * mut std::os::raw::c_uchar,
                                   mut row_size: std::os::raw::c_ulong,
                                   mut compression: std::os::raw::c_uint,
                                   mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong {
    let mut result: u64 = 0; /* this should not happen */
    let mut crt_row: * mut * mut u8 = 0 as *mut *mut png_byte; /* overflow */
    let mut inc: i32 =
        0; /* error: compression method not applicable. */
    let mut crtn: u64 = 0;
    let mut dcrtn: u64 = 0;
    let mut endn: u64 = 0;
    let mut len: u32 = 0;
    let mut b1: u32 = 0;
    let mut b2: u32 = 0;
    let mut ch: i32 = 0;
    let mut bmp_memset_fn:
            Option<unsafe extern "C"  fn(_: * mut u8,_: u64,_: i32,_: u64,) -> ()> =
        None;
    let mut bmp_fread_fn:
            Option<unsafe extern "C"  fn(_: * mut u8,_: u64,_: u64,_: * mut crate::src::gifread::gifread::_IO_FILE,) -> u64> = None;
    if row_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as size_t
    }
    inc =
        if begin_row <= end_row {
            1 as std::os::raw::c_int
        } else { -(1 as std::os::raw::c_int) };
    crtn = 0 as std::os::raw::c_int as size_t;
    result = 0 as std::os::raw::c_int as size_t;
    if compression == 2 as std::os::raw::c_int as std::os::raw::c_uint {
        endn = row_size.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong);
        if endn <= row_size { return 0 as std::os::raw::c_int as size_t }
        bmp_memset_fn =
            Some(bmp_memset_halfbytes);
        bmp_fread_fn =
            Some(bmp_fread_halfbytes)
    } else {
        endn = row_size;
        bmp_memset_fn =
            Some(bmp_memset_bytes);
        bmp_fread_fn =
            Some(bmp_fread_bytes)
    }
    if compression == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           compression == 3 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Read uncompressed bitmap. */
        crt_row = begin_row;
        while crt_row != end_row {
            crtn =
                bmp_fread_fn.expect("non-null function pointer")(*crt_row,
                                                                 0 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     size_t,
                                                                 endn,
                                                                 stream);
            if crtn != endn { break ; }
            result = result.wrapping_add(1);
            crt_row = crt_row.offset(inc as isize)
        }
    } else if compression == 1 as std::os::raw::c_int as std::os::raw::c_uint ||
                  compression == 2 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Read RLE-compressed bitmap. */
        if compression == 1 as std::os::raw::c_int as std::os::raw::c_uint {
            endn = row_size
        } else {
            /* BI_RLE4 */
            endn = row_size.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong);
            if endn <= row_size { return 0 as std::os::raw::c_int as size_t }
            /* overflow */
        }
        crt_row = begin_row;
        while crt_row != end_row {
            ch = getc(stream);
            b1 = ch as std::os::raw::c_uint;
            ch = getc(stream);
            b2 = ch as std::os::raw::c_uint;
            if ch == -(1 as std::os::raw::c_int) { break ; }
            if b1 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                /* escape */
                if b2 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    /* end of line */
                    bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                      crtn,
                                                                      0 as
                                                                          std::os::raw::c_int,
                                                                      endn.wrapping_sub(crtn));
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as std::os::raw::c_int as size_t;
                    result = result.wrapping_add(1);
                    if !(crt_row == end_row) { continue ; }
                    /* all rows are read */
                    ch = getc(stream); /* check for the end of bitmap */
                    if ch != -(1 as std::os::raw::c_int) && ch != 0 as std::os::raw::c_int {
                        ungetc(ch,
                               stream); /* forget about the end of bitmap */
                        break ; /* expect 1, but break the loop anyway */
                    } else { getc(stream); break ; }
                } else if b2 == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                    /* end of bitmap */
                    bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                      crtn,
                                                                      0 as
                                                                          std::os::raw::c_int,
                                                                      endn.wrapping_sub(crtn));
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as std::os::raw::c_int as size_t;
                    result =
                        if begin_row <= end_row {
                            end_row.offset_from(begin_row) as
                                std::os::raw::c_long
                        } else {
                            begin_row.offset_from(end_row) as
                                std::os::raw::c_long
                        } as size_t;
                    break ;
                    /* the rest is wiped out at the end */
                } else if b2 == 2 as std::os::raw::c_int as std::os::raw::c_uint {
                    /* delta */
                    ch = getc(stream); /* horiz. offset */
                    b1 = ch as std::os::raw::c_uint; /* vert. offset */
                    ch = getc(stream);
                    b2 = ch as std::os::raw::c_uint;
                    if ch == -(1 as std::os::raw::c_int) { break ; }
                    dcrtn =
                        if (b1 as std::os::raw::c_ulong) < endn.wrapping_sub(crtn) {
                            crtn.wrapping_add(b1 as std::os::raw::c_ulong)
                        } else { endn };
                    while b2 > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                          crtn,
                                                                          0 as
                                                                              std::os::raw::c_int,
                                                                          endn.wrapping_sub(crtn));
                        crt_row = crt_row.offset(inc as isize);
                        crtn = 0 as std::os::raw::c_int as size_t;
                        result = result.wrapping_add(1);
                        if crt_row == end_row { break ; }
                        b2 = b2.wrapping_sub(1)
                    }
                    if crt_row != end_row {
                        bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                          crtn,
                                                                          0 as
                                                                              std::os::raw::c_int,
                                                                          dcrtn.wrapping_sub(crtn));
                    }
                } else {
                    /* b2 >= 3 bytes in absolute mode */
                    len =
                        if b2 as std::os::raw::c_ulong <= endn.wrapping_sub(crtn) {
                            b2
                        } else { endn.wrapping_sub(crtn) as std::os::raw::c_uint };
                    if bmp_fread_fn.expect("non-null function pointer")(*crt_row,
                                                                        crtn,
                                                                        len as
                                                                            size_t,
                                                                        stream)
                           != len as std::os::raw::c_ulong {
                        break ;
                    }
                    crtn =
                        (crtn as
                             std::os::raw::c_ulong).wrapping_add(len as std::os::raw::c_ulong)
                            as size_t as size_t
                }
            } else {
                /* b1 > 0 bytes in run-length encoded mode */
                len =
                    if b1 as std::os::raw::c_ulong <= endn.wrapping_sub(crtn) {
                        b1
                    } else { endn.wrapping_sub(crtn) as std::os::raw::c_uint };
                bmp_memset_fn.expect("non-null function pointer")(*crt_row,
                                                                  crtn,
                                                                  b2 as
                                                                      std::os::raw::c_int,
                                                                  len as
                                                                      size_t);
                crtn =
                    (crtn as std::os::raw::c_ulong).wrapping_add(len as std::os::raw::c_ulong)
                        as size_t as size_t
            }
        }
    } else { return 0 as std::os::raw::c_int as size_t }
    /* Wipe out the portion left unread. */
    while crt_row != end_row {
        bmp_memset_fn.expect("non-null function pointer")(*crt_row, crtn,
                                                          0 as std::os::raw::c_int,
                                                          endn.wrapping_sub(crtn));
        crtn = 0 as std::os::raw::c_int as size_t;
        crt_row = crt_row.offset(inc as isize)
    }
    return result;
}
/* ****************************************************************************/
/* BMP-to-PNG sample conversion                                              */
/* ****************************************************************************/
unsafe extern "C" fn bmp_to_png_rows(mut row_pointers: * mut * mut std::os::raw::c_uchar,
                                     mut width: std::os::raw::c_uint,
                                     mut height: std::os::raw::c_uint,
                                     mut pixdepth: std::os::raw::c_uint,
                                     mut rgba_sig: * mut std::os::raw::c_uchar,
                                     mut rgba_shift: * mut std::os::raw::c_uchar) {
    let mut src_ptr: * mut u8 = 0 as *mut png_byte;
    let mut dest_ptr: * mut u8 = 0 as *mut png_byte;
    let mut rgba_mask: [u32; 4] = [0; 4];
    let mut num_samples: u32 = 0;
    let mut sample: u32 = 0;
    let mut mask: u32 = 0;
    let mut wpix: u32 = 0;
    let mut dwpix: u32 = 0;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut i: u32 = 0;
    if pixdepth == 24 as std::os::raw::c_int as std::os::raw::c_uint {
        /* BGR -> RGB */
        y = 0 as std::os::raw::c_int as png_uint_32;
        while y < height {
            src_ptr = *row_pointers.offset(y as isize);
            x = 0 as std::os::raw::c_int as png_uint_32;
            while x < width {
                let mut tmp: u8 =
                    *src_ptr.offset(0 as std::os::raw::c_int as isize);
                *src_ptr.offset(0 as std::os::raw::c_int as isize) =
                    *src_ptr.offset(2 as std::os::raw::c_int as isize);
                *src_ptr.offset(2 as std::os::raw::c_int as isize) = tmp;
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(3 as std::os::raw::c_int as isize)
            }
            y = y.wrapping_add(1)
        }
        return
    }
    num_samples =
        if *rgba_sig.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               0 as std::os::raw::c_int {
            4 as std::os::raw::c_int
        } else { 3 as std::os::raw::c_int } as std::os::raw::c_uint;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < num_samples {
        rgba_mask[i as usize] =
            ((1 as std::os::raw::c_uint) <<
                 *rgba_sig.offset(i as isize) as
                     std::os::raw::c_int).wrapping_sub(1 as std::os::raw::c_int as
                                                   std::os::raw::c_uint);
        i = i.wrapping_add(1)
    }
    if pixdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
        y = 0 as std::os::raw::c_int as png_uint_32;
        while y < height {
            src_ptr =
                (*row_pointers.offset(y as
                                          isize)).offset(width.wrapping_sub(1
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_uint).wrapping_mul(2
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               std::os::raw::c_uint)
                                                             as isize);
            dest_ptr =
                (*row_pointers.offset(y as
                                          isize)).offset(width.wrapping_sub(1
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_uint).wrapping_mul(num_samples)
                                                             as isize);
            x = 0 as std::os::raw::c_int as png_uint_32;
            while x < width {
                /* Inline bmp_get_word() for performance reasons. */
                wpix =
                    (*src_ptr.offset(0 as std::os::raw::c_int as isize) as
                         std::os::raw::c_uint).wrapping_add((*src_ptr.offset(1 as
                                                                         std::os::raw::c_int
                                                                         as
                                                                         isize)
                                                         as std::os::raw::c_uint) <<
                                                        8 as std::os::raw::c_int);
                i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample =
                        wpix >> *rgba_shift.offset(i as isize) as std::os::raw::c_int
                            & mask;
                    *dest_ptr.offset(i as isize) =
                        sample.wrapping_mul(255 as std::os::raw::c_int as
                                                std::os::raw::c_uint).wrapping_add(mask.wrapping_div(2
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_uint)).wrapping_div(mask)
                            as png_byte;
                    i = i.wrapping_add(1)
                }
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(-(2 as std::os::raw::c_int as isize));
                dest_ptr = dest_ptr.offset(-(num_samples as isize))
            }
            y = y.wrapping_add(1)
        }
    } else if pixdepth == 32 as std::os::raw::c_int as std::os::raw::c_uint {
        y = 0 as std::os::raw::c_int as png_uint_32;
        while y < height {
            dest_ptr = *row_pointers.offset(y as isize);
            src_ptr = dest_ptr;
            x = 0 as std::os::raw::c_int as png_uint_32;
            while x < width {
                /* Inline bmp_get_dword() for performance reasons. */
                dwpix =
                    (*src_ptr.offset(0 as std::os::raw::c_int as isize) as
                         png_uint_32).wrapping_add((*src_ptr.offset(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        isize)
                                                        as png_uint_32) <<
                                                       8 as
                                                           std::os::raw::c_int).wrapping_add((*src_ptr.offset(2
                                                                                                          as
                                                                                                          std::os::raw::c_int
                                                                                                          as
                                                                                                          isize)
                                                                                          as
                                                                                          png_uint_32)
                                                                                         <<
                                                                                         16
                                                                                             as
                                                                                             std::os::raw::c_int).wrapping_add((*src_ptr.offset(3
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_int
                                                                                                                                            as
                                                                                                                                            isize)
                                                                                                                            as
                                                                                                                            png_uint_32)
                                                                                                                           <<
                                                                                                                           24
                                                                                                                               as
                                                                                                                               std::os::raw::c_int);
                i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample =
                        dwpix >> *rgba_shift.offset(i as isize) as std::os::raw::c_int
                            & mask;
                    *dest_ptr.offset(i as isize) =
                        sample.wrapping_mul(255 as std::os::raw::c_int as
                                                std::os::raw::c_uint).wrapping_add(mask.wrapping_div(2
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_uint)).wrapping_div(mask)
                            as png_byte;
                    i = i.wrapping_add(1)
                }
                x = x.wrapping_add(1);
                src_ptr = src_ptr.offset(4 as std::os::raw::c_int as isize);
                dest_ptr = dest_ptr.offset(num_samples as isize)
            }
            y = y.wrapping_add(1)
        }
    };
    /* else do nothing */
}
/* ****************************************************************************/
/* BMP read support for pngxtern                                             */
/* ****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_bmp<'a1, 'a2>(mut sig: * mut std::os::raw::c_uchar,
                                         mut sig_size: std::os::raw::c_ulong,
                                         mut fmt_name_ptr: Option<&'a1 mut * const std::os::raw::c_char>,
                                         mut fmt_long_name_ptr:
                                             Option<&'a2 mut * const std::os::raw::c_char>)
 -> std::os::raw::c_int {
    static mut bmp_fmt_name: [std::os::raw::c_char; 4] =
        [0,0,0,0,]; unsafe fn laertes_init_bmp_fmt_name() {
bmp_fmt_name = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 4], &'_ [i8; 4]>(b"BMP\x00")
        };}//;
    static mut os2bmp_fmt_long_name: [std::os::raw::c_char; 12] =
        [0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_os2bmp_fmt_long_name() {
os2bmp_fmt_long_name = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 12], &'_ [i8; 12]>(b"OS/2 Bitmap\x00")
        };}//;
    static mut winbmp_fmt_long_name: [std::os::raw::c_char; 15] =
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_winbmp_fmt_long_name() {
winbmp_fmt_long_name = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 15], &'_ [i8; 15]>(b"Windows Bitmap\x00")
        };}//;
    let mut bihsize: u32 = 0;
    /* Require at least the bitmap file header and the subsequent 4 bytes. */
    if sig_size < (14 as std::os::raw::c_int + 4 as std::os::raw::c_int) as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    } /* insufficient data */
    if bmp_get_word(sig) != 0x4d42 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    } /* not BMP */
    /* Avoid using bfhsize because it is not reliable. */
    bihsize =
        bmp_get_dword(sig.offset(14 as std::os::raw::c_int as
                                     isize)); /* garbage in bihsize, this cannot be BMP */
    if bihsize > 0x7fffffff as std::os::raw::c_long as png_uint_32 ||
           bihsize != 12 as std::os::raw::c_int as std::os::raw::c_uint &&
               bihsize < 40 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    /* Store the format name. */
    if !borrow(& fmt_name_ptr).is_none() { *(borrow_mut(&mut fmt_name_ptr)).unwrap() = bmp_fmt_name.as_ptr() }
    if !borrow(& fmt_long_name_ptr).is_none() {
        if bihsize == 12 as std::os::raw::c_int as std::os::raw::c_uint {
            *(borrow_mut(&mut fmt_long_name_ptr)).unwrap() = os2bmp_fmt_long_name.as_ptr()
        } else { *(borrow_mut(&mut fmt_long_name_ptr)).unwrap() = winbmp_fmt_long_name.as_ptr() }
    }
    return 1 as std::os::raw::c_int;
    /* BMP */
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_bmp(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                       mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int {
    let mut bfh: [u8; 138] = [0; 138];
    let bih: * mut u8 = bfh.as_mut_ptr().offset(14 as std::os::raw::c_int as isize);
    let mut rgbq: [u8; 4] = [0; 4];
    let mut offbits: u32 = 0;
    let mut bihsize: u32 = 0;
    let mut skip: u32 = 0;
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut rowsize: u32 = 0;
    let mut topdown: i32 = 0;
    let mut pixdepth: u32 = 0;
    let mut compression: u32 = 0;
    let mut palsize: u32 = 0;
    let mut palnum: u32 = 0;
    let mut rgba_mask: [u32; 4] = [0; 4];
    let mut rgba_sig: [u8; 4] = [0; 4];
    let mut rgba_shift: [u8; 4] = [0; 4];
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut palette: [crate::src::libpng::png::png_color_struct; 256] =
        [png_color{red: 0, green: 0, blue: 0,}; 256];
    let mut sig_bit: crate::src::libpng::png::png_color_8_struct =
        png_color_8{red: 0, green: 0, blue: 0, gray: 0, alpha: 0,};
    let mut row_pointers: * mut * mut u8 = 0 as *mut *mut png_byte;
    let mut begin_row: * mut * mut u8 = 0 as *mut *mut png_byte;
    let mut end_row: * mut * mut u8 = 0 as *mut *mut png_byte;
    let mut i: u32 = 0;
    let mut y: u64 = 0;
    /* Find the BMP header. */
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    loop 
         /* skip macbinary header */
         {
        if fread(bfh.as_mut_ptr() as *mut std::os::raw::c_void,
                 (14 as std::os::raw::c_int + 4 as std::os::raw::c_int) as std::os::raw::c_ulong,
                 1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong {
            i = i.wrapping_add(1)
        } else if bmp_get_word(bfh.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                           isize)) ==
                      0x4d42 as std::os::raw::c_int as std::os::raw::c_uint {
            break ;
        }
        if fread(bfh.as_mut_ptr() as *mut std::os::raw::c_void,
                 (128 as std::os::raw::c_int - 14 as std::os::raw::c_int - 4 as std::os::raw::c_int)
                     as std::os::raw::c_ulong, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
                 stream) != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            i = i.wrapping_add(1)
        }
        if i > 0 as std::os::raw::c_int as std::os::raw::c_uint { return 0 as std::os::raw::c_int }
        i = i.wrapping_add(1)
        /* not a BMP file */
    }
    /* Read the BMP header. */
    offbits =
        bmp_get_dword(bfh.as_mut_ptr().offset(10 as std::os::raw::c_int as
                                                  isize)); /* not a BMP file, just a file with a matching signature */
    bihsize =
        bmp_get_dword(bfh.as_mut_ptr().offset(14 as std::os::raw::c_int as
                                                  isize)); /* new skip */
    if offbits > 0x7fffffff as std::os::raw::c_long as png_uint_32 ||
           bihsize > 0x7fffffff as std::os::raw::c_long as png_uint_32 ||
           offbits < bihsize.wrapping_add(14 as std::os::raw::c_int as std::os::raw::c_uint)
           ||
           bihsize != 12 as std::os::raw::c_int as std::os::raw::c_uint &&
               bihsize < 40 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if bihsize > 124 as std::os::raw::c_int as std::os::raw::c_uint {
        skip = bihsize.wrapping_sub(124 as std::os::raw::c_int as std::os::raw::c_uint);
        bihsize = 124 as std::os::raw::c_int as png_uint_32
    } else { skip = 0 as std::os::raw::c_int as png_uint_32 }
    if fread(bih.offset(4 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
             bihsize.wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_uint) as
                 std::os::raw::c_ulong, 1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    if skip > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        if fseek(stream, skip as std::os::raw::c_long, 1 as std::os::raw::c_int) !=
               0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int
        }
    }
    skip =
        offbits.wrapping_sub(bihsize).wrapping_sub(14 as std::os::raw::c_int as
                                                       std::os::raw::c_uint);
    topdown = 0 as std::os::raw::c_int;
    if bihsize < 40 as std::os::raw::c_int as std::os::raw::c_uint {
        /* OS/2 BMP */
        width = bmp_get_word(bih.offset(4 as std::os::raw::c_int as isize));
        height = bmp_get_word(bih.offset(6 as std::os::raw::c_int as isize));
        pixdepth = bmp_get_word(bih.offset(10 as std::os::raw::c_int as isize));
        compression = 0 as std::os::raw::c_int as png_uint_32;
        palsize = 3 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        /* Windows BMP */
        width = bmp_get_dword(bih.offset(4 as std::os::raw::c_int as isize));
        height = bmp_get_dword(bih.offset(8 as std::os::raw::c_int as isize));
        pixdepth = bmp_get_word(bih.offset(14 as std::os::raw::c_int as isize));
        compression = bmp_get_dword(bih.offset(16 as std::os::raw::c_int as isize));
        palsize = 4 as std::os::raw::c_int as std::os::raw::c_uint;
        if height > 0x7fffffff as std::os::raw::c_long as png_uint_32 {
            /* top-down BMP */
            height =
                (-(1 as std::os::raw::c_int) as
                     png_uint_32).wrapping_sub(height).wrapping_add(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_uint);
            topdown = 1 as std::os::raw::c_int
        }
        if bihsize == 40 as std::os::raw::c_int as std::os::raw::c_uint &&
               compression == 3 as std::os::raw::c_int as std::os::raw::c_uint {
            /* Read the RGB[A] mask. */
            i =
                if skip <= 16 as std::os::raw::c_int as std::os::raw::c_uint {
                    skip
                } else { 16 as std::os::raw::c_int as std::os::raw::c_uint };
            if fread(bih.offset(40 as std::os::raw::c_int as isize) as
                         *mut std::os::raw::c_void, i as std::os::raw::c_ulong,
                     1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
                   1 as std::os::raw::c_int as std::os::raw::c_ulong {
                return 0 as std::os::raw::c_int
            }
            bihsize =
                (bihsize as std::os::raw::c_uint).wrapping_add(i) as png_uint_32 as
                    png_uint_32;
            skip =
                (skip as std::os::raw::c_uint).wrapping_sub(i) as png_uint_32 as
                    png_uint_32
        }
    }
    memset(rgba_mask.as_mut_ptr() as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<[png_uint_32; 4]>() as std::os::raw::c_ulong);
    if pixdepth > 8 as std::os::raw::c_int as std::os::raw::c_uint {
        if compression == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if pixdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
                compression = 3 as std::os::raw::c_int as png_uint_32;
                rgba_mask[0 as std::os::raw::c_int as usize] =
                    0x7c00 as std::os::raw::c_int as png_uint_32;
                rgba_mask[1 as std::os::raw::c_int as usize] =
                    0x3e0 as std::os::raw::c_int as png_uint_32;
                rgba_mask[2 as std::os::raw::c_int as usize] =
                    0x1f as std::os::raw::c_int as png_uint_32
            } else {
                /* pixdepth == 24 || pixdepth == 32 */
                rgba_mask[0 as std::os::raw::c_int as usize] =
                    0xff0000 as std::os::raw::c_long as png_uint_32;
                rgba_mask[1 as std::os::raw::c_int as usize] =
                    0xff00 as std::os::raw::c_long as png_uint_32;
                rgba_mask[2 as std::os::raw::c_int as usize] =
                    0xff as std::os::raw::c_long as png_uint_32
            }
        } else if compression == 3 as std::os::raw::c_int as std::os::raw::c_uint {
            if bihsize >=
                   (40 as std::os::raw::c_int + 12 as std::os::raw::c_int) as std::os::raw::c_uint {
                rgba_mask[0 as std::os::raw::c_int as usize] =
                    bmp_get_dword(bih.offset(40 as std::os::raw::c_int as isize));
                rgba_mask[1 as std::os::raw::c_int as usize] =
                    bmp_get_dword(bih.offset(44 as std::os::raw::c_int as isize));
                rgba_mask[2 as std::os::raw::c_int as usize] =
                    bmp_get_dword(bih.offset(48 as std::os::raw::c_int as isize))
            } else {
                png_error(png_ptr as *const png_struct,
                          b"Missing color mask in BMP file\x00" as *const u8
                              as *const std::os::raw::c_char);
            }
        }
        if bihsize >= (40 as std::os::raw::c_int + 16 as std::os::raw::c_int) as std::os::raw::c_uint
           {
            rgba_mask[3 as std::os::raw::c_int as usize] =
                bmp_get_dword(bih.offset(52 as std::os::raw::c_int as isize))
        }
    }
    match compression {
        0 => {
            /* Allow pixel depth values 1, 2, 4, 8, 16, 24, 32.
       * (Although the BMP spec does not mention pixel depth = 2,
       * it is supported for robustness reasons, at no extra cost.)
       */
            if pixdepth > 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   (32 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_rem(pixdepth)
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   pixdepth != 24 as std::os::raw::c_int as std::os::raw::c_uint {
                pixdepth = 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
        }
        1 => {
            if pixdepth != 8 as std::os::raw::c_int as std::os::raw::c_uint {
                pixdepth = 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
        }
        2 => {
            if pixdepth != 4 as std::os::raw::c_int as std::os::raw::c_uint {
                pixdepth = 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
        }
        3 => {
            if pixdepth != 16 as std::os::raw::c_int as std::os::raw::c_uint &&
                   pixdepth != 32 as std::os::raw::c_int as std::os::raw::c_uint {
                pixdepth = 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
        }
        4 => {
            png_error(png_ptr as *const png_struct,
                      b"JPEG-compressed BMP files are not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
        }
        5 => {
            if ungetc(getc(stream), stream) == 0 as std::os::raw::c_int {
                /* IHDR is likely to follow */
                png_set_sig_bytes(png_ptr, 8 as std::os::raw::c_int);
            }
            png_set_read_fn(png_ptr, stream as png_voidp, None);
            png_read_png(png_ptr, info_ptr, 0 as std::os::raw::c_int,
                         (0 as * mut core::ffi::c_void));
            /* TODO: Check for mismatches between the BMP and PNG info. */
            return 1 as std::os::raw::c_int
        }
        _ => {
            png_error(png_ptr as *const png_struct,
                      b"Unsupported compression method in BMP file\x00" as
                          *const u8 as *const std::os::raw::c_char);
        }
    }
    /* Check the BMP image parameters. */
    if width == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           width > 0x7fffffff as std::os::raw::c_long as png_uint_32 ||
           height == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Invalid image dimensions in BMP file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    if pixdepth == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Invalid pixel depth in BMP file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* Compute the PNG image parameters. */
    if pixdepth <= 8 as std::os::raw::c_int as std::os::raw::c_uint {
        palnum = skip.wrapping_div(palsize);
        if palnum > 256 as std::os::raw::c_int as std::os::raw::c_uint {
            palnum = 256 as std::os::raw::c_int as std::os::raw::c_uint
        }
        skip =
            (skip as std::os::raw::c_uint).wrapping_sub(palsize.wrapping_mul(palnum))
                as png_uint_32 as png_uint_32;
        rowsize =
            width.wrapping_add((32 as std::os::raw::c_int as
                                    std::os::raw::c_uint).wrapping_div(pixdepth)).wrapping_sub(1
                                                                                           as
                                                                                           std::os::raw::c_int
                                                                                           as
                                                                                           std::os::raw::c_uint).wrapping_div((32
                                                                                                                           as
                                                                                                                           std::os::raw::c_int
                                                                                                                           as
                                                                                                                           std::os::raw::c_uint).wrapping_div(pixdepth)).wrapping_mul(4
                                                                                                                                                                                  as
                                                                                                                                                                                  std::os::raw::c_int
                                                                                                                                                                                  as
                                                                                                                                                                                  std::os::raw::c_uint);
        /* rowsize becomes 0 on overflow. */
        bit_depth = pixdepth as std::os::raw::c_int;
        color_type =
            if palnum > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                (2 as std::os::raw::c_int) | 1 as std::os::raw::c_int
            } else { 0 as std::os::raw::c_int }
    } else {
        palnum = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        bit_depth = 8 as std::os::raw::c_int;
        match pixdepth {
            16 => {
                rowsize =
                    width.wrapping_mul(2 as std::os::raw::c_int as
                                           std::os::raw::c_uint).wrapping_add(3 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          std::os::raw::c_uint)
                        & !(3 as std::os::raw::c_int) as std::os::raw::c_uint
            }
            24 => {
                rowsize =
                    width.wrapping_mul(3 as std::os::raw::c_int as
                                           std::os::raw::c_uint).wrapping_add(3 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          std::os::raw::c_uint)
                        & !(3 as std::os::raw::c_int) as std::os::raw::c_uint
            }
            32 => {
                rowsize = width.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
            }
            _ => {
                /* never get here */
                bit_depth = 0 as std::os::raw::c_int; /* overflow */
                rowsize = 0 as std::os::raw::c_int as png_uint_32
            }
        }
        if rowsize.wrapping_div(width) <
               pixdepth.wrapping_div(8 as std::os::raw::c_int as std::os::raw::c_uint) {
            rowsize = 0 as std::os::raw::c_int as png_uint_32
        }
        color_type =
            if rgba_mask[3 as std::os::raw::c_int as usize] !=
                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                (2 as std::os::raw::c_int) | 4 as std::os::raw::c_int
            } else { 2 as std::os::raw::c_int }
    }
    if rowsize == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle exceedingly large BMP dimensions\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    /* Set the PNG image type. */
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, color_type, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                 0 as std::os::raw::c_int);
    if pixdepth > 8 as std::os::raw::c_int as std::os::raw::c_uint {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < 4 as std::os::raw::c_int as std::os::raw::c_uint {
            bmp_process_mask(rgba_mask[i as usize],
                             Some(&mut *rgba_sig.as_mut_ptr().offset(i as isize)),
                             Some(&mut *rgba_shift.as_mut_ptr().offset(i as
                                                                      isize)));
            i = i.wrapping_add(1)
        }
        if rgba_sig[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
               0 as std::os::raw::c_int ||
               rgba_sig[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int ||
               rgba_sig[2 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            png_error(png_ptr as *const png_struct,
                      b"Invalid color mask in BMP file\x00" as *const u8 as
                          *const std::os::raw::c_char);
        }
        if rgba_sig[0 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
               8 as std::os::raw::c_int ||
               rgba_sig[1 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
                   8 as std::os::raw::c_int ||
               rgba_sig[2 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
                   8 as std::os::raw::c_int ||
               rgba_sig[3 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
                   0 as std::os::raw::c_int &&
                   rgba_sig[3 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
                       8 as std::os::raw::c_int {
            sig_bit.red = rgba_sig[0 as std::os::raw::c_int as usize];
            sig_bit.green = rgba_sig[1 as std::os::raw::c_int as usize];
            sig_bit.blue = rgba_sig[2 as std::os::raw::c_int as usize];
            sig_bit.alpha = rgba_sig[3 as std::os::raw::c_int as usize];
            png_set_sBIT(png_ptr as *const png_struct, info_ptr,
                         Some(&mut sig_bit));
        }
    }
    /* Read the color palette (if applicable). */
    if palnum > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < palnum {
            if fread(rgbq.as_mut_ptr() as *mut std::os::raw::c_void,
                     palsize as std::os::raw::c_ulong,
                     1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
                   1 as std::os::raw::c_int as std::os::raw::c_ulong {
                break ;
            }
            palette[i as usize].red = rgbq[2 as std::os::raw::c_int as usize];
            palette[i as usize].green = rgbq[1 as std::os::raw::c_int as usize];
            palette[i as usize].blue = rgbq[0 as std::os::raw::c_int as usize];
            i = i.wrapping_add(1)
        }
        png_set_PLTE(png_ptr, info_ptr,
                     palette.as_mut_ptr() as png_const_colorp,
                     i as std::os::raw::c_int);
        if i != palnum {
            png_error(png_ptr as *const png_struct,
                      b"Error reading color palette in BMP file\x00" as
                          *const u8 as *const std::os::raw::c_char);
        }
    }
    /* Allocate memory and read the image data. */
    row_pointers =
        pngx_malloc_rows_extended(png_ptr, info_ptr,
                                  rowsize as pngx_alloc_size_t,
                                  -(1 as std::os::raw::c_int));
    if topdown != 0 {
        begin_row = row_pointers;
        end_row = row_pointers.offset(height as isize)
    } else {
        begin_row =
            row_pointers.offset(height as
                                    isize).offset(-(1 as std::os::raw::c_int as
                                                        isize));
        end_row = row_pointers.offset(-(1 as std::os::raw::c_int as isize))
    }
    if skip > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        fseek(stream, skip as std::os::raw::c_long, 1 as std::os::raw::c_int);
    }
    y =
        bmp_read_rows(begin_row, end_row, rowsize as size_t, compression,
                      stream);
    /* Postprocess the image data, even if it has not been read entirely. */
    if pixdepth > 8 as std::os::raw::c_int as std::os::raw::c_uint {
        bmp_to_png_rows(row_pointers, width, height, pixdepth,
                        rgba_sig.as_mut_ptr(), rgba_shift.as_mut_ptr());
    }
    /* Check the result. */
    if y != height as size_t {
        png_error(png_ptr as *const png_struct,
                  b"Error reading BMP file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    return 1 as std::os::raw::c_int;
    /* one image has been successfully read */
}
use crate::laertes_rt::*;
