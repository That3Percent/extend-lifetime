use std::mem::transmute;
use std::cell::{Ref, RefMut};

// TODO: Parameterize output lifetime instead of making it always 'static. This would require GAT

pub unsafe fn extend_lifetime<T: ExtendableLife>(r: T) -> T::Out {
    r.extend_lifetime()
}

pub unsafe fn extend_lifetime_mut<T: ExtendableLifeMut>(r: T) -> T::OutMut {
    r.extend_lifetime_mut()
}

pub unsafe trait ExtendableLife {
	type Out;

	unsafe fn extend_lifetime(self) -> Self::Out;
}

pub unsafe trait ExtendableLifeMut {
	type OutMut;
	unsafe fn extend_lifetime_mut(self) -> Self::OutMut;
}

unsafe impl<'a, T: ?Sized + 'static> ExtendableLife for &'a T {
	type Out=&'static T;
	unsafe fn extend_lifetime(self) -> Self::Out {
		transmute(self)
	}
}

unsafe impl<'a, T: ?Sized + 'static> ExtendableLifeMut for &'a mut T {
	type OutMut=&'static mut T;
	unsafe fn extend_lifetime_mut(self) -> Self::OutMut {
		transmute(self)
	}
}

unsafe impl<'a, T: ExtendableLife + Sized> ExtendableLife for Option<T> {
	type Out = Option<T::Out>;
	unsafe fn extend_lifetime(self) -> Self::Out {
		match self {
			None=>None,
			Some(inner)=>Some(inner.extend_lifetime())
		}
	}
}

unsafe impl<'a, T: ExtendableLifeMut + Sized> ExtendableLifeMut for Option<T> {
	type OutMut = Option<T::OutMut>;
	unsafe fn extend_lifetime_mut(self) -> Self::OutMut {
		match self {
			None=>None,
			Some(inner)=>Some(inner.extend_lifetime_mut())
		}
	}
}

unsafe impl<'a, T: 'static> ExtendableLife for Ref<'a, T> {
	type Out=Ref<'static, T>;
	unsafe fn extend_lifetime(self) -> Self::Out {
		transmute(self)
	}
}

unsafe impl<'a, T: 'static> ExtendableLifeMut for RefMut<'a, T> {
	type OutMut=RefMut<'static, T>;
	unsafe fn extend_lifetime_mut(self) -> Self::OutMut {
		transmute(self)
	}
}