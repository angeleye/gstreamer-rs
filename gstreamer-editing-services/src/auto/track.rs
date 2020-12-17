// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Timeline;
use crate::TrackElement;
use crate::TrackType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::ptr;

glib::wrapper! {
    pub struct Track(Object<ffi::GESTrack, ffi::GESTrackClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || ffi::ges_track_get_type(),
    }
}

impl Track {
    #[doc(alias = "ges_track_new")]
    pub fn new(type_: TrackType, caps: &gst::Caps) -> Track {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_track_new(type_.to_glib(), caps.to_glib_full())) }
    }
}

pub const NONE_TRACK: Option<&Track> = None;

pub trait GESTrackExt: 'static {
    #[doc(alias = "ges_track_add_element")]
    fn add_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_add_element_full")]
    fn add_element_full<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::Error>;

    #[doc(alias = "ges_track_commit")]
    fn commit(&self) -> bool;

    #[doc(alias = "ges_track_get_caps")]
    fn get_caps(&self) -> Option<gst::Caps>;

    #[doc(alias = "ges_track_get_elements")]
    fn get_elements(&self) -> Vec<TrackElement>;

    #[doc(alias = "ges_track_get_mixing")]
    fn get_mixing(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_get_restriction_caps")]
    fn get_restriction_caps(&self) -> Option<gst::Caps>;

    #[doc(alias = "ges_track_get_timeline")]
    fn get_timeline(&self) -> Option<Timeline>;

    #[doc(alias = "ges_track_remove_element")]
    fn remove_element<P: IsA<TrackElement>>(
        &self,
        object: &P,
    ) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_remove_element_full")]
    fn remove_element_full<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::Error>;

    //#[doc(alias = "ges_track_set_create_element_for_gap_func")]
    //fn set_create_element_for_gap_func<P: Fn(&Track) -> gst::Element + 'static>(&self, func: P);

    #[doc(alias = "ges_track_set_mixing")]
    fn set_mixing(&self, mixing: bool);

    #[doc(alias = "ges_track_set_restriction_caps")]
    fn set_restriction_caps(&self, caps: &gst::Caps);

    #[doc(alias = "ges_track_set_timeline")]
    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P);

    #[doc(alias = "ges_track_update_restriction_caps")]
    fn update_restriction_caps(&self, caps: &gst::Caps);

    fn get_property_duration(&self) -> u64;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_property_id(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_property_id(&self, id: Option<&str>);

    fn get_property_restriction_caps(&self) -> Option<gst::Caps>;

    fn get_property_track_type(&self) -> TrackType;

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Track>> GESTrackExt for O {
    fn add_element<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_track_add_element(
                    self.as_ref().to_glib_none().0,
                    object.as_ref().to_glib_none().0
                ),
                "Failed to add element"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn add_element_full<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_track_add_element_full(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn commit(&self) -> bool {
        unsafe { from_glib(ffi::ges_track_commit(self.as_ref().to_glib_none().0)) }
    }

    fn get_caps(&self) -> Option<gst::Caps> {
        unsafe { from_glib_none(ffi::ges_track_get_caps(self.as_ref().to_glib_none().0)) }
    }

    fn get_elements(&self) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_track_get_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_mixing(&self) -> bool {
        unsafe { from_glib(ffi::ges_track_get_mixing(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_restriction_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::ges_track_get_restriction_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timeline(&self) -> Option<Timeline> {
        unsafe { from_glib_none(ffi::ges_track_get_timeline(self.as_ref().to_glib_none().0)) }
    }

    fn remove_element<P: IsA<TrackElement>>(
        &self,
        object: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_track_remove_element(
                    self.as_ref().to_glib_none().0,
                    object.as_ref().to_glib_none().0
                ),
                "Failed to remove element"
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn remove_element_full<P: IsA<TrackElement>>(&self, object: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_track_remove_element_full(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn set_create_element_for_gap_func<P: Fn(&Track) -> gst::Element + 'static>(&self, func: P) {
    //    unsafe { TODO: call ffi:ges_track_set_create_element_for_gap_func() }
    //}

    fn set_mixing(&self, mixing: bool) {
        unsafe {
            ffi::ges_track_set_mixing(self.as_ref().to_glib_none().0, mixing.to_glib());
        }
    }

    fn set_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::ges_track_set_restriction_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            );
        }
    }

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) {
        unsafe {
            ffi::ges_track_set_timeline(
                self.as_ref().to_glib_none().0,
                timeline.as_ref().to_glib_none().0,
            );
        }
    }

    fn update_restriction_caps(&self, caps: &gst::Caps) {
        unsafe {
            ffi::ges_track_update_restriction_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            );
        }
    }

    fn get_property_duration(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"duration\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `duration` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_property_id(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `id` getter")
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_property_id(&self, id: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"id\0".as_ptr() as *const _,
                glib::Value::from(id).to_glib_none().0,
            );
        }
    }

    fn get_property_restriction_caps(&self) -> Option<gst::Caps> {
        unsafe {
            let mut value = glib::Value::from_type(<gst::Caps as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"restriction-caps\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `restriction-caps` getter")
        }
    }

    fn get_property_track_type(&self) -> TrackType {
        unsafe {
            let mut value = glib::Value::from_type(<TrackType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"track-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `track-type` getter")
                .unwrap()
        }
    }

    fn connect_commited<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn commited_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Track>,
        {
            let f: &F = &*(f as *const F);
            f(&Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"commited\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    commited_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_track_element_added<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn track_element_added_trampoline<P, F: Fn(&P, &TrackElement) + 'static>(
            this: *mut ffi::GESTrack,
            effect: *mut ffi::GESTrackElement,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Track>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Track::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(effect),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"track-element-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    track_element_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_track_element_removed<F: Fn(&Self, &TrackElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn track_element_removed_trampoline<
            P,
            F: Fn(&P, &TrackElement) + 'static,
        >(
            this: *mut ffi::GESTrack,
            effect: *mut ffi::GESTrackElement,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Track>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Track::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(effect),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"track-element-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    track_element_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Track>,
        {
            let f: &F = &*(f as *const F);
            f(&Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Track>,
        {
            let f: &F = &*(f as *const F);
            f(&Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mixing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mixing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Track>,
        {
            let f: &F = &*(f as *const F);
            f(&Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mixing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mixing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_restriction_caps_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Track>,
        {
            let f: &F = &*(f as *const F);
            f(&Track::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::restriction-caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_restriction_caps_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
