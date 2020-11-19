use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;
use glib::GString;
use glib::Variant;
use gtk_sys;

use Actionable;

pub trait ActionableImpl: ObjectImpl {
    fn get_action_name(&self, actionable: &Self::Type) -> Option<GString>;
    fn get_action_target_value(&self, actionable: &Self::Type) -> Option<Variant>;
    fn set_action_name(&self, actionable: &Self::Type, name: Option<&str>);
    fn set_action_target_value(&self, actionable: &Self::Type, value: Option<&Variant>);
}

unsafe impl<T: ActionableImpl> IsImplementable<T> for Actionable {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let actionable_iface = &mut *(iface as *mut gtk_sys::GtkActionableInterface);

        actionable_iface.get_action_name = Some(actionable_get_action_name::<T>);
        actionable_iface.get_action_target_value = Some(actionable_get_action_target_value::<T>);
        actionable_iface.set_action_name = Some(actionable_set_action_name::<T>);
        actionable_iface.set_action_target_value = Some(actionable_set_action_target_value::<T>);
    }
}

unsafe extern "C" fn actionable_get_action_name<T: ActionableImpl>(
    actionable: *mut gtk_sys::GtkActionable,
) -> *const libc::c_char {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_action_name(from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn actionable_get_action_target_value<T: ActionableImpl>(
    actionable: *mut gtk_sys::GtkActionable,
) -> *mut glib_sys::GVariant {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_action_target_value(from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn actionable_set_action_name<T: ActionableImpl>(
    actionable: *mut gtk_sys::GtkActionable,
    name: *const libc::c_char,
) {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();
    let name: Borrowed<Option<GString>> = from_glib_borrow(name);
    imp.set_action_name(
        from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref(),
        name.as_ref().as_ref().map(|s| s.as_str()),
    )
}

unsafe extern "C" fn actionable_set_action_target_value<T: ActionableImpl>(
    actionable: *mut gtk_sys::GtkActionable,
    value: *mut glib_sys::GVariant,
) {
    let instance = &*(actionable as *mut T::Instance);
    let imp = instance.get_impl();
    let val: Borrowed<Option<Variant>> = from_glib_borrow(value);

    imp.set_action_target_value(
        from_glib_borrow::<_, Actionable>(actionable).unsafe_cast_ref(),
        val.as_ref().as_ref(),
    )
}
