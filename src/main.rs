#![feature(fn_traits, unboxed_closures, tuple_trait)]

use tuplex::*;
use std::{marker::Tuple, ops::{AddAssign, Deref, DerefMut}};
struct Curry<T: PopFront + Tuple, F: FnOnce<T>> where T::Remain: PushFront<T::Front> {
    v: T::Front,
    f: F,
}

impl<T: PopFront + Tuple, F: FnOnce<T>> FnOnce<T::Remain> for Curry<T, F> where T::Remain: Tuple + PushFront<T::Front, Output = T> {
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: T::Remain) -> Self::Output {
        self.f.call_once(args.push_front(self.v))
    }
}

impl<T: PopFront + Tuple , F: FnMut<T>> FnMut<T::Remain> for Curry<T, F> where T::Front: Clone, T::Remain: Tuple + PushFront<T::Front, Output = T> {
    extern "rust-call" fn call_mut(&mut self, args: T::Remain) -> Self::Output {
        self.f.call_mut(args.push_front(self.v.clone()))
    }
}

impl<T: PopFront + Tuple, F: Fn<T>> Fn<T::Remain> for Curry<T, F> where T::Front: Clone, T::Remain: Tuple + PushFront<T::Front, Output = T> {
    extern "rust-call" fn call(&self, args: T::Remain) -> Self::Output {
        self.f.call(args.push_front(self.v.clone()))
    }
}

fn curry<T: PopFront + Tuple, F: FnOnce<T>>(v: T::Front, f: F) -> Curry<T, F> where T::Remain: PushFront<T::Front> {
    Curry { v, f }
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

fn main() {
    let add = |x, y, z| x+y+z;
    let curried = curry(5, add);
    let res = curried(6, 2);
    println!("{}", res);
    let _n = Nothing()()()()()()()()()();
    let _s = Something(6)()()()()()()();
    let mut rev = Reverse(10);
    rev.add_assign(1);
    let res = rev(|x| x+5);
    println!("{res}");
}
