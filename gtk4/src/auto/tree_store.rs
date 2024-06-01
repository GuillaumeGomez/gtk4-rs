// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, Buildable, TreeDragDest, TreeDragSource, TreeIter, TreeModel, TreeSortable};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkTreeStore")]
    pub struct TreeStore(Object<ffi::GtkTreeStore, ffi::GtkTreeStoreClass>) @implements Buildable, TreeDragDest, TreeDragSource, TreeModel, TreeSortable;

    match fn {
        type_ => || ffi::gtk_tree_store_get_type(),
    }
}

impl TreeStore {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_append")]
    pub fn append(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_append(
                self.to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
            );
            iter
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::gtk_tree_store_clear(self.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_insert")]
    pub fn insert(&self, parent: Option<&TreeIter>, position: i32) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert(
                self.to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
                position,
            );
            iter
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_insert_after")]
    pub fn insert_after(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_after(
                self.to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
                mut_override(sibling.to_glib_none().0),
            );
            iter
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_insert_before")]
    pub fn insert_before(&self, parent: Option<&TreeIter>, sibling: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_before(
                self.to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
                mut_override(sibling.to_glib_none().0),
            );
            iter
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_is_ancestor")]
    pub fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_is_ancestor(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(descendant.to_glib_none().0),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_iter_depth")]
    pub fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe {
            ffi::gtk_tree_store_iter_depth(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            )
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_iter_is_valid")]
    pub fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_iter_is_valid(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_move_after")]
    pub fn move_after(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_after(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(position.to_glib_none().0),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_move_before")]
    pub fn move_before(&self, iter: &TreeIter, position: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_tree_store_move_before(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                mut_override(position.to_glib_none().0),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_prepend")]
    pub fn prepend(&self, parent: Option<&TreeIter>) -> TreeIter {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_prepend(
                self.to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
            );
            iter
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_remove")]
    pub fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_remove(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_store_swap")]
    pub fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_tree_store_swap(
                self.to_glib_none().0,
                mut_override(a.to_glib_none().0),
                mut_override(b.to_glib_none().0),
            );
        }
    }
}
