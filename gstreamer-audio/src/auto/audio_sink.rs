// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AudioBaseSink;

glib::wrapper! {
    pub struct AudioSink(Object<ffi::GstAudioSink, ffi::GstAudioSinkClass>) @extends AudioBaseSink, gst_base::BaseSink, gst::Element, gst::Object;

    match fn {
        get_type => || ffi::gst_audio_sink_get_type(),
    }
}

impl AudioSink {}

unsafe impl Send for AudioSink {}
unsafe impl Sync for AudioSink {}

pub const NONE_AUDIO_SINK: Option<&AudioSink> = None;
