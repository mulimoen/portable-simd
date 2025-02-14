/// Provides implementations of `From<$a> for $b` and `From<$b> for $a` that transmutes the value.
macro_rules! from_transmute {
    { unsafe $a:ty => $b:ty } => {
        from_transmute!{ @impl $a => $b }
        from_transmute!{ @impl $b => $a }
    };
    { @impl $from:ty => $to:ty } => {
        impl core::convert::From<$from> for $to {
            #[inline]
            fn from(value: $from) -> $to {
                unsafe { core::mem::transmute(value) }
            }
        }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

#[cfg(any(target_arch = "wasm32"))]
mod wasm32;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod arm;

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
mod powerpc;
