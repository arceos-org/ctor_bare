#![no_std]
#![doc = include_str!("../README.md")]

pub use constructor_array_macros::register_ctor;

/// Placeholder for the `ctors` section, so that
/// the `__start_ctors` and `__stop_ctors` symbols can be generated.
#[link_section = "ctors"]
#[used]
static _SECTION_PLACE_HOLDER: [u8; 0] = [];

extern "C" {
    fn __start_ctors();
    fn __stop_ctors();
}

/// Invoke all constructor functions registered by the `register_ctor` attribute.
///
/// # Notes
/// Caller should ensure that the `ctor` section will not be disturbed by other sections.
pub fn invoke_ctors() {
    for ctor_ptr in (__start_ctors as usize..__stop_ctors as usize)
        .step_by(core::mem::size_of::<*const core::ffi::c_void>())
    {
        unsafe {
            core::mem::transmute::<*const core::ffi::c_void, fn()>(
                *(ctor_ptr as *const *const core::ffi::c_void),
            )();
        }
    }
}
