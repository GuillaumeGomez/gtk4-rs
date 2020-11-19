use gtk_sys;

use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

use libc::c_int;
use std::mem;

use LayoutChild;
use LayoutManager;
use Orientation;
use Widget;

pub trait LayoutManagerImpl: LayoutManagerImplExt + ObjectImpl {
    fn allocate(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        width: i32,
        height: i32,
        baseline: i32,
    ) {
        self.parent_allocate(layout_manager, widget, width, height, baseline)
    }
    fn create_layout_child(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        for_child: &Widget,
    ) -> LayoutChild {
        self.parent_create_layout_child(layout_manager, widget, for_child)
    }
    fn measure(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        self.parent_measure(layout_manager, widget, orientation, for_size)
    }

    fn root(&self, layout_manager: &Self::Type) {
        self.parent_root(layout_manager)
    }
    fn unroot(&self, layout_manager: &Self::Type) {
        self.parent_unroot(layout_manager)
    }
}

pub trait LayoutManagerImplExt: ObjectSubclass {
    fn parent_allocate(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        width: i32,
        height: i32,
        baseline: i32,
    );
    fn parent_create_layout_child(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        for_child: &Widget,
    ) -> LayoutChild;
    fn parent_measure(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32);
    fn parent_root(&self, layout_manager: &Self::Type);
    fn parent_unroot(&self, layout_manager: &Self::Type);
}

impl<T: LayoutManagerImpl> LayoutManagerImplExt for T {
    fn parent_allocate(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        width: i32,
        height: i32,
        baseline: i32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).allocate {
                f(
                    layout_manager
                        .unsafe_cast_ref::<LayoutManager>()
                        .to_glib_none()
                        .0,
                    widget.to_glib_none().0,
                    width,
                    height,
                    baseline,
                )
            }
        }
    }
    fn parent_create_layout_child(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        for_child: &Widget,
    ) -> LayoutChild {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkLayoutManagerClass;
            let f = (*parent_class)
                .create_layout_child
                .expect("No parent class impl for \"create_layout_child\"");
            from_glib_none(f(
                layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
                for_child.to_glib_none().0,
            ))
        }
    }
    fn parent_measure(
        &self,
        layout_manager: &Self::Type,
        widget: &Widget,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkLayoutManagerClass;
            let f = (*parent_class)
                .measure
                .expect("No parent class impl for \"measure\"");
            let mut minimum = mem::MaybeUninit::uninit();
            let mut natural = mem::MaybeUninit::uninit();
            let mut minimum_baseline = mem::MaybeUninit::uninit();
            let mut natural_baseline = mem::MaybeUninit::uninit();

            f(
                layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0,
                widget.to_glib_none().0,
                orientation.to_glib(),
                for_size,
                minimum.as_mut_ptr(),
                natural.as_mut_ptr(),
                minimum_baseline.as_mut_ptr(),
                natural_baseline.as_mut_ptr(),
            );
            (
                minimum.assume_init(),
                natural.assume_init(),
                minimum_baseline.assume_init(),
                natural_baseline.assume_init(),
            )
        }
    }
    fn parent_root(&self, layout_manager: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).root {
                f(layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }
    fn parent_unroot(&self, layout_manager: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkLayoutManagerClass;
            if let Some(f) = (*parent_class).unroot {
                f(layout_manager
                    .unsafe_cast_ref::<LayoutManager>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: LayoutManagerImpl> IsSubclassable<T> for LayoutManager {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.allocate = Some(layout_manager_allocate::<T>);
        klass.create_layout_child = Some(layout_manager_create_layout_child::<T>);
        klass.measure = Some(layout_manager_measure::<T>);
        klass.root = Some(layout_manager_root::<T>);
        klass.unroot = Some(layout_manager_unroot::<T>);
    }
}

unsafe extern "C" fn layout_manager_allocate<T: LayoutManagerImpl>(
    ptr: *mut gtk_sys::GtkLayoutManager,
    widgetptr: *mut gtk_sys::GtkWidget,
    width: i32,
    height: i32,
    baseline: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);

    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    imp.allocate(wrap.unsafe_cast_ref(), &widget, width, height, baseline)
}

unsafe extern "C" fn layout_manager_create_layout_child<T: LayoutManagerImpl>(
    ptr: *mut gtk_sys::GtkLayoutManager,
    widgetptr: *mut gtk_sys::GtkWidget,
    for_childptr: *mut gtk_sys::GtkWidget,
) -> *mut gtk_sys::GtkLayoutChild {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);
    let for_child: Borrowed<Widget> = from_glib_borrow(for_childptr);

    imp.create_layout_child(wrap.unsafe_cast_ref(), &widget, &for_child)
        .to_glib_full()
}

unsafe extern "C" fn layout_manager_measure<T: LayoutManagerImpl>(
    ptr: *mut gtk_sys::GtkLayoutManager,
    widgetptr: *mut gtk_sys::GtkWidget,
    orientation: gtk_sys::GtkOrientation,
    for_size: i32,
    minmum_ptr: *mut c_int,
    natural_ptr: *mut c_int,
    minimum_baseline_ptr: *mut c_int,
    natural_baseline_ptr: *mut c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    let (min, nat, min_baseline, nat_baseline) = imp.measure(
        wrap.unsafe_cast_ref(),
        &widget,
        from_glib(orientation),
        for_size,
    );
    if !minmum_ptr.is_null() {
        *minmum_ptr = min;
    }
    if !natural_ptr.is_null() {
        *natural_ptr = nat;
    }
    if !minimum_baseline_ptr.is_null() {
        *minimum_baseline_ptr = min_baseline;
    }
    if !natural_baseline_ptr.is_null() {
        *natural_baseline_ptr = nat_baseline;
    }
}

unsafe extern "C" fn layout_manager_root<T: LayoutManagerImpl>(
    ptr: *mut gtk_sys::GtkLayoutManager,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);

    imp.root(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn layout_manager_unroot<T: LayoutManagerImpl>(
    ptr: *mut gtk_sys::GtkLayoutManager,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<LayoutManager> = from_glib_borrow(ptr);

    imp.unroot(wrap.unsafe_cast_ref())
}
