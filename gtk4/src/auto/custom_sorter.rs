// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Sorter;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkCustomSorter")]
    pub struct CustomSorter(Object<ffi::GtkCustomSorter, ffi::GtkCustomSorterClass>) @extends Sorter;

    match fn {
        type_ => || ffi::gtk_custom_sorter_get_type(),
    }
}

impl fmt::Display for CustomSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CustomSorter")
    }
}
