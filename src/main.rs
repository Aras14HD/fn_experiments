#![feature(fn_traits, unboxed_closures, tuple_trait)]

use std::marker::{Tuple, PhantomData};
struct Curry<V, F: FnOnce<(V, U)>, U> {
    v: V,
    f: F,
    _phantom: PhantomData<U>
}

impl<V, U, F: FnOnce<(V, U)>> FnOnce<(U,)> for Curry<V, F, U> {
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: (U,)) -> Self::Output {
        self.f.call_once((self.v, args.0))
    }
}

impl<V: Clone, U, F: FnMut<(V, U)>> FnMut<(U,)> for Curry<V, F, U> {
    extern "rust-call" fn call_mut(&mut self, args: (U,)) -> Self::Output {
        self.f.call_mut((self.v.clone(), args.0))
    }
}

impl<V: Clone, U, F: Fn<(V, U)>> Fn<(U,)> for Curry<V, F, U> {
    extern "rust-call" fn call(&self, args: (U,)) -> Self::Output {
        self.f.call((self.v.clone(), args.0))
    }
}

fn curry<V, F: FnOnce<(V, U)>, U>(v: V, f: F) -> Curry<V, F, U> {
    Curry { v, f, _phantom: PhantomData }
}

fn main() {
    let add = |x, y| x+y;
    let curried = curry(5, add);
    let res = curried(6);
    println!("{}", res);
}
