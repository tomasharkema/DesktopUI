// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GConverter")]
    pub struct Converter(Interface<ffi::GConverter, ffi::GConverterIface>);

    match fn {
        type_ => || ffi::g_converter_get_type(),
    }
}

pub const NONE_CONVERTER: Option<&Converter> = None;

pub trait ConverterExt: 'static {
    #[doc(alias = "g_converter_reset")]
    fn reset(&self);
}

impl<O: IsA<Converter>> ConverterExt for O {
    fn reset(&self) {
        unsafe {
            ffi::g_converter_reset(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for Converter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Converter")
    }
}