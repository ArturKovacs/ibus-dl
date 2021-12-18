
use std::ffi::c_void;

use dlib::dlopen_external_library;

pub type gint8 = ::std::os::raw::c_schar;
pub type guint8 = ::std::os::raw::c_uchar;
pub type guint16 = ::std::os::raw::c_ushort;
pub type gint32 = ::std::os::raw::c_int;
pub type guint32 = ::std::os::raw::c_uint;
pub type gsize = ::std::os::raw::c_ulong;
pub type gchar = ::std::os::raw::c_char;
pub type glong = ::std::os::raw::c_long;
pub type gint = ::std::os::raw::c_int;
pub type gboolean = gint;
pub type gulong = ::std::os::raw::c_ulong;
pub type guint = ::std::os::raw::c_uint;
pub type gdouble = f64;
pub type gpointer = *mut ::std::os::raw::c_void;
pub type GQuark = guint32;
pub type GType = gsize;

pub const IBUS_CAP_PREEDIT_TEXT: guint32 = 1 << 0;
pub const IBUS_CAP_AUXILIARY_TEXT: guint32 = 1 << 1;
pub const IBUS_CAP_LOOKUP_TABLE: guint32 = 1 << 2;
pub const IBUS_CAP_FOCUS: guint32 = 1 << 3;
pub const IBUS_CAP_PROPERTY: guint32 = 1 << 4;
pub const IBUS_CAP_SURROUNDING_TEXT: guint32 = 1 << 5;

#[doc = " gunichar:"]
#[doc = ""]
#[doc = " A type which can hold any UTF-32 or UCS-4 character code,"]
#[doc = " also known as a Unicode code point."]
#[doc = ""]
#[doc = " If you want to produce the UTF-8 representation of a #gunichar,"]
#[doc = " use g_ucs4_to_utf8(). See also g_utf8_to_ucs4() for the reverse"]
#[doc = " process."]
#[doc = ""]
#[doc = " To print/scan values of this type as integer, use"]
#[doc = " %G_GINT32_MODIFIER and/or %G_GUINT32_FORMAT."]
#[doc = ""]
#[doc = " The notation to express a Unicode code point in running text is"]
#[doc = " as a hexadecimal number with four to six digits and uppercase"]
#[doc = " letters, prefixed by the string \"U+\". Leading zeros are omitted,"]
#[doc = " unless the code point would have fewer than four hexadecimal digits."]
#[doc = " For example, \"U+0041 LATIN CAPITAL LETTER A\". To print a code point"]
#[doc = " in the U+-notation, use the format string \"U+\\%04\"G_GINT32_FORMAT\"X\"."]
#[doc = " To scan, use the format string \"U+\\%06\"G_GINT32_FORMAT\"X\"."]
#[doc = ""]
#[doc = " ```"]
#[doc = " gunichar c;"]
#[doc = " sscanf (\"U+0041\", \"U+%06\"G_GINT32_FORMAT\"X\", &amp;c)"]
#[doc = " g_print (\"Read U+%04\"G_GINT32_FORMAT\"X\", c);"]
#[doc = " ```"]
pub type gunichar = guint32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}

#[doc = " IBusBus:"]
#[doc = ""]
#[doc = " An opaque data type representing IBus bus (daemon communication) status."]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct IBusBus {
    pub _bindgen_opaque_blob: [u64; 6usize],
}

#[doc = " IBusInputContext:"]
#[doc = ""]
#[doc = " An opaque data type representing an IBusInputContext."]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct IBusInputContext {
    pub _bindgen_opaque_blob: [u64; 5usize],
}

#[doc = " IBusEngineDesc:"]
#[doc = ""]
#[doc = " Input method engine description data."]
#[doc = " You can get extended values with g_object_get_properties."]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct IBusEngineDesc {
    pub _bindgen_opaque_blob: [u64; 7usize],
}

#[doc = " @text: The string content of IBusText in UTF-8."]
#[doc = " @attrs: Associated IBusAttributes."]
#[doc = ""]
#[doc = " A text object in IBus."]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct IBusText {
    pub _bindgen_opaque_blob: [u64; 9usize],
}

#[doc = " IBusAttrList:"]
#[doc = " @attributes: GArray that holds #IBusAttribute."]
#[doc = ""]
#[doc = " Array of IBusAttribute."]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct IBusAttrList {
    pub _bindgen_opaque_blob: [u64; 7usize],
}

#[doc = " IBusAttribute:"]
#[doc = " @type: IBusAttributeType"]
#[doc = " @value: Value for the type."]
#[doc = " @start_index: The starting index, inclusive."]
#[doc = " @end_index: The ending index, exclusive."]
#[doc = ""]
#[doc = " Signify the type, value and scope of the attribute."]
#[doc = " The scope starts from @start_index till the @end_index-1."]
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct IBusAttribute {
    pub _bindgen_opaque_blob: [u64; 8usize],
}

