// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use GLContext;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gst;
use gst_gl_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct GLBaseFilter(Object<gst_gl_sys::GstGLBaseFilter, gst_gl_sys::GstGLBaseFilterClass, GLBaseFilterClass>) @extends gst::Object;

    match fn {
        get_type => || gst_gl_sys::gst_gl_base_filter_get_type(),
    }
}

unsafe impl Send for GLBaseFilter {}
unsafe impl Sync for GLBaseFilter {}

pub const NONE_GL_BASE_FILTER: Option<&GLBaseFilter> = None;

pub trait GLBaseFilterExt: 'static {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn find_gl_context(&self) -> bool;

    fn get_property_context(&self) -> Option<GLContext>;

    fn connect_property_context_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GLBaseFilter>> GLBaseFilterExt for O {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn find_gl_context(&self) -> bool {
        unsafe {
            from_glib(gst_gl_sys::gst_gl_base_filter_find_gl_context(self.as_ref().to_glib_none().0))
        }
    }

    fn get_property_context(&self) -> Option<GLContext> {
        unsafe {
            let mut value = Value::from_type(<GLContext as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"context\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_context_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::context\0".as_ptr() as *const _,
                Some(transmute(notify_context_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_context_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut gst_gl_sys::GstGLBaseFilter, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<GLBaseFilter> {
    let f: &F = &*(f as *const F);
    f(&GLBaseFilter::from_glib_borrow(this).unsafe_cast())
}