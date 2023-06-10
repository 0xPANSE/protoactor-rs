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

pub fn is_debug<T>() -> bool {
    let is_debug = std::cell::Cell::new(true);
    let _ = [D(&is_debug, PhantomData::<T>)].clone();
    is_debug.get()
}
