#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type conditional_type<T> = T;
pub type conditional_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Container<_Val_types> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Val_types>>,
    pub data: _Val_types,
}
impl<_Val_types> Default for Container<_Val_types> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vector {
    pub field: vector_Scary_val,
}
pub type vector_Scary_val = u8;
impl Default for vector {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
