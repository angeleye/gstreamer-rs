// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod allocator;
pub use self::allocator::Allocator;

mod bin;
pub use self::bin::Bin;

mod buffer_pool;
pub use self::buffer_pool::BufferPool;

mod bus;
pub use self::bus::Bus;

mod child_proxy;
pub use self::child_proxy::ChildProxy;

mod clock;
pub use self::clock::Clock;

mod control_binding;
pub use self::control_binding::ControlBinding;

mod control_source;
pub use self::control_source::ControlSource;

mod device;
pub use self::device::Device;

mod device_monitor;
pub use self::device_monitor::DeviceMonitor;

mod device_provider;
pub use self::device_provider::DeviceProvider;

mod device_provider_factory;
pub use self::device_provider_factory::DeviceProviderFactory;

mod element;
pub use self::element::Element;

mod element_factory;
pub use self::element_factory::ElementFactory;

mod ghost_pad;
pub use self::ghost_pad::GhostPad;

mod object;
pub use self::object::Object;

mod pad;
pub use self::pad::Pad;

mod pad_template;
pub use self::pad_template::PadTemplate;

mod pipeline;
pub use self::pipeline::Pipeline;

mod plugin;
pub use self::plugin::Plugin;

mod plugin_feature;
pub use self::plugin_feature::PluginFeature;

mod preset;
pub use self::preset::Preset;

mod proxy_pad;
pub use self::proxy_pad::ProxyPad;

mod registry;
pub use self::registry::Registry;

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
mod stream;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::stream::Stream;

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
mod stream_collection;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::stream_collection::StreamCollection;

mod system_clock;
pub use self::system_clock::SystemClock;

mod tag_setter;
pub use self::tag_setter::TagSetter;

mod toc_setter;
pub use self::toc_setter::TocSetter;

mod tracer;
pub use self::tracer::Tracer;

mod tracer_factory;
pub use self::tracer_factory::TracerFactory;

mod type_find_factory;
pub use self::type_find_factory::TypeFindFactory;

mod uri_handler;
pub use self::uri_handler::URIHandler;

mod date_time;
pub use self::date_time::DateTime;

mod enums;
pub use self::enums::BufferingMode;
pub use self::enums::BusSyncReply;
pub use self::enums::CapsIntersectMode;
pub use self::enums::ClockEntryType;
pub use self::enums::ClockReturn;
pub use self::enums::ClockType;
pub use self::enums::CoreError;
pub use self::enums::DebugLevel;
pub use self::enums::EventType;
pub use self::enums::FlowReturn;
pub use self::enums::Format;
pub use self::enums::LibraryError;
pub use self::enums::PadDirection;
pub use self::enums::PadLinkReturn;
pub use self::enums::PadMode;
pub use self::enums::PadPresence;
pub use self::enums::PadProbeReturn;
pub use self::enums::ParseError;
pub use self::enums::PluginError;
pub use self::enums::ProgressType;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
pub use self::enums::PromiseResult;
pub use self::enums::QOSType;
pub use self::enums::Rank;
pub use self::enums::ResourceError;
pub use self::enums::SeekType;
pub use self::enums::State;
pub use self::enums::StateChange;
pub use self::enums::StateChangeReturn;
pub use self::enums::StreamError;
pub use self::enums::StreamStatusType;
pub use self::enums::StructureChangeType;
pub use self::enums::TagFlag;
pub use self::enums::TagMergeMode;
pub use self::enums::TagScope;
pub use self::enums::TaskState;
pub use self::enums::TocEntryType;
pub use self::enums::TocLoopType;
pub use self::enums::TocScope;
pub use self::enums::TypeFindProbability;
pub use self::enums::URIError;
pub use self::enums::URIType;

mod flags;
pub use self::flags::BinFlags;
pub use self::flags::BufferCopyFlags;
pub use self::flags::BufferFlags;
pub use self::flags::BufferPoolAcquireFlags;
pub use self::flags::ClockFlags;
pub use self::flags::DebugColorFlags;
pub use self::flags::DebugGraphDetails;
pub use self::flags::ElementFlags;
pub use self::flags::EventTypeFlags;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
pub use self::flags::GapFlags;
pub use self::flags::MemoryFlags;
pub use self::flags::MetaFlags;
pub use self::flags::ObjectFlags;
pub use self::flags::PadFlags;
pub use self::flags::PadLinkCheck;
pub use self::flags::PadProbeType;
pub use self::flags::ParseFlags;
pub use self::flags::PipelineFlags;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
pub use self::flags::PluginAPIFlags;
pub use self::flags::PluginDependencyFlags;
pub use self::flags::PluginFlags;
pub use self::flags::SchedulingFlags;
pub use self::flags::SeekFlags;
pub use self::flags::SegmentFlags;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
pub use self::flags::SerializeFlags;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
pub use self::flags::StackTraceFlags;
pub use self::flags::StreamFlags;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub use self::flags::StreamType;

mod alias;
pub use self::alias::ClockTimeDiff;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::allocator::AllocatorExt;
    pub use super::bin::GstBinExt;
    pub use super::buffer_pool::BufferPoolExt;
    pub use super::child_proxy::ChildProxyExt;
    pub use super::clock::ClockExt;
    pub use super::control_binding::ControlBindingExt;
    pub use super::control_source::ControlSourceExt;
    pub use super::device::DeviceExt;
    pub use super::device_monitor::DeviceMonitorExt;
    pub use super::device_provider::DeviceProviderExt;
    pub use super::element::ElementExt;
    pub use super::ghost_pad::GhostPadExt;
    pub use super::object::GstObjectExt;
    pub use super::pad::PadExt;
    pub use super::pipeline::PipelineExt;
    pub use super::plugin_feature::PluginFeatureExt;
    pub use super::preset::PresetExt;
    pub use super::proxy_pad::ProxyPadExt;
    pub use super::system_clock::SystemClockExt;
    pub use super::tag_setter::TagSetterExt;
    pub use super::toc_setter::TocSetterExt;
    pub use super::tracer::TracerExt;
    pub use super::uri_handler::URIHandlerExt;
}