dlopen_external_library!(
    IBus,
functions:
    // -----------------------------------------------
    // IBusBus
    // -----------------------------------------------
    fn ibus_bus_get_type() -> GType,
    fn ibus_bus_new() -> *mut IBusBus,
    fn ibus_bus_is_connected(*mut IBusBus) -> gboolean,
    fn ibus_bus_hello(*mut IBusBus) -> *const gchar,
    fn ibus_bus_request_name(*mut IBusBus, *const gchar, guint32) -> guint32,
    fn ibus_bus_release_name(*mut IBusBus, *const gchar) -> guint,
    fn ibus_bus_name_has_owner(*mut IBusBus, *const gchar) -> gboolean,
    fn ibus_bus_list_names(*mut IBusBus) -> *mut GList,
    fn ibus_bus_add_match(*mut IBusBus, *const gchar) -> gboolean,
    fn ibus_bus_remove_match(*mut IBusBus, *const gchar) -> gboolean,
    fn ibus_bus_get_name_owner(*mut IBusBus, *const gchar) -> *mut gchar,
    fn ibus_bus_exit(*mut IBusBus, gboolean) -> gboolean,
    fn ibus_bus_create_input_context(*mut IBusBus, *const gchar) -> *mut IBusInputContext,
    fn ibus_bus_current_input_context(*mut IBusBus) -> *mut gchar,
    // fn ibus_bus_register_component(*mut IBusBus, *mut IBusComponent) -> gboolean,
    fn ibus_bus_list_engines(*mut IBusBus) -> *mut GList,
    fn ibus_bus_get_use_sys_layout(*mut IBusBus) -> gboolean,
    fn ibus_bus_get_use_global_engine(*mut IBusBus) -> gboolean,
    fn ibus_bus_get_global_engine(*mut IBusBus) -> *mut IBusEngineDesc,
    fn ibus_bus_is_global_engine_enabled(*mut IBusBus) -> gboolean,
    fn ibus_bus_set_global_engine(*mut IBusBus, *const gchar) -> gboolean,

    // -----------------------------------------------
    // IBusInputContext 
    // -----------------------------------------------
    fn ibus_input_context_get_type() -> GType,
    fn ibus_input_context_process_key_event(*mut IBusInputContext, guint32, guint32, guint32) -> gboolean,
    fn ibus_input_context_set_cursor_location(*mut IBusInputContext, gint32, gint32, gint32, gint32) -> (),
    // fn ibus_input_context_set_cursor_location_relative(*mut IBusInputContext, gint32, gint32, gint32, gint32) -> (),
    fn ibus_input_context_set_capabilities(*mut IBusInputContext, guint32) -> (),
    fn ibus_input_context_focus_in(*mut IBusInputContext) -> (),
    fn ibus_input_context_focus_out(*mut IBusInputContext) -> (),
    fn ibus_input_context_reset(*mut IBusInputContext) -> (),
    fn ibus_input_context_get_engine(*mut IBusInputContext) -> *mut IBusEngineDesc,
    fn ibus_input_context_set_engine(*mut IBusInputContext, *const gchar) -> (),
    fn ibus_input_context_set_surrounding_text(*mut IBusInputContext, *mut IBusText, guint32, guint32) -> (),
    fn ibus_input_context_needs_surrounding_text(*mut IBusInputContext) -> gboolean,

    // -----------------------------------------------
    // IBusEngineDesc
    // -----------------------------------------------
    fn ibus_engine_desc_get_type() -> GType,
    fn ibus_engine_desc_new(*const gchar, *const gchar, *const gchar, *const gchar, *const gchar, *const gchar, *const gchar, *const gchar) -> *mut IBusEngineDesc,
    // fn ibus_engine_desc_new_from_xml_node(*mut IBusXML) -> *mut IBusEngineDesc,
    fn ibus_engine_desc_get_name(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_longname(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_description(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_language(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_license(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_author(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_icon(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_layout(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_layout_variant(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_layout_option(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_rank(*mut IBusEngineDesc) -> guint,
    fn ibus_engine_desc_get_hotkeys(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_symbol(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_setup(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_version(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_textdomain(*mut IBusEngineDesc) -> *const gchar,
    fn ibus_engine_desc_get_icon_prop_key(*mut IBusEngineDesc) -> *const gchar,

    // The second parameter of this should be GString, but that's implemented in
    // GLib and I don't want to reimplement it here
    fn ibus_engine_desc_output(*mut IBusEngineDesc, *mut c_void, gint) -> (),

    // -----------------------------------------------
    // IBusText
    // -----------------------------------------------
    fn ibus_text_get_type() -> GType,
    fn ibus_text_new_from_string(*const gchar) -> *mut IBusText,
    fn ibus_text_new_from_ucs4(*const gunichar) -> *mut IBusText,
    fn ibus_text_new_from_static_string(*const gchar) -> *mut IBusText,
    fn ibus_text_new_from_unichar(gunichar) -> *mut IBusText,
    fn ibus_text_get_length(*mut IBusText) -> guint,
    fn ibus_text_get_is_static(*mut IBusText) -> gboolean,
    fn ibus_text_get_text(*mut IBusText) -> *const gchar,
    fn ibus_text_get_attributes(*mut IBusText) -> *mut IBusAttrList,
    fn ibus_text_set_attributes(*mut IBusText, *mut IBusAttrList) -> (),

    // -----------------------------------------------
    // IBusAttrList
    // -----------------------------------------------
    fn ibus_attr_list_get_type() -> GType,
    fn ibus_attr_list_new() -> *mut IBusAttrList,
    fn ibus_attr_list_append(*mut IBusAttrList, *mut IBusAttribute) -> (),
    fn ibus_attr_list_get(*mut IBusAttrList, guint) -> *mut IBusAttribute,

    // -----------------------------------------------
    // IBusAttribute
    // -----------------------------------------------
    fn ibus_attribute_get_type() -> GType,
    fn ibus_attribute_new(guint,guint,guint,guint) -> *mut IBusAttribute,
    fn ibus_attribute_get_attr_type(*mut IBusAttribute) -> guint,
    fn ibus_attribute_get_value(*mut IBusAttribute) -> guint,
    fn ibus_attribute_get_start_index(*mut IBusAttribute) -> guint,
    fn ibus_attribute_get_end_index(*mut IBusAttribute) -> guint,
    fn ibus_attr_underline_new(guint, guint, guint) -> *mut IBusAttribute,
    fn ibus_attr_foreground_new(guint, guint, guint) -> *mut IBusAttribute,
    fn ibus_attr_background_new(guint, guint, guint) -> *mut IBusAttribute,
);
