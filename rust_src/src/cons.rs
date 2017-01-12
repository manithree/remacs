extern crate libc;

use std::os::raw::c_char;
use std::ptr;
use std::mem;

use lisp::{LispObject, LispType, XTYPE, XUNTAG, Qt, Qnil, LispSubr, PvecType, VectorLikeHeader,
           PSEUDOVECTOR_AREA_BITS, CHECK_TYPE};

extern "C" {
    static Qconsp: LispObject;
    fn CHECK_IMPURE(obj: LispObject, ptr: *const libc::c_void);
}


fn CONSP(x: LispObject) -> bool {
    XTYPE(x) == LispType::Lisp_Cons
}

fn Fconsp(object: LispObject) -> LispObject {
    if CONSP(object) { unsafe { Qt } } else { Qnil }
}

defun!("consp", Fconsp, Sconsp, 1, 1, ptr::null(), "Return t if OBJECT is a cons cell.

(fn OBJECT)");

/// Represents a cons cell, or GC bookkeeping for cons cells.
///
/// A cons cell is pair of two pointers, used to build linked lists in
/// lisp.
///
/// # C Porting Notes
///
/// The equivalent C struct is `Lisp_Cons`. Note that the second field
/// may be used as the cdr or GC bookkeeping.
// TODO: this should be aligned to 8 bytes.
#[repr(C)]
#[allow(unused_variables)]
struct LispCons {
    /// Car of this cons cell.
    car: LispObject,
    /// Cdr of this cons cell, or the chain used for the free list.
    cdr: LispObject,
}

// alloc.c uses a union for `Lisp_Cons`, which we emulate with an
// opaque struct.
#[repr(C)]
#[allow(dead_code)]
pub struct LispConsChain {
    chain: *const LispCons,
}

/// Extract the LispCons data from an elisp value.
fn XCONS(a: LispObject) -> *mut LispCons {
    debug_assert!(CONSP(a));
    unsafe { mem::transmute(XUNTAG(a, LispType::Lisp_Cons)) }
}

/// Set the car of a cons cell.
fn XSETCAR(c: LispObject, n: LispObject) {
    let cons_cell = XCONS(c);
    unsafe {
        (*cons_cell).car = n;
    }
}

/// Set the cdr of a cons cell.
fn XSETCDR(c: LispObject, n: LispObject) {
    let cons_cell = XCONS(c);
    unsafe {
        (*cons_cell).cdr = n;
    }
}

#[no_mangle]
pub extern "C" fn Fsetcar(cell: LispObject, newcar: LispObject) -> LispObject {
    unsafe {
        CHECK_TYPE(CONSP(cell), Qconsp, cell);
        CHECK_IMPURE(cell, XCONS(cell) as *const libc::c_void);
    }

    XSETCAR(cell, newcar);
    newcar
}

defun!("setcar", Fsetcar, Ssetcar, 2, 2, ptr::null(), "Set the car of CELL to be NEWCAR. Returns NEWCAR.

(fn CELL NEWCAR)");

#[no_mangle]
pub extern "C" fn Fsetcdr(cell: LispObject, newcar: LispObject) -> LispObject {
    unsafe {
        CHECK_TYPE(CONSP(cell), Qconsp, cell);
        CHECK_IMPURE(cell, XCONS(cell) as *const libc::c_void);
    }

    XSETCDR(cell, newcar);
    newcar
}

defun!("setcdr", Fsetcdr, Ssetcdr, 2, 2, ptr::null(), "Set the cdr of CELL to be NEWCDR.  Returns NEWCDR.

(fn CELL NEWCDR)");
