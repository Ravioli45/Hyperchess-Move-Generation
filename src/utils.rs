macro_rules! num_and_all{
    (
        $(#[$meta:meta])*
        $vis:vis enum $trait:ident{
            $(
                $(#[$variant_meta:meta])*
                $variant:ident
                $(= $variant_value:expr)?
            ),*
            $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $trait{
            $(
                $(#[$variant_meta])*
                $variant
                $(= $variant_value)?
            ),*
        }
        impl $trait{
            $vis const NUM: usize = [$(Self::$variant),*].len();
            $vis const ALL: [Self; Self::NUM] = [$(Self::$variant),*];
        }
    };
}
pub(crate) use num_and_all;

macro_rules! impl_indexing {
    (
        $($enum_name:ident),*
        $(,)?
    ) => {
        $(
            impl<T, const N: usize> Index<$enum_name> for [T; N]{
                type Output = T;

                #[inline]
                fn index(&self, index: $enum_name) -> &Self::Output{
                    &self[index as usize]
                }
            }
            impl<T, const N: usize> IndexMut<$enum_name> for [T; N]{
                #[inline]
                fn index_mut(&mut self, index: $enum_name) -> &mut Self::Output{
                    &mut self[index as usize]
                }
            }
        )*
    };
}
pub(crate) use impl_indexing;

