// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Edge;
use crate::EditMode;
use crate::Extractable;
use crate::Layer;
use crate::TimelineElement;
use crate::Track;
use crate::TrackType;
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
    pub struct TrackElement(Object<ffi::GESTrackElement, ffi::GESTrackElementClass>) @extends TimelineElement, @implements Extractable;

    match fn {
        get_type => || ffi::ges_track_element_get_type(),
    }
}

pub const NONE_TRACK_ELEMENT: Option<&TrackElement> = None;

pub trait TrackElementExt: 'static {
    fn add_children_props<P: IsA<gst::Element>>(
        &self,
        element: &P,
        wanted_categories: &[&str],
        blacklist: &[&str],
        whitelist: &[&str],
    );

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn clamp_control_source(&self, property_name: &str);

    #[cfg_attr(feature = "v1_18", deprecated)]
    fn edit(
        &self,
        layers: &[Layer],
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::error::BoolError>;

    //fn get_all_control_bindings(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 6, id: 83 };

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_auto_clamp_control_sources(&self) -> bool;

    //fn get_control_binding(&self, property_name: &str) -> /*Ignored*/Option<gst::ControlBinding>;

    fn get_element(&self) -> Option<gst::Element>;

    fn get_gnlobject(&self) -> Option<gst::Element>;

    fn get_nleobject(&self) -> Option<gst::Element>;

    fn get_track(&self) -> Option<Track>;

    fn get_track_type(&self) -> TrackType;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn has_internal_source(&self) -> bool;

    fn is_active(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_core(&self) -> bool;

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<gst::Element>;

    fn remove_control_binding(&self, property_name: &str) -> Result<(), glib::error::BoolError>;

    fn set_active(&self, active: bool) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_auto_clamp_control_sources(&self, auto_clamp: bool);

    //fn set_control_source(&self, source: /*Ignored*/&gst::ControlSource, property_name: &str, binding_type: &str) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_has_internal_source(&self, has_internal_source: bool) -> bool;

    fn set_track_type(&self, type_: TrackType);

    fn get_property_active(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_property_has_internal_source(&self) -> bool;

    //fn connect_control_binding_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //fn connect_control_binding_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_auto_clamp_control_sources_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_has_internal_source_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TrackElement>> TrackElementExt for O {
    fn add_children_props<P: IsA<gst::Element>>(
        &self,
        element: &P,
        wanted_categories: &[&str],
        blacklist: &[&str],
        whitelist: &[&str],
    ) {
        unsafe {
            ffi::ges_track_element_add_children_props(
                self.as_ref().to_glib_none().0,
                element.as_ref().to_glib_none().0,
                wanted_categories.to_glib_none().0,
                blacklist.to_glib_none().0,
                whitelist.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn clamp_control_source(&self, property_name: &str) {
        unsafe {
            ffi::ges_track_element_clamp_control_source(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

    fn edit(
        &self,
        layers: &[Layer],
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_track_element_edit(
                    self.as_ref().to_glib_none().0,
                    layers.to_glib_none().0,
                    mode.to_glib(),
                    edge.to_glib(),
                    position
                ),
                "Failed to edit"
            )
        }
    }

    //fn get_all_control_bindings(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 6, id: 83 } {
    //    unsafe { TODO: call ffi:ges_track_element_get_all_control_bindings() }
    //}

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_auto_clamp_control_sources(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_get_auto_clamp_control_sources(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_control_binding(&self, property_name: &str) -> /*Ignored*/Option<gst::ControlBinding> {
    //    unsafe { TODO: call ffi:ges_track_element_get_control_binding() }
    //}

    fn get_element(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_gnlobject(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_gnlobject(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_nleobject(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_nleobject(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_track(&self) -> Option<Track> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_track(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_track_type(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_track_element_get_track_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn has_internal_source(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_has_internal_source(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_is_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_core(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_is_core(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<gst::Element> {
    //    unsafe { TODO: call ffi:ges_track_element_lookup_child() }
    //}

    fn remove_control_binding(&self, property_name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::ges_track_element_remove_control_binding(
                    self.as_ref().to_glib_none().0,
                    property_name.to_glib_none().0
                ),
                "Failed to remove control binding"
            )
        }
    }

    fn set_active(&self, active: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_set_active(
                self.as_ref().to_glib_none().0,
                active.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_auto_clamp_control_sources(&self, auto_clamp: bool) {
        unsafe {
            ffi::ges_track_element_set_auto_clamp_control_sources(
                self.as_ref().to_glib_none().0,
                auto_clamp.to_glib(),
            );
        }
    }

    //fn set_control_source(&self, source: /*Ignored*/&gst::ControlSource, property_name: &str, binding_type: &str) -> bool {
    //    unsafe { TODO: call ffi:ges_track_element_set_control_source() }
    //}

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_has_internal_source(&self, has_internal_source: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_set_has_internal_source(
                self.as_ref().to_glib_none().0,
                has_internal_source.to_glib(),
            ))
        }
    }

    fn set_track_type(&self, type_: TrackType) {
        unsafe {
            ffi::ges_track_element_set_track_type(self.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"active\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `active` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_property_has_internal_source(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"has-internal-source\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-internal-source` getter")
                .unwrap()
        }
    }

    //fn connect_control_binding_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored control_binding: Gst.ControlBinding
    //}

    //fn connect_control_binding_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored control_binding: Gst.ControlBinding
    //}

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TrackElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_auto_clamp_control_sources_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_clamp_control_sources_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TrackElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-clamp-control-sources\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_clamp_control_sources_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_has_internal_source_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_internal_source_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TrackElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-internal-source\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_internal_source_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TrackElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::track\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_track_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TrackElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::track-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_track_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
