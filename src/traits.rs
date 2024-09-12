pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one_for {
    ($($t:ty),*) => {
        $(
            impl One for $t {
                fn one() -> Self {
                    1
                }
            }
        )*
    };
}

impl_one_for!(u8, u16, u32, u64, usize);
