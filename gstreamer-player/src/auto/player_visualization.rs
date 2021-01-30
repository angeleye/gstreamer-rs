// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlayerVisualization(Boxed<ffi::GstPlayerVisualization>);

    match fn {
        copy => |ptr| ffi::gst_player_visualization_copy(ptr),
        free => |ptr| ffi::gst_player_visualization_free(ptr),
        get_type => || ffi::gst_player_visualization_get_type(),
    }
}

unsafe impl Send for PlayerVisualization {}
unsafe impl Sync for PlayerVisualization {}
