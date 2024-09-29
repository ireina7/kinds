pub trait Kind<'a> {
    type F<A: 'a>;
}

pub mod kinds {
    macro_rules! kinded {
        ($t:ident, $k:ty) => {
            pub struct $t;
            impl<'a> crate::kind::Kind<'a> for $t {
                type F<A: 'a> = $k;
            }
        };
    }

    pub mod std {
        pub mod vec {
            use crate::kind::Functor;

            pub struct Vec;
            impl<'a> crate::kind::Kind<'a> for Vec {
                type F<A: 'a> = ::std::vec::Vec<A>;
            }
            impl<'a> Functor<'a> for Vec {
                fn fmap<A: 'a, B: 'a>(fa: Self::F<A>, f: impl FnMut(A) -> B) -> Self::F<B> {
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
            struct Slice<'a>(::std::marker::PhantomData<&'a ()>);
            impl<'a> crate::kind::Kind<'a> for Slice<'a> {
                type F<A: 'a> = &'a [A];
            }
        }
        pub mod result {
            pub struct Result<Err> {
                _data: ::std::marker::PhantomData<Err>,
            }
            impl<'a, Err> crate::kind::Kind<'a> for Result<Err> {
                type F<A: 'a> = ::std::result::Result<A, Err>;
            }
        }
        pub mod collections {
            pub struct HashMap<K> {
                _data: ::std::marker::PhantomData<K>,
            }
            impl<'a, K> crate::kind::Kind<'a> for HashMap<K> {
                type F<A: 'a> = ::std::collections::HashMap<K, A>;
            }

            pub struct HashSet;
            impl<'a> crate::kind::Kind<'a> for HashSet {
                type F<A: 'a> = ::std::collections::HashSet<A>;
            }
        }
        pub mod array {
            struct Array<const N: usize>;
            impl<'a, const N: usize> crate::kind::Kind<'a> for Array<N> {
                type F<A: 'a> = [A; N];
            }
        }

        pub mod iter {
            pub struct Iter<I>(::std::marker::PhantomData<I>);
            impl<'a, I: Iterator + 'a> crate::kind::Kind<'a> for Iter<I> {
                type F<A: 'a> = I;
            }
        }

        pub mod future {
            pub struct Future<Fut>(::std::marker::PhantomData<Fut>);
            impl<'a, Fut: ::std::future::Future + 'a> crate::kind::Kind<'a> for Future<Fut> {
                type F<A: 'a> = Fut;
            }
        }
    }
}

/// fix point of kinded
pub struct Fix<'a, K: Kind<'a> + 'a>(K::F<Fix<'a, K>>)
where
    K::F<Fix<'a, K>>: Sized;

pub trait Functor<'a>: Kind<'a> {
    fn fmap<A, B>(fa: Self::F<A>, f: impl FnMut(A) -> B) -> Self::F<B>;
}
