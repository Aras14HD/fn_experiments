#![feature(fn_traits, unboxed_closures, tuple_trait)]

use tuplex::*;
use std::marker::Tuple;
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

fn main() {
    let add = |x, y, z| x+y+z;
    let curried = curry(5, add);
    let res = curried(6, 2);
    println!("{}", res);
    let _n = Nothing()()()()()()()()()();
    let _s = Something(6)()()()()()()();
}
