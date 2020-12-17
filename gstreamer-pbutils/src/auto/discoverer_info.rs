// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DiscovererResult;
use crate::DiscovererSerializeFlags;
use crate::DiscovererStreamInfo;
use glib::translate::*;

glib::wrapper! {
    pub struct DiscovererInfo(Object<ffi::GstDiscovererInfo>);

    match fn {
        get_type => || ffi::gst_discoverer_info_get_type(),
    }
}

impl DiscovererInfo {
    #[doc(alias = "gst_discoverer_info_copy")]
    pub fn copy(&self) -> DiscovererInfo {
        unsafe { from_glib_full(ffi::gst_discoverer_info_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_audio_streams")]
    pub fn get_audio_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_audio_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_container_streams")]
    pub fn get_container_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_container_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_duration")]
    pub fn get_duration(&self) -> gst::ClockTime {
        unsafe { from_glib(ffi::gst_discoverer_info_get_duration(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_discoverer_info_get_live")]
    pub fn get_live(&self) -> bool {
        unsafe { from_glib(ffi::gst_discoverer_info_get_live(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_misc")]
    pub fn get_misc(&self) -> Option<gst::Structure> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_misc(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_missing_elements_installer_details")]
    pub fn get_missing_elements_installer_details(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::gst_discoverer_info_get_missing_elements_installer_details(
                    self.to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "gst_discoverer_info_get_result")]
    pub fn get_result(&self) -> DiscovererResult {
        unsafe { from_glib(ffi::gst_discoverer_info_get_result(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_seekable")]
    pub fn get_seekable(&self) -> bool {
        unsafe { from_glib(ffi::gst_discoverer_info_get_seekable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_stream_info")]
    pub fn get_stream_info(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_info_get_stream_info(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_stream_list")]
    pub fn get_stream_list(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_stream_list(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_streams")]
    pub fn get_streams(&self, streamtype: glib::types::Type) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_streams(
                self.to_glib_none().0,
                streamtype.to_glib(),
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_subtitle_streams")]
    pub fn get_subtitle_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_subtitle_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_get_tags")]
    pub fn get_tags(&self) -> Option<gst::TagList> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_tags(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_toc")]
    pub fn get_toc(&self) -> Option<gst::Toc> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_toc(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_uri")]
    pub fn get_uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gst_discoverer_info_get_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_discoverer_info_get_video_streams")]
    pub fn get_video_streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_info_get_video_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_info_to_variant")]
    pub fn to_variant(
        &self,
        flags: DiscovererSerializeFlags,
    ) -> Result<glib::Variant, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_discoverer_info_to_variant(
                self.to_glib_none().0,
                flags.to_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to serialize DiscovererInfo to Variant"))
        }
    }

    #[doc(alias = "gst_discoverer_info_from_variant")]
    pub fn from_variant(variant: &glib::Variant) -> Result<DiscovererInfo, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_discoverer_info_from_variant(
                variant.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to deserialize DiscovererInfo from Variant"))
        }
    }
}

unsafe impl Send for DiscovererInfo {}
unsafe impl Sync for DiscovererInfo {}
