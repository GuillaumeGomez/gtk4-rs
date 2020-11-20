use gtk_sys;

use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

use Filter;
use FilterMatch;

pub trait FilterImpl: FilterImplExt + ObjectImpl {
    fn get_strictness(&self, filter: &Self::Type) -> FilterMatch {
        self.parent_get_strictness(filter)
    }
    fn match_(&self, filter: &Self::Type, item: &Object) -> bool {
        self.parent_match_(filter, item)
    }
}

pub trait FilterImplExt: ObjectSubclass {
    fn parent_get_strictness(&self, filter: &Self::Type) -> FilterMatch;
    fn parent_match_(&self, filter: &Self::Type, item: &Object) -> bool;
}

impl<T: FilterImpl> FilterImplExt for T {
    fn parent_get_strictness(&self, filter: &Self::Type) -> FilterMatch {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkFilterClass;
            let f = (*parent_class)
                .get_strictness
                .expect("No parent class impl for \"get_strictness\"");
            from_glib(f(filter.unsafe_cast_ref::<Filter>().to_glib_none().0))
        }
    }

    fn parent_match_(&self, filter: &Self::Type, item: &Object) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkFilterClass;
            let f = (*parent_class)
                .match_
                .expect("No parent class impl for \"match\"");
            from_glib(f(
                filter.unsafe_cast_ref::<Filter>().to_glib_none().0,
                item.to_glib_none().0,
            ))
        }
    }
}

unsafe impl<T: FilterImpl> IsSubclassable<T> for Filter {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.match_ = Some(filter_match::<T>);
        klass.get_strictness = Some(filter_get_strictness::<T>);
    }
}

unsafe extern "C" fn filter_get_strictness<T: FilterImpl>(
    ptr: *mut gtk_sys::GtkFilter,
) -> gtk_sys::GtkFilterMatch {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Filter> = from_glib_borrow(ptr);

    imp.get_strictness(wrap.unsafe_cast_ref()).to_glib()
}

unsafe extern "C" fn filter_match<T: FilterImpl>(
    ptr: *mut gtk_sys::GtkFilter,
    itemptr: *mut gobject_sys::GObject,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Filter> = from_glib_borrow(ptr);

    imp.match_(wrap.unsafe_cast_ref(), &from_glib_borrow(itemptr))
        .to_glib()
}
