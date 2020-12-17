// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    pub struct BufferPool(Object<ffi::GstBufferPool, ffi::GstBufferPoolClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_buffer_pool_get_type(),
    }
}

unsafe impl Send for BufferPool {}
unsafe impl Sync for BufferPool {}

pub const NONE_BUFFER_POOL: Option<&BufferPool> = None;

pub trait BufferPoolExt: 'static {
    #[doc(alias = "gst_buffer_pool_get_options")]
    fn get_options(&self) -> Vec<glib::GString>;

    #[doc(alias = "gst_buffer_pool_has_option")]
    fn has_option(&self, option: &str) -> bool;

    #[doc(alias = "gst_buffer_pool_is_active")]
    fn is_active(&self) -> bool;

    #[doc(alias = "gst_buffer_pool_set_active")]
    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_buffer_pool_set_flushing")]
    fn set_flushing(&self, flushing: bool);
}

impl<O: IsA<BufferPool>> BufferPoolExt for O {
    fn get_options(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_buffer_pool_get_options(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_option(&self, option: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_has_option(
                self.as_ref().to_glib_none().0,
                option.to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_is_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_buffer_pool_set_active(self.as_ref().to_glib_none().0, active.to_glib()),
                "Failed to activate buffer pool"
            )
        }
    }

    fn set_flushing(&self, flushing: bool) {
        unsafe {
            ffi::gst_buffer_pool_set_flushing(self.as_ref().to_glib_none().0, flushing.to_glib());
        }
    }
}
