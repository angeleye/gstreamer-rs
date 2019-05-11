// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use gst_video_sys;

bitflags! {
    pub struct VideoChromaSite: u32 {
        const UNKNOWN = 0;
        const NONE = 1;
        const H_COSITED = 2;
        const V_COSITED = 4;
        const ALT_LINE = 8;
        const COSITED = 6;
        const JPEG = 1;
        const MPEG2 = 2;
        const DV = 14;
    }
}

#[doc(hidden)]
impl ToGlib for VideoChromaSite {
    type GlibType = gst_video_sys::GstVideoChromaSite;

    fn to_glib(&self) -> gst_video_sys::GstVideoChromaSite {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoChromaSite> for VideoChromaSite {
    fn from_glib(value: gst_video_sys::GstVideoChromaSite) -> VideoChromaSite {
        skip_assert_initialized!();
        VideoChromaSite::from_bits_truncate(value)
    }
}

impl StaticType for VideoChromaSite {
    fn static_type() -> Type {
        unsafe { from_glib(gst_video_sys::gst_video_chroma_site_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoChromaSite {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoChromaSite {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for VideoChromaSite {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct VideoCodecFrameFlags: u32 {
        const DECODE_ONLY = 1;
        const SYNC_POINT = 2;
        const FORCE_KEYFRAME = 4;
        const FORCE_KEYFRAME_HEADERS = 8;
    }
}

#[doc(hidden)]
impl ToGlib for VideoCodecFrameFlags {
    type GlibType = gst_video_sys::GstVideoCodecFrameFlags;

    fn to_glib(&self) -> gst_video_sys::GstVideoCodecFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoCodecFrameFlags> for VideoCodecFrameFlags {
    fn from_glib(value: gst_video_sys::GstVideoCodecFrameFlags) -> VideoCodecFrameFlags {
        skip_assert_initialized!();
        VideoCodecFrameFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct VideoFlags: u32 {
        const NONE = 0;
        const VARIABLE_FPS = 1;
        const PREMULTIPLIED_ALPHA = 2;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFlags {
    type GlibType = gst_video_sys::GstVideoFlags;

    fn to_glib(&self) -> gst_video_sys::GstVideoFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoFlags> for VideoFlags {
    fn from_glib(value: gst_video_sys::GstVideoFlags) -> VideoFlags {
        skip_assert_initialized!();
        VideoFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_video_sys::gst_video_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for VideoFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct VideoFormatFlags: u32 {
        const YUV = 1;
        const RGB = 2;
        const GRAY = 4;
        const ALPHA = 8;
        const LE = 16;
        const PALETTE = 32;
        const COMPLEX = 64;
        const UNPACK = 128;
        const TILED = 256;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFormatFlags {
    type GlibType = gst_video_sys::GstVideoFormatFlags;

    fn to_glib(&self) -> gst_video_sys::GstVideoFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoFormatFlags> for VideoFormatFlags {
    fn from_glib(value: gst_video_sys::GstVideoFormatFlags) -> VideoFormatFlags {
        skip_assert_initialized!();
        VideoFormatFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_video_sys::gst_video_format_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoFormatFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoFormatFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for VideoFormatFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct VideoFrameFlags: u32 {
        const NONE = 0;
        const INTERLACED = 1;
        const TFF = 2;
        const RFF = 4;
        const ONEFIELD = 8;
        const MULTIPLE_VIEW = 16;
        const FIRST_IN_BUNDLE = 32;
        const TOP_FIELD = 10;
        const BOTTOM_FIELD = 8;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFrameFlags {
    type GlibType = gst_video_sys::GstVideoFrameFlags;

    fn to_glib(&self) -> gst_video_sys::GstVideoFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoFrameFlags> for VideoFrameFlags {
    fn from_glib(value: gst_video_sys::GstVideoFrameFlags) -> VideoFrameFlags {
        skip_assert_initialized!();
        VideoFrameFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoFrameFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_video_sys::gst_video_frame_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoFrameFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoFrameFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for VideoFrameFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct VideoMultiviewFlags: u32 {
        const NONE = 0;
        const RIGHT_VIEW_FIRST = 1;
        const LEFT_FLIPPED = 2;
        const LEFT_FLOPPED = 4;
        const RIGHT_FLIPPED = 8;
        const RIGHT_FLOPPED = 16;
        const HALF_ASPECT = 16384;
        const MIXED_MONO = 32768;
    }
}

#[doc(hidden)]
impl ToGlib for VideoMultiviewFlags {
    type GlibType = gst_video_sys::GstVideoMultiviewFlags;

    fn to_glib(&self) -> gst_video_sys::GstVideoMultiviewFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoMultiviewFlags> for VideoMultiviewFlags {
    fn from_glib(value: gst_video_sys::GstVideoMultiviewFlags) -> VideoMultiviewFlags {
        skip_assert_initialized!();
        VideoMultiviewFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoMultiviewFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_video_sys::gst_video_multiview_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoMultiviewFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoMultiviewFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for VideoMultiviewFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct VideoOverlayFormatFlags: u32 {
        const NONE = 0;
        const PREMULTIPLIED_ALPHA = 1;
        const GLOBAL_ALPHA = 2;
    }
}

#[doc(hidden)]
impl ToGlib for VideoOverlayFormatFlags {
    type GlibType = gst_video_sys::GstVideoOverlayFormatFlags;

    fn to_glib(&self) -> gst_video_sys::GstVideoOverlayFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoOverlayFormatFlags> for VideoOverlayFormatFlags {
    fn from_glib(value: gst_video_sys::GstVideoOverlayFormatFlags) -> VideoOverlayFormatFlags {
        skip_assert_initialized!();
        VideoOverlayFormatFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoOverlayFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_video_sys::gst_video_overlay_format_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoOverlayFormatFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoOverlayFormatFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for VideoOverlayFormatFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
bitflags! {
    pub struct VideoTimeCodeFlags: u32 {
        const NONE = 0;
        const DROP_FRAME = 1;
        const INTERLACED = 2;
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for VideoTimeCodeFlags {
    type GlibType = gst_video_sys::GstVideoTimeCodeFlags;

    fn to_glib(&self) -> gst_video_sys::GstVideoTimeCodeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gst_video_sys::GstVideoTimeCodeFlags> for VideoTimeCodeFlags {
    fn from_glib(value: gst_video_sys::GstVideoTimeCodeFlags) -> VideoTimeCodeFlags {
        skip_assert_initialized!();
        VideoTimeCodeFlags::from_bits_truncate(value)
    }
}

