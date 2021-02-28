// This file was generated by gir (https://github.com/gtk-rs/gir @ a9fa404)
// from gir-files (https://github.com/gtk-rs/gir-files @ 6d6438b1)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use gio_sys as gio;
use glib_sys as glib;
use gstreamer_sys as gst;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Constants
pub const GST_NET_TIME_PACKET_SIZE: c_int = 16;
pub const GST_PTP_CLOCK_ID_NONE: u64 = 18446744073709551615;
pub const GST_PTP_STATISTICS_BEST_MASTER_CLOCK_SELECTED: *const c_char =
    b"GstPtpStatisticsBestMasterClockSelected\0" as *const u8 as *const c_char;
pub const GST_PTP_STATISTICS_NEW_DOMAIN_FOUND: *const c_char =
    b"GstPtpStatisticsNewDomainFound\0" as *const u8 as *const c_char;
pub const GST_PTP_STATISTICS_PATH_DELAY_MEASURED: *const c_char =
    b"GstPtpStatisticsPathDelayMeasured\0" as *const u8 as *const c_char;
pub const GST_PTP_STATISTICS_TIME_UPDATED: *const c_char =
    b"GstPtpStatisticsTimeUpdated\0" as *const u8 as *const c_char;

// Callbacks
pub type GstPtpStatisticsCallback =
    Option<unsafe extern "C" fn(u8, *const gst::GstStructure, gpointer) -> gboolean>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNetAddressMeta {
    pub meta: gst::GstMeta,
    pub addr: *mut gio::GSocketAddress,
}

impl ::std::fmt::Debug for GstNetAddressMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNetAddressMeta @ {:?}", self as *const _))
            .field("meta", &self.meta)
            .field("addr", &self.addr)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNetClientClockClass {
    pub parent_class: gst::GstSystemClockClass,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstNetClientClockClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNetClientClockClass @ {:?}", self as *const _))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct _GstNetClientClockPrivate(c_void);

pub type GstNetClientClockPrivate = *mut _GstNetClientClockPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNetControlMessageMeta {
    pub meta: gst::GstMeta,
    pub message: *mut gio::GSocketControlMessage,
}

impl ::std::fmt::Debug for GstNetControlMessageMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "GstNetControlMessageMeta @ {:?}",
            self as *const _
        ))
        .field("meta", &self.meta)
        .field("message", &self.message)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNetTimePacket {
    pub local_time: gst::GstClockTime,
    pub remote_time: gst::GstClockTime,
}

impl ::std::fmt::Debug for GstNetTimePacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNetTimePacket @ {:?}", self as *const _))
            .field("local_time", &self.local_time)
            .field("remote_time", &self.remote_time)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNetTimeProviderClass {
    pub parent_class: gst::GstObjectClass,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstNetTimeProviderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNetTimeProviderClass @ {:?}", self as *const _))
            .field("parent_class", &self.parent_class)
            .field("_gst_reserved", &self._gst_reserved)
            .finish()
    }
}

#[repr(C)]
pub struct _GstNetTimeProviderPrivate(c_void);

