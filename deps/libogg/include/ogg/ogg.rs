cargo:warning=couldn't execute `llvm-config --prefix` (error: The system cannot find the file specified. (os error 2))
cargo:warning=set the LLVM_CONFIG_PATH environment variable to the full path to a valid `llvm-config` executable (including the executable itself)
/* automatically generated by rust-bindgen */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub type size_t = ::std::os::raw::c_ulonglong;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub type ogg_int16_t = i16;
pub type ogg_uint16_t = u16;
pub type ogg_int32_t = i32;
pub type ogg_uint32_t = u32;
pub type ogg_int64_t = i64;
pub type ogg_uint64_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_iovec_t {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: size_t,
}
#[test]
fn bindgen_test_layout_ogg_iovec_t() {
    assert_eq!(
        ::std::mem::size_of::<ogg_iovec_t>(),
        16usize,
        concat!("Size of: ", stringify!(ogg_iovec_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ogg_iovec_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_iovec_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_iovec_t>())).iov_base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_iovec_t),
            "::",
            stringify!(iov_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_iovec_t>())).iov_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_iovec_t),
            "::",
            stringify!(iov_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oggpack_buffer {
    pub endbyte: ::std::os::raw::c_long,
    pub endbit: ::std::os::raw::c_int,
    pub buffer: *mut ::std::os::raw::c_uchar,
    pub ptr: *mut ::std::os::raw::c_uchar,
    pub storage: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_oggpack_buffer() {
    assert_eq!(
        ::std::mem::size_of::<oggpack_buffer>(),
        32usize,
        concat!("Size of: ", stringify!(oggpack_buffer))
    );
    assert_eq!(
        ::std::mem::align_of::<oggpack_buffer>(),
        8usize,
        concat!("Alignment of ", stringify!(oggpack_buffer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).endbyte as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(oggpack_buffer),
            "::",
            stringify!(endbyte)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).endbit as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(oggpack_buffer),
            "::",
            stringify!(endbit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).buffer as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(oggpack_buffer),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).ptr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(oggpack_buffer),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).storage as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(oggpack_buffer),
            "::",
            stringify!(storage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_page {
    pub header: *mut ::std::os::raw::c_uchar,
    pub header_len: ::std::os::raw::c_long,
    pub body: *mut ::std::os::raw::c_uchar,
    pub body_len: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ogg_page() {
    assert_eq!(
        ::std::mem::size_of::<ogg_page>(),
        32usize,
        concat!("Size of: ", stringify!(ogg_page))
    );
    assert_eq!(
        ::std::mem::align_of::<ogg_page>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_page))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).header as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_page),
            "::",
            stringify!(header)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).header_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_page),
            "::",
            stringify!(header_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).body as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_page),
            "::",
            stringify!(body)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).body_len as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_page),
            "::",
            stringify!(body_len)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ogg_stream_state {
    pub body_data: *mut ::std::os::raw::c_uchar,
    pub body_storage: ::std::os::raw::c_long,
    pub body_fill: ::std::os::raw::c_long,
    pub body_returned: ::std::os::raw::c_long,
    pub lacing_vals: *mut ::std::os::raw::c_int,
    pub granule_vals: *mut ogg_int64_t,
    pub lacing_storage: ::std::os::raw::c_long,
    pub lacing_fill: ::std::os::raw::c_long,
    pub lacing_packet: ::std::os::raw::c_long,
    pub lacing_returned: ::std::os::raw::c_long,
    pub header: [::std::os::raw::c_uchar; 282usize],
    pub header_fill: ::std::os::raw::c_int,
    pub e_o_s: ::std::os::raw::c_int,
    pub b_o_s: ::std::os::raw::c_int,
    pub serialno: ::std::os::raw::c_long,
    pub pageno: ::std::os::raw::c_long,
    pub packetno: ogg_int64_t,
    pub granulepos: ogg_int64_t,
}
#[test]
fn bindgen_test_layout_ogg_stream_state() {
    assert_eq!(
        ::std::mem::size_of::<ogg_stream_state>(),
        376usize,
        concat!("Size of: ", stringify!(ogg_stream_state))
    );
    assert_eq!(
        ::std::mem::align_of::<ogg_stream_state>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_stream_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(body_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_storage as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(body_storage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_fill as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(body_fill)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_returned as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(body_returned)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_vals as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(lacing_vals)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).granule_vals as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(granule_vals)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_storage as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(lacing_storage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_fill as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(lacing_fill)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_packet as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(lacing_packet)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ogg_stream_state>())).lacing_returned as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(lacing_returned)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).header as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(header)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).header_fill as *const _ as usize },
        340usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(header_fill)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).e_o_s as *const _ as usize },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(e_o_s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).b_o_s as *const _ as usize },
        348usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(b_o_s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).serialno as *const _ as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(serialno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).pageno as *const _ as usize },
        356usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(pageno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).packetno as *const _ as usize },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(packetno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).granulepos as *const _ as usize },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_stream_state),
            "::",
            stringify!(granulepos)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_packet {
    pub packet: *mut ::std::os::raw::c_uchar,
    pub bytes: ::std::os::raw::c_long,
    pub b_o_s: ::std::os::raw::c_long,
    pub e_o_s: ::std::os::raw::c_long,
    pub granulepos: ogg_int64_t,
    pub packetno: ogg_int64_t,
}
#[test]
fn bindgen_test_layout_ogg_packet() {
    assert_eq!(
        ::std::mem::size_of::<ogg_packet>(),
        40usize,
        concat!("Size of: ", stringify!(ogg_packet))
    );
    assert_eq!(
        ::std::mem::align_of::<ogg_packet>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_packet))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).packet as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_packet),
            "::",
            stringify!(packet)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).bytes as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_packet),
            "::",
            stringify!(bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).b_o_s as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_packet),
            "::",
            stringify!(b_o_s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).e_o_s as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_packet),
            "::",
            stringify!(e_o_s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).granulepos as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_packet),
            "::",
            stringify!(granulepos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).packetno as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_packet),
            "::",
            stringify!(packetno)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ogg_sync_state {
    pub data: *mut ::std::os::raw::c_uchar,
    pub storage: ::std::os::raw::c_int,
    pub fill: ::std::os::raw::c_int,
    pub returned: ::std::os::raw::c_int,
    pub unsynced: ::std::os::raw::c_int,
    pub headerbytes: ::std::os::raw::c_int,
    pub bodybytes: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ogg_sync_state() {
    assert_eq!(
        ::std::mem::size_of::<ogg_sync_state>(),
        32usize,
        concat!("Size of: ", stringify!(ogg_sync_state))
    );
    assert_eq!(
        ::std::mem::align_of::<ogg_sync_state>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_sync_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_sync_state),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).storage as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_sync_state),
            "::",
            stringify!(storage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).fill as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_sync_state),
            "::",
            stringify!(fill)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).returned as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_sync_state),
            "::",
            stringify!(returned)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).unsynced as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_sync_state),
            "::",
            stringify!(unsynced)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).headerbytes as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_sync_state),
            "::",
            stringify!(headerbytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).bodybytes as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ogg_sync_state),
            "::",
            stringify!(bodybytes)
        )
    );
}
extern "C" {
    pub fn oggpack_writeinit(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpack_writecheck(b: *mut oggpack_buffer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn oggpack_writetrunc(b: *mut oggpack_buffer, bits: ::std::os::raw::c_long);
}
extern "C" {
    pub fn oggpack_writealign(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpack_writecopy(
        b: *mut oggpack_buffer,
        source: *mut ::std::os::raw::c_void,
        bits: ::std::os::raw::c_long,
    );
}
extern "C" {
    pub fn oggpack_reset(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpack_writeclear(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpack_readinit(
        b: *mut oggpack_buffer,
        buf: *mut ::std::os::raw::c_uchar,
        bytes: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn oggpack_write(
        b: *mut oggpack_buffer,
        value: ::std::os::raw::c_ulong,
        bits: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn oggpack_look(
        b: *mut oggpack_buffer,
        bits: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpack_look1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpack_adv(b: *mut oggpack_buffer, bits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn oggpack_adv1(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpack_read(
        b: *mut oggpack_buffer,
        bits: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpack_read1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpack_bytes(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpack_bits(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpack_get_buffer(b: *mut oggpack_buffer) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn oggpackB_writeinit(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpackB_writecheck(b: *mut oggpack_buffer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn oggpackB_writetrunc(b: *mut oggpack_buffer, bits: ::std::os::raw::c_long);
}
extern "C" {
    pub fn oggpackB_writealign(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpackB_writecopy(
        b: *mut oggpack_buffer,
        source: *mut ::std::os::raw::c_void,
        bits: ::std::os::raw::c_long,
    );
}
extern "C" {
    pub fn oggpackB_reset(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpackB_writeclear(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpackB_readinit(
        b: *mut oggpack_buffer,
        buf: *mut ::std::os::raw::c_uchar,
        bytes: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn oggpackB_write(
        b: *mut oggpack_buffer,
        value: ::std::os::raw::c_ulong,
        bits: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn oggpackB_look(
        b: *mut oggpack_buffer,
        bits: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpackB_look1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpackB_adv(b: *mut oggpack_buffer, bits: ::std::os::raw::c_int);
}
extern "C" {
    pub fn oggpackB_adv1(b: *mut oggpack_buffer);
}
extern "C" {
    pub fn oggpackB_read(
        b: *mut oggpack_buffer,
        bits: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpackB_read1(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpackB_bytes(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpackB_bits(b: *mut oggpack_buffer) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn oggpackB_get_buffer(b: *mut oggpack_buffer) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn ogg_stream_packetin(
        os: *mut ogg_stream_state,
        op: *mut ogg_packet,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_iovecin(
        os: *mut ogg_stream_state,
        iov: *mut ogg_iovec_t,
        count: ::std::os::raw::c_int,
        e_o_s: ::std::os::raw::c_long,
        granulepos: ogg_int64_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_pageout(
        os: *mut ogg_stream_state,
        og: *mut ogg_page,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_pageout_fill(
        os: *mut ogg_stream_state,
        og: *mut ogg_page,
        nfill: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_flush(os: *mut ogg_stream_state, og: *mut ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_flush_fill(
        os: *mut ogg_stream_state,
        og: *mut ogg_page,
        nfill: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_sync_init(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_sync_clear(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_sync_reset(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_sync_destroy(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_sync_check(oy: *mut ogg_sync_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_sync_buffer(
        oy: *mut ogg_sync_state,
        size: ::std::os::raw::c_long,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ogg_sync_wrote(
        oy: *mut ogg_sync_state,
        bytes: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_sync_pageseek(oy: *mut ogg_sync_state, og: *mut ogg_page) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn ogg_sync_pageout(oy: *mut ogg_sync_state, og: *mut ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_pagein(os: *mut ogg_stream_state, og: *mut ogg_page)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_packetout(
        os: *mut ogg_stream_state,
        op: *mut ogg_packet,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_packetpeek(
        os: *mut ogg_stream_state,
        op: *mut ogg_packet,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_init(
        os: *mut ogg_stream_state,
        serialno: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_clear(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_reset(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_reset_serialno(
        os: *mut ogg_stream_state,
        serialno: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_destroy(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_check(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_stream_eos(os: *mut ogg_stream_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_page_checksum_set(og: *mut ogg_page);
}
extern "C" {
    pub fn ogg_page_version(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_page_continued(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_page_bos(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_page_eos(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_page_granulepos(og: *const ogg_page) -> ogg_int64_t;
}
extern "C" {
    pub fn ogg_page_serialno(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_page_pageno(og: *const ogg_page) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn ogg_page_packets(og: *const ogg_page) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ogg_packet_clear(op: *mut ogg_packet);
}
