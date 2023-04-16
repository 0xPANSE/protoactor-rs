use core::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

struct D<'a, T>(&'a std::cell::Cell<bool>, PhantomData<T>);
impl<T> Clone for D<'_, T> {
    fn clone(&self) -> Self {
        self.0.set(false);
        D(self.0, PhantomData)
    }
}
impl<T: Debug> Copy for D<'_, T> {}

/// Checks if a type implements `Debug`.
fn as_debug<T>(value: &T) -> Option<&T> {
    let is_debug = std::cell::Cell::new(true);
    let _ = [D(&is_debug, PhantomData::<T>)].clone();
    if is_debug.get() {
        Some(value)
    } else {
        None
    }
}

/// A wrapper type that implements `Debug` by delegating to `Debug` if the wrapped type implements it,
/// otherwise it prints `[unprintable <struct_name>]`.
pub struct AsDiagnostic<T>(pub T);

impl<T> Debug for AsDiagnostic<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match as_debug(self) {
            Some(value) => Debug::fmt(value, f),
            None => write!(f, "[unprintable <{}>]", stringify!(T)),
        }
    }
}
