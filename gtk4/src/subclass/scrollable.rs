use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;
use gtk_sys;

use Border;
use Scrollable;

pub trait ScrollableImpl: ObjectImpl {
    fn get_border(&self, scrollable: &Self::Type, border: &mut Border) -> bool;
}

unsafe impl<T: ScrollableImpl> IsImplementable<T> for Scrollable {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let scrollable_iface = &mut *(iface as *mut gtk_sys::GtkScrollableInterface);

        scrollable_iface.get_border = Some(scrollable_get_border::<T>);
    }
}

unsafe extern "C" fn scrollable_get_border<T: ScrollableImpl>(
    scrollable: *mut gtk_sys::GtkScrollable,
    borderptr: *mut gtk_sys::GtkBorder,
) -> glib_sys::gboolean {
    let instance = &*(scrollable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_border(
        from_glib_borrow::<_, Scrollable>(scrollable).unsafe_cast_ref(),
        &mut from_glib_full(borderptr),
    )
    .to_glib()
}
