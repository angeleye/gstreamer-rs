// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;

bitflags! {
    pub struct PipelineFlags: u32 {
        const AUDIO_PREVIEW = 1;
        const VIDEO_PREVIEW = 2;
        const FULL_PREVIEW = 3;
        const RENDER = 4;
        const SMART_RENDER = 8;
    }
}

#[doc(hidden)]
impl ToGlib for PipelineFlags {
    type GlibType = ffi::GESPipelineFlags;

    fn to_glib(&self) -> ffi::GESPipelineFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESPipelineFlags> for PipelineFlags {
    fn from_glib(value: ffi::GESPipelineFlags) -> PipelineFlags {
        skip_assert_initialized!();
        PipelineFlags::from_bits_truncate(value)
    }
}

impl StaticType for PipelineFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_pipeline_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PipelineFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PipelineFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PipelineFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct TrackType: u32 {
        const UNKNOWN = 1;
        const AUDIO = 2;
        const VIDEO = 4;
        const TEXT = 8;
        const CUSTOM = 16;
    }
}

#[doc(hidden)]
impl ToGlib for TrackType {
    type GlibType = ffi::GESTrackType;

    fn to_glib(&self) -> ffi::GESTrackType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESTrackType> for TrackType {
    fn from_glib(value: ffi::GESTrackType) -> TrackType {
        skip_assert_initialized!();
        TrackType::from_bits_truncate(value)
    }
}

impl StaticType for TrackType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_track_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TrackType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TrackType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for TrackType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
