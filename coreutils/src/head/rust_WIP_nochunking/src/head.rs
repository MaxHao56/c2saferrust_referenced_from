#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types, label_break_value)]











use libc::stat as libc_stat;

use std::ffi::CString;

use std::ffi::CStr;
use std::io::Write;

use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn offtostr(_: off_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn close_stdout();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn proper_name_lite(
        name_ascii: *const libc::c_char,
        name_utf8: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn xdectoumax(
        n_str: *const libc::c_char,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type header_mode = libc::c_uint;
pub const never: header_mode = 2;
pub const always: header_mode = 1;
pub const multiple_files: header_mode = 0;
pub type Copy_fd_status = libc::c_uint;
pub const COPY_FD_UNEXPECTED_EOF: Copy_fd_status = 2;
pub const COPY_FD_READ_ERROR: Copy_fd_status = 1;
pub const COPY_FD_OK: Copy_fd_status = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PRESUME_INPUT_PIPE_OPTION: C2RustUnnamed_0 = 256;
pub type LBUFFER = linebuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub buffer: [libc::c_char; 8193],
    pub nbytes: size_t,
    pub nlines: size_t,
    pub next: *mut linebuffer,
}
#[inline]
fn usable_st_size(sb: &stat) -> bool {
    (sb.st_mode & 0o170000 as libc::c_uint == 0o100000 as libc::c_uint)
        || (sb.st_mode & 0o170000 as libc::c_uint == 0o120000 as libc::c_uint)
        || (sb.st_mode.wrapping_sub(sb.st_mode) != 0 || 0 != 0)
}

#[inline]
fn emit_ancillary_info(program: &CStr) {
    let infomap_0: [(&CStr, &CStr); 7] = [
        (CStr::from_bytes_with_nul(b"[\0").unwrap(), CStr::from_bytes_with_nul(b"test invocation\0").unwrap()),
        (CStr::from_bytes_with_nul(b"coreutils\0").unwrap(), CStr::from_bytes_with_nul(b"Multi-call invocation\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha224sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha256sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha384sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha512sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"\0").unwrap(), CStr::from_bytes_with_nul(b"\0").unwrap()),
    ];
    
    let mut node = program.as_ptr();
    let mut map_prog = &infomap_0[0];

    while !map_prog.0.as_ptr().is_null() && !program.to_bytes().eq(map_prog.0.to_bytes()) {
        map_prog = &infomap_0[map_prog as *const _ as usize / std::mem::size_of::<(&CStr, &CStr)>() + 1];
    }

    if !map_prog.1.as_ptr().is_null() {
        node = map_prog.1.as_ptr();
    }

    println!(
        "{} online help: <{}>",
        "GNU coreutils",
        "https://www.gnu.org/software/coreutils/"
    );

    let lc_messages: *const libc::c_char;
    unsafe {
        lc_messages = setlocale(5, std::ptr::null());
    }
    
    if !lc_messages.is_null() {
        let lc_messages_str = unsafe { CStr::from_ptr(lc_messages).to_str().unwrap() };
        if !lc_messages_str.starts_with("en_") {
            writeln!(
                std::io::stdout(),
                "{}",
                "Report any translation bugs to <https://translationproject.org/team/>"
            ).unwrap();
        }
    }

    let url_program = if program.to_bytes() == b"[\0" {
        CStr::from_bytes_with_nul(b"test\0").unwrap()
    } else {
        program
    };

    println!(
        "Full documentation <{}{}>",
        "https://www.gnu.org/software/coreutils/",
        url_program.to_str().unwrap()
    );

    let node_str = unsafe { CStr::from_ptr(node).to_str().unwrap() };
    println!(
        "or available locally via: info '(coreutils) {}{}'",
        node_str,
        if node == program.as_ptr() {
            " invocation"
        } else {
            ""
        }
    );
}

#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
}
#[inline]
fn emit_stdin_note() {
    let message = "\nWith no FILE, or when FILE is -, read standard input.\n";
    let translated_message = unsafe { gettext(message.as_ptr() as *const libc::c_char) };
    let _ = std::io::stdout().write_all(unsafe { std::ffi::CStr::from_ptr(translated_message).to_bytes() });
}

#[inline]
fn emit_mandatory_arg_note() {
    let message = "\nMandatory arguments to long options are mandatory for short options too.\n";
    let stdout_handle = std::io::stdout();
    let mut handle = stdout_handle.lock();
    handle.write_all(message.as_bytes()).expect("Failed to write to stdout");
}

#[inline]
fn __gl_setmode(fd: i32, mode: i32) -> i32 {
    return 0;
}

#[inline]
fn set_binary_mode(fd: i32, mode: i32) -> i32 {
    __gl_setmode(fd, mode)
}

#[inline]
fn xset_binary_mode_error() {
    // Implement the functionality here in a safe manner.
    // For example, if this function is meant to set binary mode for I/O,
    // you might want to use standard Rust I/O features instead.
    // This is a placeholder for the actual implementation.
}

#[inline]
fn xset_binary_mode(fd: i32, mode: i32) {
    unsafe {
        if set_binary_mode(fd, mode) < 0 {
            xset_binary_mode_error();
        }
    }
}

static mut presume_input_pipe: bool = false;
static mut print_headers: bool = false;
static mut line_end: libc::c_char = 0;
static mut have_read_stdin: bool = false;
static mut long_options: [option; 10] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"-presume-input-pipe\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESUME_INPUT_PIPE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero-terminated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_HELP_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_VERSION_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub fn usage(status: i32) {
    if status != 0 {
        eprintln!(
            "{}",
            unsafe { CStr::from_ptr(gettext(format!("Try '{} --help' for more information.\n", unsafe { CStr::from_ptr(program_name).to_string_lossy() }).as_ptr() as *const i8)).to_string_lossy() }
        );
    } else {
        println!(
            "{}",
            unsafe { CStr::from_ptr(gettext(format!("Usage: {} [OPTION]... [FILE]...\n", unsafe { CStr::from_ptr(program_name).to_string_lossy() }).as_ptr() as *const i8)).to_string_lossy() }
        );
        println!(
            "{}",
            unsafe { CStr::from_ptr(gettext(format!("Print the first {} lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n", 10).as_ptr() as *const i8)).to_string_lossy() }
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        println!(
            "{}",
            unsafe { CStr::from_ptr(gettext(format!("  -c, --bytes=[-]NUM       print the first NUM bytes of each file;\n                             with the leading '-', print all but the last\n                             NUM bytes of each file\n  -n, --lines=[-]NUM       print the first NUM lines instead of the first {};\n                             with the leading '-', print all but the last\n                             NUM lines of each file\n", 10).as_ptr() as *const i8)).to_string_lossy() }
        );
        writeln!(
            std::io::stdout(),
            "{}",
            unsafe { CStr::from_ptr(gettext("  -q, --quiet, --silent    never print headers giving file names\n  -v, --verbose            always print headers giving file names\n".as_ptr() as *const i8)).to_string_lossy() }
        ).unwrap();
        writeln!(
            std::io::stdout(),
            "{}",
            unsafe { CStr::from_ptr(gettext("  -z, --zero-terminated    line delimiter is NUL, not newline\n".as_ptr() as *const i8)).to_string_lossy() }
        ).unwrap();
        writeln!(
            std::io::stdout(),
            "{}",
            unsafe { CStr::from_ptr(gettext("      --help        display this help and exit\n".as_ptr() as *const i8)).to_string_lossy() }
        ).unwrap();
        writeln!(
            std::io::stdout(),
            "{}",
            unsafe { CStr::from_ptr(gettext("      --version     output version information and exit\n".as_ptr() as *const i8)).to_string_lossy() }
        ).unwrap();
        writeln!(
            std::io::stdout(),
            "{}",
            unsafe { CStr::from_ptr(gettext("\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y, R, Q.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n".as_ptr() as *const i8)).to_string_lossy() }
        ).unwrap();
        emit_ancillary_info(CStr::from_bytes_with_nul(b"head\0").unwrap());
    }
    std::process::exit(status);
}

fn diagnose_copy_fd_failure(err: Copy_fd_status, filename: *const libc::c_char) {
    let filename_str = unsafe { std::ffi::CStr::from_ptr(filename).to_string_lossy() };
    match err as u32 {
        1 => {
            let errstatus: i32 = 0;
            unsafe {
                error(
                    errstatus,
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                    gettext(b"error reading %s\0".as_ptr() as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                error(
                    errstatus,
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                    gettext(b"error reading %s\0".as_ptr() as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
            }
        }
        2 => {
            let errstatus: i32 = 0;
            unsafe {
                error(
                    errstatus,
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                    gettext(b"%s: file has shrunk too much\0".as_ptr() as *const libc::c_char),
                    quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
                );
                error(
                    errstatus,
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                    gettext(b"%s: file has shrunk too much\0".as_ptr() as *const libc::c_char),
                    quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
                );
            }
        }
        _ => {
            panic!("Unexpected Copy_fd_status value");
        }
    }
}

fn write_header(filename: &str) {
    static mut FIRST_FILE: bool = true;
    let prefix = if unsafe { FIRST_FILE } { "" } else { "\n" };
    println!("{}==> {} <==", prefix, filename);
    unsafe { FIRST_FILE = false; }
}

unsafe extern "C" fn xwrite_stdout(
    mut buffer: *const libc::c_char,
    mut n_bytes: size_t,
) {
    if n_bytes > 0 as libc::c_int as libc::c_ulong
        && fwrite_unlocked(
            buffer as *const libc::c_void,
            1 as libc::c_int as size_t,
            n_bytes,
            stdout,
        ) < n_bytes
    {
        clearerr_unlocked(stdout);
        fpurge(stdout);
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    b"standard output\0" as *const u8 as *const libc::c_char,
                ),
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"standard output\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"standard output\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
}
fn copy_fd(src_fd: libc::c_int, mut n_bytes: u64) -> Copy_fd_status {
    let mut buf = [0; 8192];
    let buf_size = buf.len() as u64;
    
    while n_bytes > 0 {
        let n_to_read = buf_size.min(n_bytes);
        let n_read = unsafe {
            safe_read(src_fd, buf.as_mut_ptr() as *mut libc::c_void, n_to_read)
        };
        
        if n_read == u64::MAX {
            return COPY_FD_READ_ERROR;
        }
        
        n_bytes -= n_read;
        
        if n_read == 0 && n_bytes > 0 {
            return COPY_FD_UNEXPECTED_EOF;
        }
        
        unsafe {
            xwrite_stdout(buf.as_ptr() as *const libc::c_char, n_read);
        }
    }
    
    COPY_FD_OK
}

unsafe extern "C" fn elseek(
    mut fd: libc::c_int,
    mut offset: off_t,
    mut whence: libc::c_int,
    mut filename: *const libc::c_char,
) -> off_t {
    let mut new_offset: off_t = lseek(fd, offset, whence);
    let mut buf: [libc::c_char; 21] = [0; 21];
    if new_offset < 0 as libc::c_int as libc::c_long {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    if whence == 0 as libc::c_int {
                        b"%s: cannot seek to offset %s\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"%s: cannot seek to relative offset %s\0" as *const u8
                            as *const libc::c_char
                    },
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    filename,
                ),
                offtostr(offset, buf.as_mut_ptr()),
            );
            if 0 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(
                        if whence == 0 as libc::c_int {
                            b"%s: cannot seek to offset %s\0" as *const u8
                                as *const libc::c_char
                        } else {
                            b"%s: cannot seek to relative offset %s\0" as *const u8
                                as *const libc::c_char
                        },
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        filename,
                    ),
                    offtostr(offset, buf.as_mut_ptr()),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(
                        if whence == 0 as libc::c_int {
                            b"%s: cannot seek to offset %s\0" as *const u8
                                as *const libc::c_char
                        } else {
                            b"%s: cannot seek to relative offset %s\0" as *const u8
                                as *const libc::c_char
                        },
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        filename,
                    ),
                    offtostr(offset, buf.as_mut_ptr()),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return new_offset;
}
unsafe extern "C" fn elide_tail_bytes_pipe(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_elide_0: uintmax_t,
    mut current_pos: off_t,
) -> bool {
    let mut n_elide: size_t = n_elide_0;
    let mut desired_pos: uintmax_t = current_pos as uintmax_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    if (18446744073709551615 as libc::c_ulong)
        < n_elide_0.wrapping_add(8192 as libc::c_int as libc::c_ulong)
    {
        let mut umax_buf: [libc::c_char; 21] = [0; 21];
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s: number of bytes is too large\0" as *const u8
                        as *const libc::c_char,
                ),
                umaxtostr(n_elide_0, umax_buf.as_mut_ptr()),
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"%s: number of bytes is too large\0" as *const u8
                            as *const libc::c_char,
                    ),
                    umaxtostr(n_elide_0, umax_buf.as_mut_ptr()),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"%s: number of bytes is too large\0" as *const u8
                            as *const libc::c_char,
                    ),
                    umaxtostr(n_elide_0, umax_buf.as_mut_ptr()),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if n_elide <= (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
        let mut first: bool = 1 as libc::c_int != 0;
        let mut eof: bool = 0 as libc::c_int != 0;
        let mut n_to_read: size_t = (8192 as libc::c_int as libc::c_ulong)
            .wrapping_add(n_elide);
        let mut i: bool = false;
        let mut b: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
        b[0 as libc::c_int
            as usize] = xnmalloc(2 as libc::c_int as size_t, n_to_read)
            as *mut libc::c_char;
        b[1 as libc::c_int
            as usize] = (b[0 as libc::c_int as usize]).offset(n_to_read as isize);
        i = 0 as libc::c_int != 0;
        while !eof {
            let mut n_read: size_t = full_read(
                fd,
                b[i as usize] as *mut libc::c_void,
                n_to_read,
            );
            let mut delta: size_t = 0 as libc::c_int as size_t;
            if n_read < n_to_read {
                if *__errno_location() != 0 as libc::c_int {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, filename),
                        );
                        if 0 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, filename),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, filename),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    ok = 0 as libc::c_int != 0;
                    break;
                } else {
                    if n_read <= n_elide {
                        if !first {
                            delta = n_elide.wrapping_sub(n_read);
                        }
                    }
                    eof = 1 as libc::c_int != 0;
                }
            }
            if !first {
                desired_pos = (desired_pos as libc::c_ulong)
                    .wrapping_add(n_elide.wrapping_sub(delta)) as uintmax_t as uintmax_t;
                xwrite_stdout(
                    (b[!i as libc::c_int as usize]).offset(8192 as libc::c_int as isize),
                    n_elide.wrapping_sub(delta),
                );
            }
            first = 0 as libc::c_int != 0;
            if n_elide < n_read {
                desired_pos = (desired_pos as libc::c_ulong)
                    .wrapping_add(n_read.wrapping_sub(n_elide)) as uintmax_t
                    as uintmax_t;
                xwrite_stdout(b[i as usize], n_read.wrapping_sub(n_elide));
            }
            i = !i;
        }
        free(b[0 as libc::c_int as usize] as *mut libc::c_void);
    } else {
        let mut current_block_69: u64;
        let mut eof_0: bool = 0 as libc::c_int != 0;
        let mut n_read_0: size_t = 0;
        let mut buffered_enough: bool = false;
        let mut i_0: size_t = 0;
        let mut i_next: size_t = 0;
        let mut b_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut rem: size_t = (8192 as libc::c_int as libc::c_ulong)
            .wrapping_sub(n_elide.wrapping_rem(8192 as libc::c_int as libc::c_ulong));
        let mut n_elide_round: size_t = n_elide.wrapping_add(rem);
        let mut n_bufs: size_t = n_elide_round
            .wrapping_div(8192 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut n_alloc: size_t = 0 as libc::c_int as size_t;
        let mut n_array_alloc: size_t = 0 as libc::c_int as size_t;
        buffered_enough = 0 as libc::c_int != 0;
        i_0 = 0 as libc::c_int as size_t;
        i_next = 1 as libc::c_int as size_t;
        loop {
            if eof_0 {
                current_block_69 = 10753070352654377903;
                break;
            }
            if n_array_alloc == i_0 {
                if n_array_alloc == 0 as libc::c_int as libc::c_ulong {
                    n_array_alloc = if n_bufs < 16 as libc::c_int as libc::c_ulong {
                        n_bufs
                    } else {
                        16 as libc::c_int as libc::c_ulong
                    };
                } else if n_array_alloc
                    <= n_bufs.wrapping_div(2 as libc::c_int as libc::c_ulong)
                {
                    n_array_alloc = (n_array_alloc as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                } else {
                    n_array_alloc = n_bufs;
                }
                b_0 = xnrealloc(
                    b_0 as *mut libc::c_void,
                    n_array_alloc,
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) as *mut *mut libc::c_char;
            }
            if !buffered_enough {
                let ref mut fresh0 = *b_0.offset(i_0 as isize);
                *fresh0 = xmalloc(8192 as libc::c_int as size_t) as *mut libc::c_char;
                n_alloc = i_0.wrapping_add(1 as libc::c_int as libc::c_ulong);
            }
            n_read_0 = full_read(
                fd,
                *b_0.offset(i_0 as isize) as *mut libc::c_void,
                8192 as libc::c_int as size_t,
            );
            if n_read_0 < 8192 as libc::c_int as libc::c_ulong {
                if *__errno_location() != 0 as libc::c_int {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, filename),
                        );
                        if 0 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, filename),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, filename),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    ok = 0 as libc::c_int != 0;
                    current_block_69 = 10121186216371937393;
                    break;
                } else {
                    eof_0 = 1 as libc::c_int != 0;
                }
            }
            if i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) == n_bufs {
                buffered_enough = 1 as libc::c_int != 0;
            }
            if buffered_enough {
                desired_pos = (desired_pos as libc::c_ulong).wrapping_add(n_read_0)
                    as uintmax_t as uintmax_t;
                xwrite_stdout(*b_0.offset(i_next as isize), n_read_0);
            }
            i_0 = i_next;
            i_next = i_next
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_rem(n_bufs);
        }
        match current_block_69 {
            10753070352654377903 => {
                if rem != 0 {
                    if buffered_enough {
                        let mut n_bytes_left_in_b_i: size_t = (8192 as libc::c_int
                            as libc::c_ulong)
                            .wrapping_sub(n_read_0);
                        desired_pos = (desired_pos as libc::c_ulong).wrapping_add(rem)
                            as uintmax_t as uintmax_t;
                        if rem < n_bytes_left_in_b_i {
                            xwrite_stdout(
                                (*b_0.offset(i_0 as isize)).offset(n_read_0 as isize),
                                rem,
                            );
                        } else {
                            xwrite_stdout(
                                (*b_0.offset(i_0 as isize)).offset(n_read_0 as isize),
                                n_bytes_left_in_b_i,
                            );
                            xwrite_stdout(
                                *b_0.offset(i_next as isize),
                                rem.wrapping_sub(n_bytes_left_in_b_i),
                            );
                        }
                    } else if i_0.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        == n_bufs
                    {
                        let mut y: size_t = (8192 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(rem);
                        let mut x: size_t = n_read_0.wrapping_sub(y);
                        desired_pos = (desired_pos as libc::c_ulong).wrapping_add(x)
                            as uintmax_t as uintmax_t;
                        xwrite_stdout(*b_0.offset(i_next as isize), x);
                    }
                }
            }
            _ => {}
        }
        i_0 = 0 as libc::c_int as size_t;
        while i_0 < n_alloc {
            free(*b_0.offset(i_0 as isize) as *mut libc::c_void);
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        free(b_0 as *mut libc::c_void);
    }
    if 0 as libc::c_int as libc::c_long <= current_pos
        && elseek(fd, desired_pos as off_t, 0 as libc::c_int, filename)
            < 0 as libc::c_int as libc::c_long
    {
        ok = 0 as libc::c_int != 0;
    }
    return ok;
}
unsafe extern "C" fn elide_tail_bytes_file(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_elide: uintmax_t,
    mut st: *const stat,
    mut current_pos: off_t,
) -> bool {
    let mut size: off_t = (*st).st_size;
    if presume_input_pipe as libc::c_int != 0
        || current_pos < 0 as libc::c_int as libc::c_long
        || size
            <= (if (0 as libc::c_int) < (*st).st_blksize
                && (*st).st_blksize as libc::c_ulong
                    <= (-(1 as libc::c_int) as size_t)
                        .wrapping_div(8 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                (*st).st_blksize
            } else {
                512 as libc::c_int
            }) as libc::c_long
    {
        return elide_tail_bytes_pipe(filename, fd, n_elide, current_pos)
    } else {
        let mut diff: off_t = size - current_pos;
        let mut bytes_remaining: off_t = if diff < 0 as libc::c_int as libc::c_long {
            0 as libc::c_int as libc::c_long
        } else {
            diff
        };
        if bytes_remaining as libc::c_ulong <= n_elide {
            return 1 as libc::c_int != 0;
        }
        let bytes_to_copy = (bytes_remaining as u64).wrapping_sub(n_elide as u64);
let err: Copy_fd_status = copy_fd(fd, bytes_to_copy);
        if err as libc::c_uint == COPY_FD_OK as libc::c_int as libc::c_uint {
            return 1 as libc::c_int != 0;
        }
        diagnose_copy_fd_failure(err, filename);
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn elide_tail_lines_pipe(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_elide: uintmax_t,
    mut current_pos: off_t,
) -> bool {
    let mut desired_pos: uintmax_t = current_pos as uintmax_t;
    let mut first: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut last: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut tmp: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut total_lines: size_t = 0 as libc::c_int as size_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_read: size_t = 0;
    last = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong) as *mut LBUFFER;
    first = last;
    (*first).nlines = 0 as libc::c_int as size_t;
    (*first).nbytes = (*first).nlines;
    (*first).next = 0 as *mut linebuffer;
    tmp = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong) as *mut LBUFFER;
    loop {
        n_read = safe_read(
            fd,
            ((*tmp).buffer).as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if n_read == 0 as libc::c_int as libc::c_ulong
            || n_read == -(1 as libc::c_int) as size_t
        {
            break;
        }
        if n_elide == 0 {
            desired_pos = (desired_pos as libc::c_ulong).wrapping_add(n_read)
                as uintmax_t as uintmax_t;
            xwrite_stdout(((*tmp).buffer).as_mut_ptr(), n_read);
        } else {
            (*tmp).nbytes = n_read;
            (*tmp).nlines = 0 as libc::c_int as size_t;
            (*tmp).next = 0 as *mut linebuffer;
            let mut buffer_end: *mut libc::c_char = ((*tmp).buffer)
                .as_mut_ptr()
                .offset(n_read as isize);
            *buffer_end = line_end;
            let mut p: *const libc::c_char = ((*tmp).buffer).as_mut_ptr();
            loop {
                p = rawmemchr(p as *const libc::c_void, line_end as libc::c_int)
                    as *const libc::c_char;
                if !(p < buffer_end as *const libc::c_char) {
                    break;
                }
                p = p.offset(1);
                p;
                (*tmp).nlines = ((*tmp).nlines).wrapping_add(1);
                (*tmp).nlines;
            }
            total_lines = (total_lines as libc::c_ulong).wrapping_add((*tmp).nlines)
                as size_t as size_t;
            if ((*tmp).nbytes).wrapping_add((*last).nbytes)
                < 8192 as libc::c_int as libc::c_ulong
            {
                memcpy(
                    &mut *((*last).buffer).as_mut_ptr().offset((*last).nbytes as isize)
                        as *mut libc::c_char as *mut libc::c_void,
                    ((*tmp).buffer).as_mut_ptr() as *const libc::c_void,
                    (*tmp).nbytes,
                );
                (*last)
                    .nbytes = ((*last).nbytes as libc::c_ulong)
                    .wrapping_add((*tmp).nbytes) as size_t as size_t;
                (*last)
                    .nlines = ((*last).nlines as libc::c_ulong)
                    .wrapping_add((*tmp).nlines) as size_t as size_t;
            } else {
                (*last).next = tmp;
                last = (*last).next;
                if n_elide < total_lines.wrapping_sub((*first).nlines) {
                    desired_pos = (desired_pos as libc::c_ulong)
                        .wrapping_add((*first).nbytes) as uintmax_t as uintmax_t;
                    xwrite_stdout(((*first).buffer).as_mut_ptr(), (*first).nbytes);
                    tmp = first;
                    total_lines = (total_lines as libc::c_ulong)
                        .wrapping_sub((*first).nlines) as size_t as size_t;
                    first = (*first).next;
                } else {
                    tmp = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong)
                        as *mut LBUFFER;
                }
            }
        }
    }
    free(tmp as *mut libc::c_void);
    if n_read == -(1 as libc::c_int) as size_t {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, filename),
            );
            if 0 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ok = 0 as libc::c_int != 0;
    } else {
        if (*last).nbytes != 0
            && (*last)
                .buffer[((*last).nbytes).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int != line_end as libc::c_int
        {
            (*last).nlines = ((*last).nlines).wrapping_add(1);
            (*last).nlines;
            total_lines = total_lines.wrapping_add(1);
            total_lines;
        }
        tmp = first;
        while n_elide < total_lines.wrapping_sub((*tmp).nlines) {
            desired_pos = (desired_pos as libc::c_ulong).wrapping_add((*tmp).nbytes)
                as uintmax_t as uintmax_t;
            xwrite_stdout(((*tmp).buffer).as_mut_ptr(), (*tmp).nbytes);
            total_lines = (total_lines as libc::c_ulong).wrapping_sub((*tmp).nlines)
                as size_t as size_t;
            tmp = (*tmp).next;
        }
        if n_elide < total_lines {
            let mut n: size_t = total_lines.wrapping_sub(n_elide);
            let mut buffer_end_0: *const libc::c_char = ((*tmp).buffer)
                .as_mut_ptr()
                .offset((*tmp).nbytes as isize);
            let mut p_0: *const libc::c_char = ((*tmp).buffer).as_mut_ptr();
            while n != 0
                && {
                    p_0 = memchr(
                        p_0 as *const libc::c_void,
                        line_end as libc::c_int,
                        buffer_end_0.offset_from(p_0) as libc::c_long as libc::c_ulong,
                    ) as *const libc::c_char;
                    !p_0.is_null()
                }
            {
                p_0 = p_0.offset(1);
                p_0;
                (*tmp).nlines = ((*tmp).nlines).wrapping_add(1);
                (*tmp).nlines;
                n = n.wrapping_sub(1);
                n;
            }
            desired_pos = (desired_pos as libc::c_ulong)
                .wrapping_add(
                    p_0.offset_from(((*tmp).buffer).as_mut_ptr()) as libc::c_long
                        as libc::c_ulong,
                ) as uintmax_t as uintmax_t;
            xwrite_stdout(
                ((*tmp).buffer).as_mut_ptr(),
                p_0.offset_from(((*tmp).buffer).as_mut_ptr()) as libc::c_long as size_t,
            );
        }
    }
    while !first.is_null() {
        tmp = (*first).next;
        free(first as *mut libc::c_void);
        first = tmp;
    }
    if 0 as libc::c_int as libc::c_long <= current_pos
        && elseek(fd, desired_pos as off_t, 0 as libc::c_int, filename)
            < 0 as libc::c_int as libc::c_long
    {
        ok = 0 as libc::c_int != 0;
    }
    return ok;
}
fn elide_tail_lines_seekable(
    pretty_filename: *const libc::c_char,
    fd: libc::c_int,
    n_lines: usize,
    start_pos: i64,
    size: i64,
) -> bool {
    let mut buffer: Vec<u8> = vec![0; 8192];
    let mut bytes_read: usize;
    let mut pos = size;
    bytes_read = ((pos - start_pos) % 8192) as usize;
    if bytes_read == 0 {
        bytes_read = 8192;
    }
    pos -= bytes_read as i64;

    unsafe {
        if elseek(fd, pos, 0, pretty_filename) < 0 {
            return false;
        }

        bytes_read = safe_read(fd, buffer.as_mut_ptr() as *mut libc::c_void, bytes_read.try_into().unwrap()) as usize;
        if bytes_read == usize::MAX {
            error(
                0,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
            );
            return false;
        }
    }

    let all_lines = n_lines == 0;
    let mut n_lines = n_lines;
    if n_lines != 0 && bytes_read != 0
        && buffer[bytes_read - 1] != b'\n' {
        n_lines = n_lines.saturating_sub(1);
    }

    loop {
        let mut n = bytes_read;
        while n != 0 {
            if all_lines {
                n = n.saturating_sub(1);
            } else {
                if let Some(nl) = buffer[..n].iter().rposition(|&b| b == b'\n') {
                    n = nl + 1;
                } else {
                    break;
                }
            }
            let fresh1 = n_lines;
            n_lines = n_lines.saturating_sub(1);
            if fresh1 == 0 {
                if start_pos < pos {
                    unsafe {
                        if elseek(fd, start_pos, 0, pretty_filename) < 0 {
                            return false;
                        }
                        let err = copy_fd(fd, (pos - start_pos) as u64);
                        if err != COPY_FD_OK {
                            diagnose_copy_fd_failure(err, pretty_filename);
                            return false;
                        }
                    }
                }
                unsafe {
                    xwrite_stdout(buffer.as_ptr() as *const libc::c_char, n.try_into().unwrap());
                }
                return unsafe { elseek(fd, pos + n as i64 + 1, 0, pretty_filename) >= 0 };
            }
        }
        if pos == start_pos {
            return true;
        }
        pos -= 8192;
        unsafe {
            if elseek(fd, pos, 0, pretty_filename) < 0 {
                return false;
            }
            bytes_read = safe_read(fd, buffer.as_mut_ptr() as *mut libc::c_void, 8192.try_into().unwrap()) as usize;
            if bytes_read == usize::MAX {
                error(
                    0,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                return false;
            }
        }
        if bytes_read == 0 {
            return true;
        }
    }
}

unsafe extern "C" fn elide_tail_lines_file(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_elide: uintmax_t,
    mut st: *const stat,
    mut current_pos: off_t,
) -> bool {
    let mut size: off_t = (*st).st_size;
    if presume_input_pipe as libc::c_int != 0
        || current_pos < 0 as libc::c_int as libc::c_long
        || size
            <= (if (0 as libc::c_int) < (*st).st_blksize
                && (*st).st_blksize as libc::c_ulong
                    <= (-(1 as libc::c_int) as size_t)
                        .wrapping_div(8 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                (*st).st_blksize
            } else {
                512 as libc::c_int
            }) as libc::c_long
    {
        return elide_tail_lines_pipe(filename, fd, n_elide, current_pos)
    } else {
    return size <= current_pos
        || elide_tail_lines_seekable(filename, fd, n_elide.try_into().unwrap(), current_pos, size);
};
}
unsafe extern "C" fn head_bytes(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut bytes_to_write: uintmax_t,
) -> bool {
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    let mut bytes_to_read: size_t = 8192 as libc::c_int as size_t;
    while bytes_to_write != 0 {
        let mut bytes_read: size_t = 0;
        if bytes_to_write < bytes_to_read {
            bytes_to_read = bytes_to_write;
        }
        bytes_read = safe_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            bytes_to_read,
        );
        if bytes_read == -(1 as libc::c_int) as size_t {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 0 as libc::c_int != 0;
        }
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        xwrite_stdout(buffer.as_mut_ptr(), bytes_read);
        bytes_to_write = (bytes_to_write as libc::c_ulong).wrapping_sub(bytes_read)
            as uintmax_t as uintmax_t;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn head_lines(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut lines_to_write: uintmax_t,
) -> bool {
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    while lines_to_write != 0 {
        let mut bytes_read: size_t = safe_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        let mut bytes_to_write: size_t = 0 as libc::c_int as size_t;
        if bytes_read == -(1 as libc::c_int) as size_t {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 0 as libc::c_int != 0;
        }
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        while bytes_to_write < bytes_read {
            let fresh2 = bytes_to_write;
            bytes_to_write = bytes_to_write.wrapping_add(1);
            if !(buffer[fresh2 as usize] as libc::c_int == line_end as libc::c_int
                && {
                    lines_to_write = lines_to_write.wrapping_sub(1);
                    lines_to_write == 0 as libc::c_int as libc::c_ulong
                })
            {
                continue;
            }
            let mut n_bytes_past_EOL: off_t = bytes_read.wrapping_sub(bytes_to_write)
                as off_t;
            if lseek(fd, -n_bytes_past_EOL, 1 as libc::c_int)
                < 0 as libc::c_int as libc::c_long
            {
                let mut st: stat = stat {
                    st_dev: 0,
                    st_ino: 0,
                    st_mode: 0,
                    st_nlink: 0,
                    st_uid: 0,
                    st_gid: 0,
                    st_rdev: 0,
                    __pad1: 0,
                    st_size: 0,
                    st_blksize: 0,
                    __pad2: 0,
                    st_blocks: 0,
                    st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                    st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                    st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                    __glibc_reserved: [0; 2],
                };
                if fstat(fd, &mut st) != 0 as libc::c_int
                    || st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o100000 as libc::c_int as libc::c_uint
                {
                    elseek(fd, -n_bytes_past_EOL, 1 as libc::c_int, filename);
                }
            }
            break;
        }
        xwrite_stdout(buffer.as_mut_ptr(), bytes_to_write);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn head(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_units: uintmax_t,
    mut count_lines: bool,
    mut elide_from_end: bool,
) -> bool {
    if print_headers {
        let filename_str = unsafe { CStr::from_ptr(filename).to_string_lossy().into_owned() };
write_header(&filename_str);
    }
    if elide_from_end {
        let mut current_pos: off_t = -(1 as libc::c_int) as off_t;
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            __pad1: 0,
            st_size: 0,
            st_blksize: 0,
            __pad2: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 2],
        };
        if fstat(fd, &mut st) != 0 as libc::c_int {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"cannot fstat %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"cannot fstat %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 0 as libc::c_int != 0;
        }
        if !presume_input_pipe && usable_st_size(&mut st) as libc::c_int != 0 {
            current_pos = elseek(
                fd,
                0 as libc::c_int as off_t,
                1 as libc::c_int,
                filename,
            );
            if current_pos < 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int != 0;
            }
        }
        if count_lines {
            return elide_tail_lines_file(filename, fd, n_units, &mut st, current_pos)
        } else {
            return elide_tail_bytes_file(filename, fd, n_units, &mut st, current_pos)
        }
    }
    if count_lines {
        return head_lines(filename, fd, n_units)
    } else {
        return head_bytes(filename, fd, n_units)
    };
}
unsafe extern "C" fn head_file(
    mut filename: *const libc::c_char,
    mut n_units: uintmax_t,
    mut count_lines: bool,
    mut elide_from_end: bool,
) -> bool {
    let mut fd: libc::c_int = 0;
    let mut ok: bool = false;
    let mut is_stdin: bool = strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    if is_stdin {
        have_read_stdin = 1 as libc::c_int != 0;
        fd = 0 as libc::c_int;
        filename = gettext(b"standard input\0" as *const u8 as *const libc::c_char);
        let fd1: i32 = 0;
let mode1: i32 = 0;
xset_binary_mode(fd1, mode1);
    } else {
        fd = open(filename, 0 as libc::c_int | 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot open %s for reading\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"cannot open %s for reading\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"cannot open %s for reading\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, filename),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 0 as libc::c_int != 0;
        }
    }
    ok = head(filename, fd, n_units, count_lines, elide_from_end);
    if !is_stdin && close(fd) != 0 as libc::c_int {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"failed to close %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, filename),
            );
            if 0 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(b"failed to close %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(b"failed to close %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        return 0 as libc::c_int != 0;
    }
    return ok;
}
unsafe extern "C" fn string_to_integer(
    mut count_lines: bool,
    mut n_string: *const libc::c_char,
) -> uintmax_t {
    return xdectoumax(
        n_string,
        0 as libc::c_int as uintmax_t,
        18446744073709551615 as libc::c_ulong,
        b"bkKmMGTPEZYRQ0\0" as *const u8 as *const libc::c_char,
        if count_lines as libc::c_int != 0 {
            gettext(b"invalid number of lines\0" as *const u8 as *const libc::c_char)
        } else {
            gettext(b"invalid number of bytes\0" as *const u8 as *const libc::c_char)
        },
        0 as libc::c_int,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut header_mode: header_mode = multiple_files;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut c: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut n_units: uintmax_t = 10 as libc::c_int as uintmax_t;
    let mut count_lines: bool = 1 as libc::c_int != 0;
    let mut elide_from_end: bool = 0 as libc::c_int != 0;
    static mut default_file_list: [*const libc::c_char; 2] = [
        b"-\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut file_list: *const *const libc::c_char = 0 as *const *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    have_read_stdin = 0 as libc::c_int != 0;
    print_headers = 0 as libc::c_int != 0;
    line_end = '\n' as i32 as libc::c_char;
    if (1 as libc::c_int) < argc
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        && (*(*argv.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
    {
        let mut a: *mut libc::c_char = *argv.offset(1 as libc::c_int as isize);
        a = a.offset(1);
        let mut n_string: *mut libc::c_char = a;
        let mut end_n_string: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut multiplier_char: libc::c_char = 0 as libc::c_int as libc::c_char;
        loop {
            a = a.offset(1);
            a;
            if !((*a as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
        end_n_string = a;
        while *a != 0 {
            match *a as libc::c_int {
                99 => {
                    count_lines = 0 as libc::c_int != 0;
                    multiplier_char = 0 as libc::c_int as libc::c_char;
                }
                98 | 107 | 109 => {
                    count_lines = 0 as libc::c_int != 0;
                    multiplier_char = *a;
                }
                108 => {
                    count_lines = 1 as libc::c_int != 0;
                }
                113 => {
                    header_mode = never;
                }
                118 => {
                    header_mode = always;
                }
                122 => {
                    line_end = '\0' as i32 as libc::c_char;
                }
                _ => {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"invalid trailing option -- %c\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            *a as libc::c_int,
                        );
                        if 0 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"invalid trailing option -- %c\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                *a as libc::c_int,
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"invalid trailing option -- %c\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                *a as libc::c_int,
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    usage(1 as libc::c_int);
                }
            }
            a = a.offset(1);
            a;
        }
        *end_n_string = multiplier_char;
        if multiplier_char != 0 {
            end_n_string = end_n_string.offset(1);
            *end_n_string = 0 as libc::c_int as libc::c_char;
        }
        n_units = string_to_integer(count_lines, n_string);
        let ref mut fresh3 = *argv.offset(1 as libc::c_int as isize);
        *fresh3 = *argv.offset(0 as libc::c_int as isize);
        argv = argv.offset(1);
        argv;
        argc -= 1;
        argc;
    }
    loop {
        c = getopt_long(
            argc,
            argv,
            b"c:n:qvz0123456789\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            256 => {
                presume_input_pipe = 1 as libc::c_int != 0;
            }
            99 => {
                count_lines = 0 as libc::c_int != 0;
                elide_from_end = *optarg as libc::c_int == '-' as i32;
                if elide_from_end {
                    optarg = optarg.offset(1);
                    optarg;
                }
                n_units = string_to_integer(count_lines, optarg);
            }
            110 => {
                count_lines = 1 as libc::c_int != 0;
                elide_from_end = *optarg as libc::c_int == '-' as i32;
                if elide_from_end {
                    optarg = optarg.offset(1);
                    optarg;
                }
                n_units = string_to_integer(count_lines, optarg);
            }
            113 => {
                header_mode = never;
            }
            118 => {
                header_mode = always;
            }
            122 => {
                line_end = '\0' as i32 as libc::c_char;
            }
            -2 => {
                usage(0);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"head\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                        b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                if (c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint
                {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"invalid trailing option -- %c\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            c,
                        );
                        if 0 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"invalid trailing option -- %c\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                c,
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"invalid trailing option -- %c\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                c,
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                usage(1);
            }
        }
    }
    if header_mode as libc::c_uint == always as libc::c_int as libc::c_uint
        || header_mode as libc::c_uint == multiple_files as libc::c_int as libc::c_uint
            && optind < argc - 1 as libc::c_int
    {
        print_headers = 1 as libc::c_int != 0;
    }
    if !count_lines && elide_from_end as libc::c_int != 0
        && ((if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as libc::c_ulong) < n_units
    {
        let mut umax_buf: [libc::c_char; 21] = [0; 21];
        if 0 != 0 {
            error(
                1 as libc::c_int,
                75 as libc::c_int,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                gettext(
                    b"invalid number of bytes\0" as *const u8 as *const libc::c_char,
                ),
                quote(umaxtostr(n_units, umax_buf.as_mut_ptr())),
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    75 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid number of bytes\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(umaxtostr(n_units, umax_buf.as_mut_ptr())),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    75 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid number of bytes\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(umaxtostr(n_units, umax_buf.as_mut_ptr())),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    file_list = if optind < argc {
        &mut *argv.offset(optind as isize) as *mut *mut libc::c_char
            as *const *const libc::c_char
    } else {
        default_file_list.as_ptr()
    };
    let fd2: i32 = 1;
let mode2: i32 = 0;
xset_binary_mode(fd2, mode2);
    i = 0 as libc::c_int as size_t;
    while !(*file_list.offset(i as isize)).is_null() {
        ok = (ok as libc::c_int
            & head_file(
                *file_list.offset(i as isize),
                n_units,
                count_lines,
                elide_from_end,
            ) as libc::c_int) != 0;
        i = i.wrapping_add(1);
        i;
    }
    if have_read_stdin as libc::c_int != 0 && close(0 as libc::c_int) < 0 as libc::c_int
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"-\0" as *const u8 as *const libc::c_char,
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"-\0" as *const u8 as *const libc::c_char,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"-\0" as *const u8 as *const libc::c_char,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let argc = args.len() as libc::c_int;
    let argv: Vec<CString> = args.iter()
        .map(|arg| CString::new(arg.clone()).expect("Failed to convert argument into CString"))
        .collect();

    let mut argv_ptr: Vec<*mut libc::c_char> = argv.iter()
        .map(|s| s.as_ptr() as *mut libc::c_char)
        .collect();
    
    argv_ptr.push(std::ptr::null_mut()); // Add null terminator

    let result = unsafe { main_0(argc, argv_ptr.as_mut_ptr()) };
    ::std::process::exit(result as i32);
}

