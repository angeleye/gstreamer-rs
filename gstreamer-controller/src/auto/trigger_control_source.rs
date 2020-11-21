// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TimedValueControlSource;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct TriggerControlSource(Object<ffi::GstTriggerControlSource, ffi::GstTriggerControlSourceClass>) @extends TimedValueControlSource, gst::ControlSource, gst::Object;

    match fn {
        get_type => || ffi::gst_trigger_control_source_get_type(),
    }
}

impl TriggerControlSource {
    pub fn new() -> TriggerControlSource {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlSource::from_glib_full(ffi::gst_trigger_control_source_new()).unsafe_cast()
        }
    }
}

impl Default for TriggerControlSource {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for TriggerControlSource {}
unsafe impl Sync for TriggerControlSource {}

pub const NONE_TRIGGER_CONTROL_SOURCE: Option<&TriggerControlSource> = None;

pub trait TriggerControlSourceExt: 'static {
    fn get_property_tolerance(&self) -> i64;

    fn set_property_tolerance(&self, tolerance: i64);

    fn connect_property_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<TriggerControlSource>> TriggerControlSourceExt for O {
    fn get_property_tolerance(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"tolerance\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tolerance` getter")
                .unwrap()
        }
    }

    fn set_property_tolerance(&self, tolerance: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"tolerance\0".as_ptr() as *const _,
                Value::from(&tolerance).to_glib_none().0,
            );
        }
    }

    fn connect_property_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tolerance_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstTriggerControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TriggerControlSource>,
        {
            let f: &F = &*(f as *const F);
            f(&TriggerControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tolerance\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tolerance_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
