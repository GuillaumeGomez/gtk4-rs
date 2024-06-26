// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::prelude::*;

glib::wrapper! {
    #[doc(alias = "GtkBuilderScope")]
    pub struct BuilderScope(Interface<ffi::GtkBuilderScope, ffi::GtkBuilderScopeInterface>);

    match fn {
        type_ => || ffi::gtk_builder_scope_get_type(),
    }
}

impl BuilderScope {
    pub const NONE: Option<&'static BuilderScope> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::BuilderScope>> Sealed for T {}
}

pub trait BuilderScopeExt: IsA<BuilderScope> + sealed::Sealed + 'static {}

impl<O: IsA<BuilderScope>> BuilderScopeExt for O {}
