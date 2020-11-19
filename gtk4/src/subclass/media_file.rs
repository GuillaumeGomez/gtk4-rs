use gtk_sys;

use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

use MediaFile;

pub trait MediaFileImpl: MediaFileImplExt + ObjectImpl {
    fn close(&self, media_file: &Self::Type) {
        self.parent_close(media_file)
    }
    fn open(&self, media_file: &Self::Type) {
        self.parent_open(media_file)
    }
}

pub trait MediaFileImplExt: ObjectSubclass {
    fn parent_close(&self, media_file: &Self::Type);
    fn parent_open(&self, media_file: &Self::Type);
}

impl<T: MediaFileImpl> MediaFileImplExt for T {
    fn parent_close(&self, media_file: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkMediaFileClass;
            if let Some(f) = (*parent_class).close {
                f(media_file.unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }

    fn parent_open(&self, media_file: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkMediaFileClass;
            if let Some(f) = (*parent_class).open {
                f(media_file.unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: MediaFileImpl> IsSubclassable<T> for MediaFile {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.close = Some(media_file_close::<T>);
        klass.open = Some(media_file_open::<T>);
    }
}

unsafe extern "C" fn media_file_close<T: MediaFileImpl>(ptr: *mut gtk_sys::GtkMediaFile) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<MediaFile> = from_glib_borrow(ptr);

    imp.close(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn media_file_open<T: MediaFileImpl>(ptr: *mut gtk_sys::GtkMediaFile) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<MediaFile> = from_glib_borrow(ptr);

    imp.open(wrap.unsafe_cast_ref())
}
