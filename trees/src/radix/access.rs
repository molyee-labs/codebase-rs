use std::marker::PhantomData;

pub(crate) trait Gate: Default {
    type Target;

    fn try_open<P>(&self, permit: P) -> AccessResult<Self, P>;
    fn open<P>(&self, permit: P) -> Access<Self, P>;
    fn close<P>(&self, access: &mut Access<Self, P>);
}

#[repr(C)]
pub(crate) struct Access<'a, G: Gate, P> {
    permit: P,
    gate: &'a G,
    ptr: *const G::Target,
    _m: PhantomData<G::Target>
}

impl<'a, T, G: Gate<Target=T>, P> Access<'a, G, P> {
    unsafe fn new<'g: 'a>(gate: &'g G, ptr: *const T, permit: P) -> Self {
        Access { permit, gate, ptr, _m: Default::default() }
    }

    pub unsafe fn get(&self) -> &T {
        &*self.ptr
    }

    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.ptr
    }

    pub unsafe fn to_owned(&self) -> T {
        *self.ptr
    }
}

impl<G: Gate, P> Drop for Access<'_, G, P> {
    fn drop(&mut self) {
        self.gate.close(self)
    }
}

pub(crate) type AccessResult<'a, G, P> = Result<Access<'a, G, P>, AccessError>;

#[repr(u8)]
pub(crate) enum AccessError {
    OpenAsMut,
    Open,
    TooManyReaders,
    Removed,
}

pub(crate) struct Transparent<T> {
    _m: PhantomData<T>
}

impl<T> Default for Transparent<T> {
    fn default() -> Self {
        Transparent::new()
    }
}

impl<T> Transparent<T> {
    fn new() -> Self {
        Transparent { _m: Default::default() }
    }

    unsafe fn ptr(&self) -> *const T {
        self as *const Self as *const T
    }
}

impl<T> Gate for Transparent<T> {
    type Target = T;

    fn try_open<P>(&self, permit: P) -> AccessResult<Self, P> {
        Ok(self.open(permit))
    }

    fn open<P>(&self, permit: P) -> Access<Self, P> {
        unsafe { Access::new(self, self.ptr(), permit) }
    }

    fn close<P>(&self, access: &mut Access<Self, P>) {
    }
}
