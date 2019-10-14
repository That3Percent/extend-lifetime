use std::cell::{Ref, RefMut};
use std::mem::transmute;

// TODO: Parameterize output lifetime instead of making it always 'static. This would require GAT

pub unsafe fn extend_lifetime<T: ExtendableLife>(r: T) -> T::Out {
    r.extend_lifetime()
}


pub unsafe trait ExtendableLife {
    type Out;

    unsafe fn extend_lifetime(self) -> Self::Out;
}

unsafe impl<'a, T: ?Sized + 'static> ExtendableLife for &'a T {
    type Out = &'static T;
    unsafe fn extend_lifetime(self) -> Self::Out {
        transmute(self)
    }
}

unsafe impl<'a, T: ?Sized + 'static> ExtendableLife for &'a mut T {
    type Out = &'static mut T;
    unsafe fn extend_lifetime(self) -> Self::Out {
        transmute(self)
    }
}

unsafe impl<'a, T: ExtendableLife + Sized> ExtendableLife for Option<T> {
    type Out = Option<T::Out>;
    unsafe fn extend_lifetime(self) -> Self::Out {
        match self {
            None => None,
            Some(inner) => Some(inner.extend_lifetime()),
        }
    }
}

unsafe impl<'a, T: 'static> ExtendableLife for Ref<'a, T> {
    type Out = Ref<'static, T>;
    unsafe fn extend_lifetime(self) -> Self::Out {
        transmute(self)
    }
}

unsafe impl<'a, T: 'static> ExtendableLife for RefMut<'a, T> {
    type Out = RefMut<'static, T>;
    unsafe fn extend_lifetime(self) -> Self::Out {
        transmute(self)
    }
}
