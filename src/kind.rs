pub trait Kind {
    type F<A>: ?Sized;
}

pub mod kinds {
    macro_rules! kinded {
        ($t:ident, $k:ty) => {
            pub struct $t;
            impl crate::kind::Kind for $t {
                type F<A> = $k;
            }
        };
    }

    pub mod std {
        pub mod vec {
            use crate::kind::Functor;

            kinded!(Vec, ::std::vec::Vec<A>);
            impl Functor for Vec {
                fn fmap<A, B>(fa: Self::F<A>, f: impl FnMut(A) -> B) -> Self::F<B> {
                    fa.into_iter().map(f).collect()
                }
            }
        }
        pub mod option {
            kinded!(Option, ::std::option::Option<A>);
        }
        pub mod boxed {
            kinded!(Box, ::std::boxed::Box<A>);
        }
        pub mod rc {
            kinded!(Rc, ::std::rc::Rc<A>);
        }
        pub mod slice {
            kinded!(Slice, [A]);
        }
        pub mod result {
            pub struct Result<Err> {
                _data: ::std::marker::PhantomData<Err>,
            }
            impl<Err> crate::kind::Kind for Result<Err> {
                type F<A> = ::std::result::Result<A, Err>;
            }
        }
        pub mod collections {
            pub struct HashMap<K> {
                _data: ::std::marker::PhantomData<K>,
            }
            impl<K> crate::kind::Kind for HashMap<K> {
                type F<A> = ::std::collections::HashMap<K, A>;
            }

            pub struct HashSet;
            impl crate::kind::Kind for HashSet {
                type F<A> = ::std::collections::HashSet<A>;
            }
        }
        pub mod array {
            struct Array<const N: usize>;
            impl<const N: usize> crate::kind::Kind for Array<N> {
                type F<A> = [A; N];
            }
        }
    }

    pub mod iter {
        pub struct Iter<I>(::std::marker::PhantomData<I>);
        impl<I: Iterator> crate::kind::Kind for Iter<I> {
            type F<A> = I;
        }
    }

    pub mod future {
        pub struct Future<Fut>(::std::marker::PhantomData<Fut>);
        impl<Fut: ::std::future::Future> crate::kind::Kind for Future<Fut> {
            type F<A> = Fut;
        }
    }
}

/// fix point of kinded
pub struct Fix<K: Kind>(K::F<Fix<K>>)
where
    K::F<Fix<K>>: Sized;

pub trait Functor: Kind {
    fn fmap<A, B>(fa: Self::F<A>, f: impl FnMut(A) -> B) -> Self::F<B>;
}
