#![feature(fn_traits, unboxed_closures, tuple_trait)]

use std::{
    collections::btree_set::Union,
    marker::Tuple,
    ops::{AddAssign, Deref, DerefMut},
};
use tuplex::*;
struct Curry<T: PopFront + Tuple, F: FnOnce<T>>
where
    T::Remain: PushFront<T::Front>,
{
    v: T::Front,
    f: F,
}

impl<T: PopFront + Tuple, F: FnOnce<T>> FnOnce<T::Remain> for Curry<T, F>
where
    T::Remain: Tuple + PushFront<T::Front, Output = T>,
{
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: T::Remain) -> Self::Output {
        self.f.call_once(args.push_front(self.v))
    }
}

impl<T: PopFront + Tuple, F: FnMut<T>> FnMut<T::Remain> for Curry<T, F>
where
    T::Front: Clone,
    T::Remain: Tuple + PushFront<T::Front, Output = T>,
{
    extern "rust-call" fn call_mut(&mut self, args: T::Remain) -> Self::Output {
        self.f.call_mut(args.push_front(self.v.clone()))
    }
}

impl<T: PopFront + Tuple, F: Fn<T>> Fn<T::Remain> for Curry<T, F>
where
    T::Front: Clone,
    T::Remain: Tuple + PushFront<T::Front, Output = T>,
{
    extern "rust-call" fn call(&self, args: T::Remain) -> Self::Output {
        self.f.call(args.push_front(self.v.clone()))
    }
}

fn curry<T: PopFront + Tuple, F: FnOnce<T>>(v: T::Front, f: F) -> Curry<T, F>
where
    T::Remain: PushFront<T::Front>,
{
    Curry { v, f }
}

macro_rules! multicurry {
    (($head:expr, $tail:expr),$f:expr) => {
        multicurry!(($head, $tail,), $f)
    };
    (($head:expr, $tail:expr,),$f:expr) => {
        curry($head, multicurry!(($tail,), $f))
    };
    (($arg:expr), $f:expr) => {
        multicurry!(($arg,), $f)
    };
    (($arg:expr,), $f:expr) => {
        curry($arg, $f)
    };
    ($arg:expr, $f:expr) => {
        multicurry!(($arg,), $f)
    };
}

struct Nothing;

impl FnOnce<()> for Nothing {
    type Output = Nothing;
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        Nothing
    }
}

struct Something<T>(T);

impl<T> FnOnce<()> for Something<T> {
    type Output = Something<T>;
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self
    }
}

/// x(f)
struct Reverse<T>(pub T);

impl<T, F: FnOnce<(T,)>> FnOnce<(F,)> for Reverse<T> {
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: (F,)) -> Self::Output {
        args.0.call_once((self.0,))
    }
}

impl<T> AsRef<T> for Reverse<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Reverse<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for Reverse<T> {
    fn from(value: T) -> Self {
        Reverse(value)
    }
}

impl<T> Deref for Reverse<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Reverse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_fns_same {
    ($args:ty, $output:ty, $name:ty, {$e:expr}) => {
        impl FnOnce<$args> for $name {
            type Output = $output;
            extern "rust-call" fn call_once(self, args: $args) -> $output {
                ($e)(self, args)
            }
        }
        impl FnMut<$args> for $name {
            extern "rust-call" fn call_mut(&mut self, args: $args) -> $output {
                ($e)(self, args)
            }
        }
        impl Fn<$args> for $name {
            extern "rust-call" fn call(&self, args: $args) -> $output {
                ($e)(self, args)
            }
        }
    };
}

// fn variadic_default(a: u8, b: f64 = 10.0, c: usize..) {}
fn _base_variadic_default<const N: usize>(a: u8, b: f64, c: [usize; N]) {
    println!("Got: {a}, {b}, {c:?}");
}
#[allow(non_camel_case_types)]
struct variadic_default;
impl_fns_same!((u8,), (), variadic_default, {
    |_, args: (u8,)| _base_variadic_default(args.0, 10.0, [])
});
impl_fns_same!((u8, f64,), (), variadic_default, {
    |_, args: (u8, f64)| _base_variadic_default(args.0, args.1, [])
});
impl_fns_same!((u8, f64, usize), (), variadic_default, {
    |_, args: (u8, f64, usize)| _base_variadic_default(args.0, args.1, [args.2])
});
impl_fns_same!((u8, f64, usize, usize), (), variadic_default, {
    |_, args: (u8, f64, usize, usize)| _base_variadic_default(args.0, args.1, [args.2, args.3])
});
impl_fns_same!((u8, f64, usize, usize, usize), (), variadic_default, {
    |_, args: (u8, f64, usize, usize, usize)| {
        _base_variadic_default(args.0, args.1, [args.2, args.3, args.4])
    }
});
impl_fns_same!(
    (u8, f64, usize, usize, usize, usize),
    (),
    variadic_default,
    {
        |_, args: (u8, f64, usize, usize, usize, usize)| {
            _base_variadic_default(args.0, args.1, [args.2, args.3, args.4, args.5])
        }
    }
);

fn main() {
    let add = |x, y, z| x + y + z;
    let curried = curry(5, add);
    let res = curried(6, 2);
    println!("{}", res);
    let _n = Nothing()()()()()()()()()();
    let _s = Something(6)()()()()()()();
    let mut rev = Reverse(10);
    rev.add_assign(1);
    let res = rev(|x| x + 5);
    println!("{res}");
    let curried = |z| add(5, 6, z);
    let res = curried(2);
    println!("{res}");
    variadic_default(4, 5.1, 6, 7, 8);
}
