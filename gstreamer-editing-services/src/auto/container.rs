// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Edge;
use crate::EditMode;
use crate::Extractable;
use crate::Layer;
use crate::TimelineElement;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct Container(Object<ffi::GESContainer, ffi::GESContainerClass>) @extends TimelineElement, @implements Extractable;

    match fn {
        get_type => || ffi::ges_container_get_type(),
    }
}

impl Container {
    #[doc(alias = "ges_container_group")]
    pub fn group(containers: &[Container]) -> Option<Container> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_container_group(containers.to_glib_none().0)) }
    }
}

pub const NONE_CONTAINER: Option<&Container> = None;

pub trait GESContainerExt: 'static {
    #[doc(alias = "ges_container_add")]
    fn add<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError>;

    #[cfg_attr(feature = "v1_18", deprecated)]
    #[doc(alias = "ges_container_edit")]
    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i32,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_container_get_children")]
    fn get_children(&self, recursive: bool) -> Vec<TimelineElement>;

    #[doc(alias = "ges_container_remove")]
    fn remove<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_container_ungroup")]
    fn ungroup(&self, recursive: bool) -> Vec<Container>;

    fn get_property_height(&self) -> u32;

    fn connect_child_added<F: Fn(&Self, &TimelineElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_child_removed<F: Fn(&Self, &TimelineElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Container>> GESContainerExt for O {
    fn add<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_container_add(
                    self.as_ref().to_glib_none().0,
                    child.as_ref().to_glib_none().0
                ),
                "Failed to add element"
            )
        }
    }

    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i32,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_container_edit(
                    self.as_ref().to_glib_none().0,
                    layers.to_glib_none().0,
                    new_layer_priority,
                    mode.to_glib(),
                    edge.to_glib(),
                    position
                ),
                "Failed to edit container"
            )
        }
    }

    fn get_children(&self, recursive: bool) -> Vec<TimelineElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_container_get_children(
                self.as_ref().to_glib_none().0,
                recursive.to_glib(),
            ))
        }
    }

    fn remove<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_container_remove(
                    self.as_ref().to_glib_none().0,
                    child.as_ref().to_glib_none().0
                ),
                "Failed to remove element"
            )
        }
    }

    fn ungroup(&self, recursive: bool) -> Vec<Container> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_container_ungroup(
                self.as_ref().to_glib_full(),
                recursive.to_glib(),
            ))
        }
    }

    fn get_property_height(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `height` getter")
                .unwrap()
        }
    }

    fn connect_child_added<F: Fn(&Self, &TimelineElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_added_trampoline<P, F: Fn(&P, &TimelineElement) + 'static>(
            this: *mut ffi::GESContainer,
            element: *mut ffi::GESTimelineElement,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Container>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_child_removed<F: Fn(&Self, &TimelineElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_removed_trampoline<P, F: Fn(&P, &TimelineElement) + 'static>(
            this: *mut ffi::GESContainer,
            element: *mut ffi::GESTimelineElement,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Container>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESContainer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Container>,
        {
            let f: &F = &*(f as *const F);
            f(&Container::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
