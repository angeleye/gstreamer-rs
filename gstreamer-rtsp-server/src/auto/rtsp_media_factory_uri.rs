// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RTSPMediaFactory;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct RTSPMediaFactoryURI(Object<ffi::GstRTSPMediaFactoryURI, ffi::GstRTSPMediaFactoryURIClass>) @extends RTSPMediaFactory;

    match fn {
        get_type => || ffi::gst_rtsp_media_factory_uri_get_type(),
    }
}

impl RTSPMediaFactoryURI {
    #[doc(alias = "gst_rtsp_media_factory_uri_new")]
    pub fn new() -> RTSPMediaFactoryURI {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_media_factory_uri_new()) }
    }
}

impl Default for RTSPMediaFactoryURI {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMediaFactoryURI {}
unsafe impl Sync for RTSPMediaFactoryURI {}

pub const NONE_RTSP_MEDIA_FACTORY_URI: Option<&RTSPMediaFactoryURI> = None;

pub trait RTSPMediaFactoryURIExt: 'static {
    #[doc(alias = "gst_rtsp_media_factory_uri_get_uri")]
    fn get_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_rtsp_media_factory_uri_set_uri")]
    fn set_uri(&self, uri: &str);

    fn get_property_use_gstpay(&self) -> bool;

    fn set_property_use_gstpay(&self, use_gstpay: bool);

    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPMediaFactoryURI>> RTSPMediaFactoryURIExt for O {
    fn get_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_uri_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gst_rtsp_media_factory_uri_set_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    fn get_property_use_gstpay(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"use-gstpay\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `use-gstpay` getter")
                .unwrap()
        }
    }

    fn set_property_use_gstpay(&self, use_gstpay: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"use-gstpay\0".as_ptr() as *const _,
                glib::Value::from(&use_gstpay).to_glib_none().0,
            );
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPMediaFactoryURI,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPMediaFactoryURI>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactoryURI::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_uri_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_gstpay_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPMediaFactoryURI,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPMediaFactoryURI>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactoryURI::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-gstpay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_gstpay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
