#![feature(const_trait_impl)]
#![feature(associated_const_equality)]
#![feature(generic_const_exprs)]

use std::mem::ManuallyDrop;

union Transmutation<A, B>
where
    A: SizedConst,
    B: SizedConst<SIZE = {A::SIZE}>
{
    a: ManuallyDrop<A>,
    b: ManuallyDrop<B>
}

#[const_trait]
pub trait SizedConst: Sized
{
    const SIZE: usize;

    unsafe fn transmute_into<To: SizedConst<SIZE = {Self::SIZE}>>(self) -> To;
    unsafe fn transmute_from<From: SizedConst<SIZE = {Self::SIZE}>>(from: From) -> Self;
}
impl<T> const SizedConst for T
where
    T: Sized
{
    const SIZE: usize = std::mem::size_of::<Self>();

    unsafe fn transmute_into<To: SizedConst<SIZE = {Self::SIZE}>>(self) -> To
    {
        let t = Transmutation
        {
            a: ManuallyDrop::new(self)
        };
        ManuallyDrop::into_inner(t.b)
    }
    unsafe fn transmute_from<From: SizedConst<SIZE = {Self::SIZE}>>(from: From) -> Self
    {
        let t = Transmutation
        {
            b: ManuallyDrop::new(from)
        };
        ManuallyDrop::into_inner(t.a)
    }
}