pub type GstNetTimeProviderPrivate = *mut _GstNetTimeProviderPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNtpClockClass {
    pub parent_class: gst::GstSystemClockClass,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstNtpClockClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNtpClockClass @ {:?}", self as *const _))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstPtpClockClass {
    pub parent_class: gst::GstSystemClockClass,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstPtpClockClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPtpClockClass @ {:?}", self as *const _))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
pub struct _GstPtpClockPrivate(c_void);

pub type GstPtpClockPrivate = *mut _GstPtpClockPrivate;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNetClientClock {
    pub clock: gst::GstSystemClock,
    pub priv_: *mut GstNetClientClockPrivate,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstNetClientClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNetClientClock @ {:?}", self as *const _))
            .field("clock", &self.clock)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNetTimeProvider {
    pub parent: gst::GstObject,
    pub priv_: *mut GstNetTimeProviderPrivate,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstNetTimeProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNetTimeProvider @ {:?}", self as *const _))
            .field("parent", &self.parent)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstNtpClock {
    pub clock: gst::GstSystemClock,
    pub priv_: *mut GstNetClientClockPrivate,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstNtpClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstNtpClock @ {:?}", self as *const _))
            .field("clock", &self.clock)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GstPtpClock {
    pub clock: gst::GstSystemClock,
    pub priv_: *mut GstPtpClockPrivate,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstPtpClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPtpClock @ {:?}", self as *const _))
            .field("clock", &self.clock)
            .finish()
    }
}

#[link(name = "gstnet-1.0")]
extern "C" {

    //=========================================================================
    // GstNetAddressMeta
    //=========================================================================
    pub fn gst_net_address_meta_get_info() -> *const gst::GstMetaInfo;

    //=========================================================================
    // GstNetControlMessageMeta
    //=========================================================================
    pub fn gst_net_control_message_meta_get_info() -> *const gst::GstMetaInfo;

    //=========================================================================
    // GstNetTimePacket
    //=========================================================================
    pub fn gst_net_time_packet_get_type() -> GType;
    pub fn gst_net_time_packet_new(buffer: *const u8) -> *mut GstNetTimePacket;
    pub fn gst_net_time_packet_copy(packet: *const GstNetTimePacket) -> *mut GstNetTimePacket;
    pub fn gst_net_time_packet_free(packet: *mut GstNetTimePacket);
    pub fn gst_net_time_packet_send(
        packet: *const GstNetTimePacket,
        socket: *mut gio::GSocket,
        dest_address: *mut gio::GSocketAddress,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn gst_net_time_packet_serialize(packet: *const GstNetTimePacket) -> *mut u8;
    pub fn gst_net_time_packet_receive(
        socket: *mut gio::GSocket,
        src_address: *mut *mut gio::GSocketAddress,
        error: *mut *mut glib::GError,
    ) -> *mut GstNetTimePacket;

    //=========================================================================
    // GstNetClientClock
    //=========================================================================
    pub fn gst_net_client_clock_get_type() -> GType;
    pub fn gst_net_client_clock_new(
        name: *const c_char,
        remote_address: *const c_char,
        remote_port: c_int,
        base_time: gst::GstClockTime,
    ) -> *mut gst::GstClock;

    //=========================================================================
    // GstNetTimeProvider
    //=========================================================================
    pub fn gst_net_time_provider_get_type() -> GType;
    pub fn gst_net_time_provider_new(
        clock: *mut gst::GstClock,
        address: *const c_char,
        port: c_int,
    ) -> *mut GstNetTimeProvider;

    //=========================================================================
    // GstNtpClock
    //=========================================================================
    pub fn gst_ntp_clock_get_type() -> GType;
    pub fn gst_ntp_clock_new(
        name: *const c_char,
        remote_address: *const c_char,
        remote_port: c_int,
        base_time: gst::GstClockTime,
    ) -> *mut gst::GstClock;

    //=========================================================================
    // GstPtpClock
    //=========================================================================
    pub fn gst_ptp_clock_get_type() -> GType;
    pub fn gst_ptp_clock_new(name: *const c_char, domain: c_uint) -> *mut gst::GstClock;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gst_buffer_add_net_address_meta(
        buffer: *mut gst::GstBuffer,
        addr: *mut gio::GSocketAddress,
    ) -> *mut GstNetAddressMeta;
    pub fn gst_buffer_add_net_control_message_meta(
        buffer: *mut gst::GstBuffer,
        message: *mut gio::GSocketControlMessage,
    ) -> *mut GstNetControlMessageMeta;
    pub fn gst_buffer_get_net_address_meta(buffer: *mut gst::GstBuffer) -> *mut GstNetAddressMeta;
    pub fn gst_net_address_meta_api_get_type() -> GType;
    pub fn gst_net_control_message_meta_api_get_type() -> GType;
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn gst_net_utils_set_socket_tos(socket: *mut gio::GSocket, qos_dscp: c_int) -> gboolean;
    pub fn gst_ptp_deinit();
    pub fn gst_ptp_init(clock_id: u64, interfaces: *mut *mut c_char) -> gboolean;
    pub fn gst_ptp_is_initialized() -> gboolean;
    pub fn gst_ptp_is_supported() -> gboolean;
    pub fn gst_ptp_statistics_callback_add(
        callback: GstPtpStatisticsCallback,
        user_data: gpointer,
        destroy_data: glib::GDestroyNotify,
    ) -> c_ulong;
    pub fn gst_ptp_statistics_callback_remove(id: c_ulong);

}
