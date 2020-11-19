use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;
use glib::GString;
use gtk_sys;
use libc::{c_char, c_int};

use Editable;

pub trait EditableImpl: ObjectImpl {
    fn insert_text(&self, editable: &Self::Type, text: &str, length: i32, position: &mut i32);
    fn delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32);
    fn changed(&self, editable: &Self::Type);
    fn get_text(&self, editable: &Self::Type) -> GString;
    fn do_insert_text(&self, editable: &Self::Type, text: &str, length: i32, position: &mut i32);
    fn do_delete_text(&self, editable: &Self::Type, start_position: i32, end_position: i32);
    fn get_selection_bounds(
        &self,
        editable: &Self::Type,
        start_position: &mut i32,
        end_position: &mut i32,
    ) -> bool;
    fn set_selection_bounds(&self, editable: &Self::Type, start_position: i32, end_position: i32);
}

unsafe impl<T: EditableImpl> IsImplementable<T> for Editable {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let editable_iface = &mut *(iface as *mut gtk_sys::GtkEditableInterface);

        editable_iface.insert_text = Some(editable_insert_text::<T>);
        editable_iface.delete_text = Some(editable_delete_text::<T>);
        editable_iface.changed = Some(editable_changed::<T>);
        editable_iface.get_text = Some(editable_get_text::<T>);
        editable_iface.do_insert_text = Some(editable_do_insert_text::<T>);
        editable_iface.do_delete_text = Some(editable_do_delete_text::<T>);
        editable_iface.get_selection_bounds = Some(editable_get_selection_bounds::<T>);
        editable_iface.set_selection_bounds = Some(editable_set_selection_bounds::<T>);
    }
}

unsafe extern "C" fn editable_insert_text<T: EditableImpl>(
    editable: *mut gtk_sys::GtkEditable,
    text: *const c_char,
    length: c_int,
    position: *mut c_int,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.insert_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        &GString::from_glib_borrow(text),
        length,
        &mut *position,
    )
}

unsafe extern "C" fn editable_delete_text<T: EditableImpl>(
    editable: *mut gtk_sys::GtkEditable,
    start_position: c_int,
    end_position: c_int,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.delete_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}

unsafe extern "C" fn editable_changed<T: EditableImpl>(editable: *mut gtk_sys::GtkEditable) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.changed(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
}

unsafe extern "C" fn editable_get_text<T: EditableImpl>(
    editable: *mut gtk_sys::GtkEditable,
) -> *const c_char {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_text(from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn editable_do_insert_text<T: EditableImpl>(
    editable: *mut gtk_sys::GtkEditable,
    text: *const c_char,
    length: i32,
    position: *mut i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.do_insert_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        &GString::from_glib_borrow(text),
        length,
        &mut *position,
    )
}

unsafe extern "C" fn editable_do_delete_text<T: EditableImpl>(
    editable: *mut gtk_sys::GtkEditable,
    start_position: i32,
    end_position: i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.do_delete_text(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}

unsafe extern "C" fn editable_get_selection_bounds<T: EditableImpl>(
    editable: *mut gtk_sys::GtkEditable,
    start_position: *mut i32,
    end_position: *mut i32,
) -> glib_sys::gboolean {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    let mut start_pos = 0;
    let mut end_pos = 0;

    let result = imp.get_selection_bounds(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        &mut start_pos,
        &mut end_pos,
    );

    if !start_position.is_null() {
        *start_position = start_pos;
    }

    if !end_position.is_null() {
        *end_position = end_pos;
    }
    result.to_glib()
}

unsafe extern "C" fn editable_set_selection_bounds<T: EditableImpl>(
    editable: *mut gtk_sys::GtkEditable,
    start_position: i32,
    end_position: i32,
) {
    let instance = &*(editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.set_selection_bounds(
        from_glib_borrow::<_, Editable>(editable).unsafe_cast_ref(),
        start_position,
        end_position,
    )
}
