// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct TreeListRow(Object<gtk_sys::GtkTreeListRow, gtk_sys::GtkTreeListRowClass, TreeListRowClass>);

    match fn {
        get_type => || gtk_sys::gtk_tree_list_row_get_type(),
    }
}

pub const NONE_TREE_LIST_ROW: Option<&TreeListRow> = None;

pub trait TreeListRowExt: 'static {
    fn get_child_row(&self, position: u32) -> Option<TreeListRow>;

    fn get_children(&self) -> Option<gio::ListModel>;

    fn get_depth(&self) -> u32;

    fn get_expanded(&self) -> bool;

    fn get_item(&self) -> Option<glib::Object>;

    fn get_parent(&self) -> Option<TreeListRow>;

    fn get_position(&self) -> u32;

    fn is_expandable(&self) -> bool;

    fn set_expanded(&self, expanded: bool);

    fn get_property_expandable(&self) -> bool;

    fn connect_property_children_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_depth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_expandable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeListRow>> TreeListRowExt for O {
    fn get_child_row(&self, position: u32) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_list_row_get_child_row(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    fn get_children(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tree_list_row_get_children(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_depth(&self) -> u32 {
        unsafe { gtk_sys::gtk_tree_list_row_get_depth(self.as_ref().to_glib_none().0) }
    }

    fn get_expanded(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_list_row_get_expanded(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_item(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_list_row_get_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_parent(&self) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_list_row_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_position(&self) -> u32 {
        unsafe { gtk_sys::gtk_tree_list_row_get_position(self.as_ref().to_glib_none().0) }
    }

    fn is_expandable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_list_row_is_expandable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_expanded(&self, expanded: bool) {
        unsafe {
            gtk_sys::gtk_tree_list_row_set_expanded(
                self.as_ref().to_glib_none().0,
                expanded.to_glib(),
            );
        }
    }

    fn get_property_expandable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"expandable\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn connect_property_children_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_children_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListRow>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListRow::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::children\0".as_ptr() as *const _,
                Some(transmute(notify_children_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_depth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_depth_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListRow>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListRow::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::depth\0".as_ptr() as *const _,
                Some(transmute(notify_depth_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_expandable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expandable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListRow>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListRow::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expandable\0".as_ptr() as *const _,
                Some(transmute(notify_expandable_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expanded_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListRow>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListRow::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expanded\0".as_ptr() as *const _,
                Some(transmute(notify_expanded_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListRow>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListRow::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::item\0".as_ptr() as *const _,
                Some(transmute(notify_item_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TreeListRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeListRow")
    }
}
