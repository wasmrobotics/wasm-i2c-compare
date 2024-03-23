#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt;
use anyhow::Result;
use bno055::{
    mint::Quaternion as mint_Quaternion, AxisRemap as BNO055AxisRemap, BNO055AxisConfig,
    BNO055AxisSign, BNO055Calibration, BNO055OperationMode, BNO055PowerMode, Bno055,
};
use linux_embedded_hal::{Delay, I2cdev};
use tracing::{debug, info};
use crate::wasm_robotics::robotics::types::AccessError::{
    HardwareAccessError, NamedResourceNotFound,
};
use wasm_robotics::robotics::imus::{
    self, AccessError, AxisConfig, AxisRemap, AxisSigns, CalibrationData, Imu as HostImu,
    PowerMode, Quaternion,
};
use wasmtime::component::{self, bindgen};
pub struct ImuClient {
    interface0: exports::wasm_robotics::robotics::run::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl ImuClient {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            U: wasm_robotics::robotics::types::Host
                + wasm_robotics::robotics::imus::Host,
        {
            wasm_robotics::robotics::types::add_to_linker(linker, get)?;
            wasm_robotics::robotics::imus::add_to_linker(linker, get)?;
            Ok(())
        }
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate(&mut store)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let interface0 = exports::wasm_robotics::robotics::run::Guest::new(
                &mut __exports
                    .instance("wasm-robotics:robotics/run@0.1.0")
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!(
                                "exported instance `wasm-robotics:robotics/run@0.1.0` not present",
                            ),
                        );
                        error
                    }))?,
            )?;
            Ok(ImuClient { interface0 })
        }
        pub fn wasm_robotics_robotics_run(
            &self,
        ) -> &exports::wasm_robotics::robotics::run::Guest {
            &self.interface0
        }
    }
};
pub mod wasm_robotics {
    pub mod robotics {
        #[allow(clippy::all)]
        pub mod types {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            #[component(variant)]
            pub enum AccessError {
                #[component(name = "named-resource-not-found")]
                NamedResourceNotFound(String),
                #[component(name = "hardware-access-error")]
                HardwareAccessError(String),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AccessError {
                #[inline]
                fn clone(&self) -> AccessError {
                    match self {
                        AccessError::NamedResourceNotFound(__self_0) => {
                            AccessError::NamedResourceNotFound(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        AccessError::HardwareAccessError(__self_0) => {
                            AccessError::HardwareAccessError(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for AccessError {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::NamedResourceNotFound(value) => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(0u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).NamedResourceNotFound)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[0usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HardwareAccessError(value) => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(1u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).HardwareAccessError)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                    }
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    match self {
                        Self::NamedResourceNotFound(value) => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[0usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HardwareAccessError(value) => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[1usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for AccessError {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => {
                                Self::NamedResourceNotFound(
                                    <String as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[0usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.NamedResourceNotFound },
                                    )?,
                                )
                            }
                            1u32 => {
                                Self::HardwareAccessError(
                                    <String as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[1usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HardwareAccessError },
                                    )?,
                                )
                            }
                            discrim => {
                                return ::anyhow::__private::Err(
                                    ::anyhow::Error::msg({
                                        let res = ::alloc::fmt::format(
                                            format_args!("unexpected discriminant: {0}", discrim),
                                        );
                                        res
                                    }),
                                );
                            }
                        },
                    )
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                    if true {
                        if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                            )
                        }
                    }
                    let discrim = bytes[0];
                    let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                    let payload = &bytes[payload_offset..];
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => {
                                Self::NamedResourceNotFound(
                                    <String as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[0usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            1u8 => {
                                Self::HardwareAccessError(
                                    <String as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[1usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            discrim => {
                                return ::anyhow::__private::Err(
                                    ::anyhow::Error::msg({
                                        let res = ::alloc::fmt::format(
                                            format_args!("unexpected discriminant: {0}", discrim),
                                        );
                                        res
                                    }),
                                );
                            }
                        },
                    )
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerAccessError<T0: Copy, T1: Copy> {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadAccessError<T0, T1>,
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerAccessError<T0, T1> {
                    #[inline]
                    fn clone(&self) -> LowerAccessError<T0, T1> {
                        LowerAccessError {
                            tag: ::core::clone::Clone::clone(&self.tag),
                            payload: ::core::clone::Clone::clone(&self.payload),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerAccessError<T0, T1> {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadAccessError<T0: Copy, T1: Copy> {
                    NamedResourceNotFound: T0,
                    HardwareAccessError: T1,
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<
                    T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerPayloadAccessError<T0, T1> {
                    #[inline]
                    fn clone(&self) -> LowerPayloadAccessError<T0, T1> {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerPayloadAccessError<T0, T1> {}
                unsafe impl wasmtime::component::ComponentType for AccessError {
                    type Lower = LowerAccessError<
                        <String as wasmtime::component::ComponentType>::Lower,
                        <String as wasmtime::component::ComponentType>::Lower,
                    >;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_variant(
                            ty,
                            types,
                            &[
                                (
                                    "named-resource-not-found",
                                    Some(
                                        <String as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "hardware-access-error",
                                    Some(
                                        <String as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                            ],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                        ],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for AccessError {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[
                        Some(<String as wasmtime::component::ComponentType>::ABI),
                        Some(<String as wasmtime::component::ComponentType>::ABI),
                    ];
                }
            };
            impl core::fmt::Debug for AccessError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        AccessError::NamedResourceNotFound(e) => {
                            f.debug_tuple("AccessError::NamedResourceNotFound")
                                .field(e)
                                .finish()
                        }
                        AccessError::HardwareAccessError(e) => {
                            f.debug_tuple("AccessError::HardwareAccessError")
                                .field(e)
                                .finish()
                        }
                    }
                }
            }
            impl core::fmt::Display for AccessError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{0:?}", self))
                }
            }
            impl std::error::Error for AccessError {}
            const _: () = {
                if !(12 == <AccessError as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 12 == <AccessError as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <AccessError as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <AccessError as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            #[component(record)]
            pub struct Quaternion {
                #[component(name = "x")]
                pub x: f32,
                #[component(name = "y")]
                pub y: f32,
                #[component(name = "z")]
                pub z: f32,
                #[component(name = "w")]
                pub w: f32,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Quaternion {}
            #[automatically_derived]
            impl ::core::clone::Clone for Quaternion {
                #[inline]
                fn clone(&self) -> Quaternion {
                    let _: ::core::clone::AssertParamIsClone<f32>;
                    *self
                }
            }
            unsafe impl wasmtime::component::Lower for Quaternion {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.x,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).x)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.y,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).y)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.z,
                        cx,
                        ty.fields[2usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).z)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.w,
                        cx,
                        ty.fields[3usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).w)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.x,
                        cx,
                        ty.fields[0usize].ty,
                        <f32 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.y,
                        cx,
                        ty.fields[1usize].ty,
                        <f32 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.z,
                        cx,
                        ty.fields[2usize].ty,
                        <f32 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.w,
                        cx,
                        ty.fields[3usize].ty,
                        <f32 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for Quaternion {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        x: <f32 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.x,
                        )?,
                        y: <f32 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.y,
                        )?,
                        z: <f32 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[2usize].ty,
                            &src.z,
                        )?,
                        w: <f32 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[3usize].ty,
                            &src.w,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        x: <f32 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<f32 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<f32 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        y: <f32 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<f32 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<f32 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        z: <f32 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[2usize].ty,
                            &bytes[<f32 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<f32 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        w: <f32 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[3usize].ty,
                            &bytes[<f32 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<f32 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerQuaternion<T0: Copy, T1: Copy, T2: Copy, T3: Copy> {
                    x: T0,
                    y: T1,
                    z: T2,
                    w: T3,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                    T2: ::core::clone::Clone + Copy,
                    T3: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerQuaternion<T0, T1, T2, T3> {
                    #[inline]
                    fn clone(&self) -> LowerQuaternion<T0, T1, T2, T3> {
                        LowerQuaternion {
                            x: ::core::clone::Clone::clone(&self.x),
                            y: ::core::clone::Clone::clone(&self.y),
                            z: ::core::clone::Clone::clone(&self.z),
                            w: ::core::clone::Clone::clone(&self.w),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                    T2: ::core::marker::Copy + Copy,
                    T3: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerQuaternion<T0, T1, T2, T3> {}
                unsafe impl wasmtime::component::ComponentType for Quaternion {
                    type Lower = LowerQuaternion<
                        <f32 as wasmtime::component::ComponentType>::Lower,
                        <f32 as wasmtime::component::ComponentType>::Lower,
                        <f32 as wasmtime::component::ComponentType>::Lower,
                        <f32 as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <f32 as wasmtime::component::ComponentType>::ABI,
                            <f32 as wasmtime::component::ComponentType>::ABI,
                            <f32 as wasmtime::component::ComponentType>::ABI,
                            <f32 as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "x",
                                    <f32 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "y",
                                    <f32 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "z",
                                    <f32 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "w",
                                    <f32 as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for Quaternion {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Quaternion")
                        .field("x", &self.x)
                        .field("y", &self.y)
                        .field("z", &self.z)
                        .field("w", &self.w)
                        .finish()
                }
            }
            const _: () = {
                if !(16 == <Quaternion as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 16 == <Quaternion as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <Quaternion as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <Quaternion as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub trait Host {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasm-robotics:robotics/types@0.1.0")?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod imus {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type AccessError = super::super::super::wasm_robotics::robotics::types::AccessError;
            const _: () = {
                if !(12 == <AccessError as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 12 == <AccessError as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <AccessError as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <AccessError as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub type Quaternion = super::super::super::wasm_robotics::robotics::types::Quaternion;
            const _: () = {
                if !(16 == <Quaternion as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 16 == <Quaternion as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <Quaternion as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <Quaternion as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            #[component(record)]
            pub struct CalibrationData {
                #[component(name = "acc-offset-x-lsb")]
                pub acc_offset_x_lsb: u8,
                #[component(name = "acc-offset-x-msb")]
                pub acc_offset_x_msb: u8,
                #[component(name = "acc-offset-y-lsb")]
                pub acc_offset_y_lsb: u8,
                #[component(name = "acc-offset-y-msb")]
                pub acc_offset_y_msb: u8,
                #[component(name = "acc-offset-z-lsb")]
                pub acc_offset_z_lsb: u8,
                #[component(name = "acc-offset-z-msb")]
                pub acc_offset_z_msb: u8,
                #[component(name = "mag-offset-x-lsb")]
                pub mag_offset_x_lsb: u8,
                #[component(name = "mag-offset-x-msb")]
                pub mag_offset_x_msb: u8,
                #[component(name = "mag-offset-y-lsb")]
                pub mag_offset_y_lsb: u8,
                #[component(name = "mag-offset-y-msb")]
                pub mag_offset_y_msb: u8,
                #[component(name = "mag-offset-z-lsb")]
                pub mag_offset_z_lsb: u8,
                #[component(name = "mag-offset-z-msb")]
                pub mag_offset_z_msb: u8,
                #[component(name = "gyr-offset-x-lsb")]
                pub gyr_offset_x_lsb: u8,
                #[component(name = "gyr-offset-x-msb")]
                pub gyr_offset_x_msb: u8,
                #[component(name = "gyr-offset-y-lsb")]
                pub gyr_offset_y_lsb: u8,
                #[component(name = "gyr-offset-y-msb")]
                pub gyr_offset_y_msb: u8,
                #[component(name = "gyr-offset-z-lsb")]
                pub gyr_offset_z_lsb: u8,
                #[component(name = "gyr-offset-z-msb")]
                pub gyr_offset_z_msb: u8,
                #[component(name = "acc-radius-lsb")]
                pub acc_radius_lsb: u8,
                #[component(name = "acc-radius-msb")]
                pub acc_radius_msb: u8,
                #[component(name = "mag-radius-lsb")]
                pub mag_radius_lsb: u8,
                #[component(name = "mag-radius-msb")]
                pub mag_radius_msb: u8,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for CalibrationData {}
            #[automatically_derived]
            impl ::core::clone::Clone for CalibrationData {
                #[inline]
                fn clone(&self) -> CalibrationData {
                    let _: ::core::clone::AssertParamIsClone<u8>;
                    *self
                }
            }
            unsafe impl wasmtime::component::Lower for CalibrationData {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.acc_offset_x_lsb,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_offset_x_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.acc_offset_x_msb,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_offset_x_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.acc_offset_y_lsb,
                        cx,
                        ty.fields[2usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_offset_y_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.acc_offset_y_msb,
                        cx,
                        ty.fields[3usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_offset_y_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.acc_offset_z_lsb,
                        cx,
                        ty.fields[4usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_offset_z_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.acc_offset_z_msb,
                        cx,
                        ty.fields[5usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_offset_z_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_offset_x_lsb,
                        cx,
                        ty.fields[6usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_offset_x_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_offset_x_msb,
                        cx,
                        ty.fields[7usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_offset_x_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_offset_y_lsb,
                        cx,
                        ty.fields[8usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_offset_y_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_offset_y_msb,
                        cx,
                        ty.fields[9usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_offset_y_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_offset_z_lsb,
                        cx,
                        ty.fields[10usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_offset_z_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_offset_z_msb,
                        cx,
                        ty.fields[11usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_offset_z_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.gyr_offset_x_lsb,
                        cx,
                        ty.fields[12usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).gyr_offset_x_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.gyr_offset_x_msb,
                        cx,
                        ty.fields[13usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).gyr_offset_x_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.gyr_offset_y_lsb,
                        cx,
                        ty.fields[14usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).gyr_offset_y_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.gyr_offset_y_msb,
                        cx,
                        ty.fields[15usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).gyr_offset_y_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.gyr_offset_z_lsb,
                        cx,
                        ty.fields[16usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).gyr_offset_z_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.gyr_offset_z_msb,
                        cx,
                        ty.fields[17usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).gyr_offset_z_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.acc_radius_lsb,
                        cx,
                        ty.fields[18usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_radius_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.acc_radius_msb,
                        cx,
                        ty.fields[19usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).acc_radius_msb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_radius_lsb,
                        cx,
                        ty.fields[20usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_radius_lsb)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.mag_radius_msb,
                        cx,
                        ty.fields[21usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).mag_radius_msb)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.acc_offset_x_lsb,
                        cx,
                        ty.fields[0usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.acc_offset_x_msb,
                        cx,
                        ty.fields[1usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.acc_offset_y_lsb,
                        cx,
                        ty.fields[2usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.acc_offset_y_msb,
                        cx,
                        ty.fields[3usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.acc_offset_z_lsb,
                        cx,
                        ty.fields[4usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.acc_offset_z_msb,
                        cx,
                        ty.fields[5usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_offset_x_lsb,
                        cx,
                        ty.fields[6usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_offset_x_msb,
                        cx,
                        ty.fields[7usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_offset_y_lsb,
                        cx,
                        ty.fields[8usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_offset_y_msb,
                        cx,
                        ty.fields[9usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_offset_z_lsb,
                        cx,
                        ty.fields[10usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_offset_z_msb,
                        cx,
                        ty.fields[11usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.gyr_offset_x_lsb,
                        cx,
                        ty.fields[12usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.gyr_offset_x_msb,
                        cx,
                        ty.fields[13usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.gyr_offset_y_lsb,
                        cx,
                        ty.fields[14usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.gyr_offset_y_msb,
                        cx,
                        ty.fields[15usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.gyr_offset_z_lsb,
                        cx,
                        ty.fields[16usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.gyr_offset_z_msb,
                        cx,
                        ty.fields[17usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.acc_radius_lsb,
                        cx,
                        ty.fields[18usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.acc_radius_msb,
                        cx,
                        ty.fields[19usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_radius_lsb,
                        cx,
                        ty.fields[20usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.mag_radius_msb,
                        cx,
                        ty.fields[21usize].ty,
                        <u8 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for CalibrationData {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        acc_offset_x_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.acc_offset_x_lsb,
                        )?,
                        acc_offset_x_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.acc_offset_x_msb,
                        )?,
                        acc_offset_y_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[2usize].ty,
                            &src.acc_offset_y_lsb,
                        )?,
                        acc_offset_y_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[3usize].ty,
                            &src.acc_offset_y_msb,
                        )?,
                        acc_offset_z_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[4usize].ty,
                            &src.acc_offset_z_lsb,
                        )?,
                        acc_offset_z_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[5usize].ty,
                            &src.acc_offset_z_msb,
                        )?,
                        mag_offset_x_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[6usize].ty,
                            &src.mag_offset_x_lsb,
                        )?,
                        mag_offset_x_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[7usize].ty,
                            &src.mag_offset_x_msb,
                        )?,
                        mag_offset_y_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[8usize].ty,
                            &src.mag_offset_y_lsb,
                        )?,
                        mag_offset_y_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[9usize].ty,
                            &src.mag_offset_y_msb,
                        )?,
                        mag_offset_z_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[10usize].ty,
                            &src.mag_offset_z_lsb,
                        )?,
                        mag_offset_z_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[11usize].ty,
                            &src.mag_offset_z_msb,
                        )?,
                        gyr_offset_x_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[12usize].ty,
                            &src.gyr_offset_x_lsb,
                        )?,
                        gyr_offset_x_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[13usize].ty,
                            &src.gyr_offset_x_msb,
                        )?,
                        gyr_offset_y_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[14usize].ty,
                            &src.gyr_offset_y_lsb,
                        )?,
                        gyr_offset_y_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[15usize].ty,
                            &src.gyr_offset_y_msb,
                        )?,
                        gyr_offset_z_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[16usize].ty,
                            &src.gyr_offset_z_lsb,
                        )?,
                        gyr_offset_z_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[17usize].ty,
                            &src.gyr_offset_z_msb,
                        )?,
                        acc_radius_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[18usize].ty,
                            &src.acc_radius_lsb,
                        )?,
                        acc_radius_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[19usize].ty,
                            &src.acc_radius_msb,
                        )?,
                        mag_radius_lsb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[20usize].ty,
                            &src.mag_radius_lsb,
                        )?,
                        mag_radius_msb: <u8 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[21usize].ty,
                            &src.mag_radius_msb,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        acc_offset_x_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        acc_offset_x_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        acc_offset_y_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[2usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        acc_offset_y_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[3usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        acc_offset_z_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[4usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        acc_offset_z_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[5usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_offset_x_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[6usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_offset_x_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[7usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_offset_y_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[8usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_offset_y_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[9usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_offset_z_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[10usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_offset_z_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[11usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        gyr_offset_x_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[12usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        gyr_offset_x_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[13usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        gyr_offset_y_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[14usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        gyr_offset_y_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[15usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        gyr_offset_z_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[16usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        gyr_offset_z_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[17usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        acc_radius_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[18usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        acc_radius_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[19usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_radius_lsb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[20usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        mag_radius_msb: <u8 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[21usize].ty,
                            &bytes[<u8 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u8 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerCalibrationData<
                    T0: Copy,
                    T1: Copy,
                    T2: Copy,
                    T3: Copy,
                    T4: Copy,
                    T5: Copy,
                    T6: Copy,
                    T7: Copy,
                    T8: Copy,
                    T9: Copy,
                    T10: Copy,
                    T11: Copy,
                    T12: Copy,
                    T13: Copy,
                    T14: Copy,
                    T15: Copy,
                    T16: Copy,
                    T17: Copy,
                    T18: Copy,
                    T19: Copy,
                    T20: Copy,
                    T21: Copy,
                > {
                    acc_offset_x_lsb: T0,
                    acc_offset_x_msb: T1,
                    acc_offset_y_lsb: T2,
                    acc_offset_y_msb: T3,
                    acc_offset_z_lsb: T4,
                    acc_offset_z_msb: T5,
                    mag_offset_x_lsb: T6,
                    mag_offset_x_msb: T7,
                    mag_offset_y_lsb: T8,
                    mag_offset_y_msb: T9,
                    mag_offset_z_lsb: T10,
                    mag_offset_z_msb: T11,
                    gyr_offset_x_lsb: T12,
                    gyr_offset_x_msb: T13,
                    gyr_offset_y_lsb: T14,
                    gyr_offset_y_msb: T15,
                    gyr_offset_z_lsb: T16,
                    gyr_offset_z_msb: T17,
                    acc_radius_lsb: T18,
                    acc_radius_msb: T19,
                    mag_radius_lsb: T20,
                    mag_radius_msb: T21,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                    T2: ::core::clone::Clone + Copy,
                    T3: ::core::clone::Clone + Copy,
                    T4: ::core::clone::Clone + Copy,
                    T5: ::core::clone::Clone + Copy,
                    T6: ::core::clone::Clone + Copy,
                    T7: ::core::clone::Clone + Copy,
                    T8: ::core::clone::Clone + Copy,
                    T9: ::core::clone::Clone + Copy,
                    T10: ::core::clone::Clone + Copy,
                    T11: ::core::clone::Clone + Copy,
                    T12: ::core::clone::Clone + Copy,
                    T13: ::core::clone::Clone + Copy,
                    T14: ::core::clone::Clone + Copy,
                    T15: ::core::clone::Clone + Copy,
                    T16: ::core::clone::Clone + Copy,
                    T17: ::core::clone::Clone + Copy,
                    T18: ::core::clone::Clone + Copy,
                    T19: ::core::clone::Clone + Copy,
                    T20: ::core::clone::Clone + Copy,
                    T21: ::core::clone::Clone + Copy,
                > ::core::clone::Clone
                for LowerCalibrationData<
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                    T17,
                    T18,
                    T19,
                    T20,
                    T21,
                > {
                    #[inline]
                    fn clone(
                        &self,
                    ) -> LowerCalibrationData<
                        T0,
                        T1,
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                        T19,
                        T20,
                        T21,
                    > {
                        LowerCalibrationData {
                            acc_offset_x_lsb: ::core::clone::Clone::clone(
                                &self.acc_offset_x_lsb,
                            ),
                            acc_offset_x_msb: ::core::clone::Clone::clone(
                                &self.acc_offset_x_msb,
                            ),
                            acc_offset_y_lsb: ::core::clone::Clone::clone(
                                &self.acc_offset_y_lsb,
                            ),
                            acc_offset_y_msb: ::core::clone::Clone::clone(
                                &self.acc_offset_y_msb,
                            ),
                            acc_offset_z_lsb: ::core::clone::Clone::clone(
                                &self.acc_offset_z_lsb,
                            ),
                            acc_offset_z_msb: ::core::clone::Clone::clone(
                                &self.acc_offset_z_msb,
                            ),
                            mag_offset_x_lsb: ::core::clone::Clone::clone(
                                &self.mag_offset_x_lsb,
                            ),
                            mag_offset_x_msb: ::core::clone::Clone::clone(
                                &self.mag_offset_x_msb,
                            ),
                            mag_offset_y_lsb: ::core::clone::Clone::clone(
                                &self.mag_offset_y_lsb,
                            ),
                            mag_offset_y_msb: ::core::clone::Clone::clone(
                                &self.mag_offset_y_msb,
                            ),
                            mag_offset_z_lsb: ::core::clone::Clone::clone(
                                &self.mag_offset_z_lsb,
                            ),
                            mag_offset_z_msb: ::core::clone::Clone::clone(
                                &self.mag_offset_z_msb,
                            ),
                            gyr_offset_x_lsb: ::core::clone::Clone::clone(
                                &self.gyr_offset_x_lsb,
                            ),
                            gyr_offset_x_msb: ::core::clone::Clone::clone(
                                &self.gyr_offset_x_msb,
                            ),
                            gyr_offset_y_lsb: ::core::clone::Clone::clone(
                                &self.gyr_offset_y_lsb,
                            ),
                            gyr_offset_y_msb: ::core::clone::Clone::clone(
                                &self.gyr_offset_y_msb,
                            ),
                            gyr_offset_z_lsb: ::core::clone::Clone::clone(
                                &self.gyr_offset_z_lsb,
                            ),
                            gyr_offset_z_msb: ::core::clone::Clone::clone(
                                &self.gyr_offset_z_msb,
                            ),
                            acc_radius_lsb: ::core::clone::Clone::clone(
                                &self.acc_radius_lsb,
                            ),
                            acc_radius_msb: ::core::clone::Clone::clone(
                                &self.acc_radius_msb,
                            ),
                            mag_radius_lsb: ::core::clone::Clone::clone(
                                &self.mag_radius_lsb,
                            ),
                            mag_radius_msb: ::core::clone::Clone::clone(
                                &self.mag_radius_msb,
                            ),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                    T2: ::core::marker::Copy + Copy,
                    T3: ::core::marker::Copy + Copy,
                    T4: ::core::marker::Copy + Copy,
                    T5: ::core::marker::Copy + Copy,
                    T6: ::core::marker::Copy + Copy,
                    T7: ::core::marker::Copy + Copy,
                    T8: ::core::marker::Copy + Copy,
                    T9: ::core::marker::Copy + Copy,
                    T10: ::core::marker::Copy + Copy,
                    T11: ::core::marker::Copy + Copy,
                    T12: ::core::marker::Copy + Copy,
                    T13: ::core::marker::Copy + Copy,
                    T14: ::core::marker::Copy + Copy,
                    T15: ::core::marker::Copy + Copy,
                    T16: ::core::marker::Copy + Copy,
                    T17: ::core::marker::Copy + Copy,
                    T18: ::core::marker::Copy + Copy,
                    T19: ::core::marker::Copy + Copy,
                    T20: ::core::marker::Copy + Copy,
                    T21: ::core::marker::Copy + Copy,
                > ::core::marker::Copy
                for LowerCalibrationData<
                    T0,
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                    T15,
                    T16,
                    T17,
                    T18,
                    T19,
                    T20,
                    T21,
                > {}
                unsafe impl wasmtime::component::ComponentType for CalibrationData {
                    type Lower = LowerCalibrationData<
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                        <u8 as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                            <u8 as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "acc-offset-x-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "acc-offset-x-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "acc-offset-y-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "acc-offset-y-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "acc-offset-z-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "acc-offset-z-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-offset-x-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-offset-x-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-offset-y-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-offset-y-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-offset-z-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-offset-z-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "gyr-offset-x-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "gyr-offset-x-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "gyr-offset-y-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "gyr-offset-y-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "gyr-offset-z-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "gyr-offset-z-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "acc-radius-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "acc-radius-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-radius-lsb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "mag-radius-msb",
                                    <u8 as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for CalibrationData {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("CalibrationData")
                        .field("acc-offset-x-lsb", &self.acc_offset_x_lsb)
                        .field("acc-offset-x-msb", &self.acc_offset_x_msb)
                        .field("acc-offset-y-lsb", &self.acc_offset_y_lsb)
                        .field("acc-offset-y-msb", &self.acc_offset_y_msb)
                        .field("acc-offset-z-lsb", &self.acc_offset_z_lsb)
                        .field("acc-offset-z-msb", &self.acc_offset_z_msb)
                        .field("mag-offset-x-lsb", &self.mag_offset_x_lsb)
                        .field("mag-offset-x-msb", &self.mag_offset_x_msb)
                        .field("mag-offset-y-lsb", &self.mag_offset_y_lsb)
                        .field("mag-offset-y-msb", &self.mag_offset_y_msb)
                        .field("mag-offset-z-lsb", &self.mag_offset_z_lsb)
                        .field("mag-offset-z-msb", &self.mag_offset_z_msb)
                        .field("gyr-offset-x-lsb", &self.gyr_offset_x_lsb)
                        .field("gyr-offset-x-msb", &self.gyr_offset_x_msb)
                        .field("gyr-offset-y-lsb", &self.gyr_offset_y_lsb)
                        .field("gyr-offset-y-msb", &self.gyr_offset_y_msb)
                        .field("gyr-offset-z-lsb", &self.gyr_offset_z_lsb)
                        .field("gyr-offset-z-msb", &self.gyr_offset_z_msb)
                        .field("acc-radius-lsb", &self.acc_radius_lsb)
                        .field("acc-radius-msb", &self.acc_radius_msb)
                        .field("mag-radius-lsb", &self.mag_radius_lsb)
                        .field("mag-radius-msb", &self.mag_radius_msb)
                        .finish()
                }
            }
            const _: () = {
                if !(22
                    == <CalibrationData as wasmtime::component::ComponentType>::SIZE32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 22 == <CalibrationData as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(1
                    == <CalibrationData as wasmtime::component::ComponentType>::ALIGN32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <CalibrationData as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            #[component(record)]
            pub struct AxisSigns {
                #[component(name = "x-negative")]
                pub x_negative: bool,
                #[component(name = "y-negative")]
                pub y_negative: bool,
                #[component(name = "z-negative")]
                pub z_negative: bool,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for AxisSigns {}
            #[automatically_derived]
            impl ::core::clone::Clone for AxisSigns {
                #[inline]
                fn clone(&self) -> AxisSigns {
                    let _: ::core::clone::AssertParamIsClone<bool>;
                    *self
                }
            }
            unsafe impl wasmtime::component::Lower for AxisSigns {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.x_negative,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).x_negative)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.y_negative,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).y_negative)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.z_negative,
                        cx,
                        ty.fields[2usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).z_negative)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.x_negative,
                        cx,
                        ty.fields[0usize].ty,
                        <bool as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.y_negative,
                        cx,
                        ty.fields[1usize].ty,
                        <bool as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.z_negative,
                        cx,
                        ty.fields[2usize].ty,
                        <bool as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for AxisSigns {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        x_negative: <bool as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.x_negative,
                        )?,
                        y_negative: <bool as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.y_negative,
                        )?,
                        z_negative: <bool as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[2usize].ty,
                            &src.z_negative,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        x_negative: <bool as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<bool as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<bool as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        y_negative: <bool as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<bool as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<bool as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        z_negative: <bool as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[2usize].ty,
                            &bytes[<bool as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<bool as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerAxisSigns<T0: Copy, T1: Copy, T2: Copy> {
                    x_negative: T0,
                    y_negative: T1,
                    z_negative: T2,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                    T2: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerAxisSigns<T0, T1, T2> {
                    #[inline]
                    fn clone(&self) -> LowerAxisSigns<T0, T1, T2> {
                        LowerAxisSigns {
                            x_negative: ::core::clone::Clone::clone(&self.x_negative),
                            y_negative: ::core::clone::Clone::clone(&self.y_negative),
                            z_negative: ::core::clone::Clone::clone(&self.z_negative),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                    T2: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerAxisSigns<T0, T1, T2> {}
                unsafe impl wasmtime::component::ComponentType for AxisSigns {
                    type Lower = LowerAxisSigns<
                        <bool as wasmtime::component::ComponentType>::Lower,
                        <bool as wasmtime::component::ComponentType>::Lower,
                        <bool as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <bool as wasmtime::component::ComponentType>::ABI,
                            <bool as wasmtime::component::ComponentType>::ABI,
                            <bool as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "x-negative",
                                    <bool as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "y-negative",
                                    <bool as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "z-negative",
                                    <bool as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for AxisSigns {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("AxisSigns")
                        .field("x-negative", &self.x_negative)
                        .field("y-negative", &self.y_negative)
                        .field("z-negative", &self.z_negative)
                        .finish()
                }
            }
            const _: () = {
                if !(3 == <AxisSigns as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 3 == <AxisSigns as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(1 == <AxisSigns as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <AxisSigns as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            #[component(enum)]
            pub enum AxisConfig {
                #[component(name = "axis-x")]
                AxisX,
                #[component(name = "axis-y")]
                AxisY,
                #[component(name = "axis-z")]
                AxisZ,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AxisConfig {
                #[inline]
                fn clone(&self) -> AxisConfig {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for AxisConfig {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for AxisConfig {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for AxisConfig {
                #[inline]
                fn eq(&self, other: &AxisConfig) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for AxisConfig {}
            #[automatically_derived]
            impl ::core::cmp::Eq for AxisConfig {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            unsafe impl wasmtime::component::Lower for AxisConfig {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::AxisX => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(0u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).AxisX)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::AxisY => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(1u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).AxisY)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::AxisZ => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(2u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).AxisZ)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                    }
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    match self {
                        Self::AxisX => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            Ok(())
                        }
                        Self::AxisY => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            Ok(())
                        }
                        Self::AxisZ => {
                            *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                            Ok(())
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for AxisConfig {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => Self::AxisX,
                            1u32 => Self::AxisY,
                            2u32 => Self::AxisZ,
                            discrim => {
                                return ::anyhow::__private::Err(
                                    ::anyhow::Error::msg({
                                        let res = ::alloc::fmt::format(
                                            format_args!("unexpected discriminant: {0}", discrim),
                                        );
                                        res
                                    }),
                                );
                            }
                        },
                    )
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                    if true {
                        if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                            )
                        }
                    }
                    let discrim = bytes[0];
                    let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                    let payload = &bytes[payload_offset..];
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => Self::AxisX,
                            1u8 => Self::AxisY,
                            2u8 => Self::AxisZ,
                            discrim => {
                                return ::anyhow::__private::Err(
                                    ::anyhow::Error::msg({
                                        let res = ::alloc::fmt::format(
                                            format_args!("unexpected discriminant: {0}", discrim),
                                        );
                                        res
                                    }),
                                );
                            }
                        },
                    )
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerAxisConfig {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadAxisConfig,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for LowerAxisConfig {
                    #[inline]
                    fn clone(&self) -> LowerAxisConfig {
                        let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                        let _: ::core::clone::AssertParamIsClone<LowerPayloadAxisConfig>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for LowerAxisConfig {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadAxisConfig {
                    AxisX: [wasmtime::ValRaw; 0],
                    AxisY: [wasmtime::ValRaw; 0],
                    AxisZ: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl ::core::clone::Clone for LowerPayloadAxisConfig {
                    #[inline]
                    fn clone(&self) -> LowerPayloadAxisConfig {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl ::core::marker::Copy for LowerPayloadAxisConfig {}
                unsafe impl wasmtime::component::ComponentType for AxisConfig {
                    type Lower = LowerAxisConfig;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_enum(
                            ty,
                            types,
                            &["axis-x", "axis-y", "axis-z"],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[None, None, None],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for AxisConfig {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[None, None, None];
                }
            };
            impl core::fmt::Debug for AxisConfig {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        AxisConfig::AxisX => f.debug_tuple("AxisConfig::AxisX").finish(),
                        AxisConfig::AxisY => f.debug_tuple("AxisConfig::AxisY").finish(),
                        AxisConfig::AxisZ => f.debug_tuple("AxisConfig::AxisZ").finish(),
                    }
                }
            }
            const _: () = {
                if !(1 == <AxisConfig as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <AxisConfig as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(1 == <AxisConfig as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <AxisConfig as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            #[component(record)]
            pub struct AxisRemap {
                #[component(name = "x")]
                pub x: AxisConfig,
                #[component(name = "y")]
                pub y: AxisConfig,
                #[component(name = "z")]
                pub z: AxisConfig,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for AxisRemap {}
            #[automatically_derived]
            impl ::core::clone::Clone for AxisRemap {
                #[inline]
                fn clone(&self) -> AxisRemap {
                    let _: ::core::clone::AssertParamIsClone<AxisConfig>;
                    *self
                }
            }
            unsafe impl wasmtime::component::Lower for AxisRemap {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.x,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).x)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.y,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).y)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.z,
                        cx,
                        ty.fields[2usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).z)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.x,
                        cx,
                        ty.fields[0usize].ty,
                        <AxisConfig as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.y,
                        cx,
                        ty.fields[1usize].ty,
                        <AxisConfig as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.z,
                        cx,
                        ty.fields[2usize].ty,
                        <AxisConfig as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for AxisRemap {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        x: <AxisConfig as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.x,
                        )?,
                        y: <AxisConfig as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.y,
                        )?,
                        z: <AxisConfig as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[2usize].ty,
                            &src.z,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        x: <AxisConfig as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<AxisConfig as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<AxisConfig as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        y: <AxisConfig as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<AxisConfig as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<AxisConfig as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        z: <AxisConfig as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[2usize].ty,
                            &bytes[<AxisConfig as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<AxisConfig as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerAxisRemap<T0: Copy, T1: Copy, T2: Copy> {
                    x: T0,
                    y: T1,
                    z: T2,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                    T2: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerAxisRemap<T0, T1, T2> {
                    #[inline]
                    fn clone(&self) -> LowerAxisRemap<T0, T1, T2> {
                        LowerAxisRemap {
                            x: ::core::clone::Clone::clone(&self.x),
                            y: ::core::clone::Clone::clone(&self.y),
                            z: ::core::clone::Clone::clone(&self.z),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                    T2: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerAxisRemap<T0, T1, T2> {}
                unsafe impl wasmtime::component::ComponentType for AxisRemap {
                    type Lower = LowerAxisRemap<
                        <AxisConfig as wasmtime::component::ComponentType>::Lower,
                        <AxisConfig as wasmtime::component::ComponentType>::Lower,
                        <AxisConfig as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <AxisConfig as wasmtime::component::ComponentType>::ABI,
                            <AxisConfig as wasmtime::component::ComponentType>::ABI,
                            <AxisConfig as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "x",
                                    <AxisConfig as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "y",
                                    <AxisConfig as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "z",
                                    <AxisConfig as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for AxisRemap {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("AxisRemap")
                        .field("x", &self.x)
                        .field("y", &self.y)
                        .field("z", &self.z)
                        .finish()
                }
            }
            const _: () = {
                if !(3 == <AxisRemap as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 3 == <AxisRemap as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(1 == <AxisRemap as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <AxisRemap as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            #[component(enum)]
            pub enum PowerMode {
                #[component(name = "NORMAL")]
                Normal,
                #[component(name = "LOW-POWER")]
                LowPower,
                #[component(name = "SUSPEND")]
                Suspend,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for PowerMode {
                #[inline]
                fn clone(&self) -> PowerMode {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for PowerMode {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PowerMode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PowerMode {
                #[inline]
                fn eq(&self, other: &PowerMode) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralEq for PowerMode {}
            #[automatically_derived]
            impl ::core::cmp::Eq for PowerMode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            unsafe impl wasmtime::component::Lower for PowerMode {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::Normal => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(0u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).Normal)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::LowPower => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(1u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).LowPower)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Suspend => {
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::component::__internal::MaybeUninitExt;
                                        let m: &mut std::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).tag)
                                    }
                                }
                            }
                                .write(wasmtime::ValRaw::u32(2u32));
                            unsafe {
                                wasmtime::component::__internal::lower_payload(
                                    {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = dst;
                                                m.map(|p| &raw mut (*p).payload)
                                            }
                                        }
                                    },
                                    |payload| {
                                        #[allow(unused_unsafe)]
                                        {
                                            unsafe {
                                                use ::wasmtime::component::__internal::MaybeUninitExt;
                                                let m: &mut std::mem::MaybeUninit<_> = payload;
                                                m.map(|p| &raw mut (*p).Suspend)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                    }
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    match self {
                        Self::Normal => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            Ok(())
                        }
                        Self::LowPower => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Suspend => {
                            *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                            Ok(())
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for PowerMode {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => Self::Normal,
                            1u32 => Self::LowPower,
                            2u32 => Self::Suspend,
                            discrim => {
                                return ::anyhow::__private::Err(
                                    ::anyhow::Error::msg({
                                        let res = ::alloc::fmt::format(
                                            format_args!("unexpected discriminant: {0}", discrim),
                                        );
                                        res
                                    }),
                                );
                            }
                        },
                    )
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                    if true {
                        if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                            )
                        }
                    }
                    let discrim = bytes[0];
                    let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                    let payload = &bytes[payload_offset..];
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Enum(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => Self::Normal,
                            1u8 => Self::LowPower,
                            2u8 => Self::Suspend,
                            discrim => {
                                return ::anyhow::__private::Err(
                                    ::anyhow::Error::msg({
                                        let res = ::alloc::fmt::format(
                                            format_args!("unexpected discriminant: {0}", discrim),
                                        );
                                        res
                                    }),
                                );
                            }
                        },
                    )
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerPowerMode {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadPowerMode,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for LowerPowerMode {
                    #[inline]
                    fn clone(&self) -> LowerPowerMode {
                        let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                        let _: ::core::clone::AssertParamIsClone<LowerPayloadPowerMode>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for LowerPowerMode {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadPowerMode {
                    Normal: [wasmtime::ValRaw; 0],
                    LowPower: [wasmtime::ValRaw; 0],
                    Suspend: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl ::core::clone::Clone for LowerPayloadPowerMode {
                    #[inline]
                    fn clone(&self) -> LowerPayloadPowerMode {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl ::core::marker::Copy for LowerPayloadPowerMode {}
                unsafe impl wasmtime::component::ComponentType for PowerMode {
                    type Lower = LowerPowerMode;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_enum(
                            ty,
                            types,
                            &["NORMAL", "LOW-POWER", "SUSPEND"],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[None, None, None],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for PowerMode {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[None, None, None];
                }
            };
            impl core::fmt::Debug for PowerMode {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        PowerMode::Normal => f.debug_tuple("PowerMode::Normal").finish(),
                        PowerMode::LowPower => {
                            f.debug_tuple("PowerMode::LowPower").finish()
                        }
                        PowerMode::Suspend => {
                            f.debug_tuple("PowerMode::Suspend").finish()
                        }
                    }
                }
            }
            const _: () = {
                if !(1 == <PowerMode as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <PowerMode as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(1 == <PowerMode as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <PowerMode as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// name into HW map including alternate address for IMU TODO
            /// Subset of underlying BNO055 interface focusing on that which we need first
            /// Still need axis remapping and axis sign mapping
            pub use super::super::super::Imu as Imu;
            pub trait HostImu {
                fn calibrate(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                ) -> wasmtime::Result<Result<CalibrationData, AccessError>>;
                fn set_calibration(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                    data: CalibrationData,
                ) -> wasmtime::Result<Result<(), AccessError>>;
                /// TODO: should this be part of the API which makes it very BNO055 specific? or more general and include this in the hardware map?
                fn set_external_crystal(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                    enabled: bool,
                ) -> wasmtime::Result<Result<(), AccessError>>;
                fn set_axis_remap(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                    remap: AxisRemap,
                ) -> wasmtime::Result<Result<(), AccessError>>;
                fn axis_remap(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                ) -> wasmtime::Result<Result<AxisRemap, AccessError>>;
                fn set_axis_signs(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                    signs: AxisSigns,
                ) -> wasmtime::Result<Result<(), AccessError>>;
                fn axis_signs(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                ) -> wasmtime::Result<Result<AxisSigns, AccessError>>;
                fn set_power_mode(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                    mode: PowerMode,
                ) -> wasmtime::Result<Result<(), AccessError>>;
                fn quaternion(
                    &mut self,
                    self_: wasmtime::component::Resource<Imu>,
                ) -> wasmtime::Result<Result<Quaternion, AccessError>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Imu>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostImu {
                fn imu_named(
                    &mut self,
                    name: String,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<Imu>, AccessError>,
                >;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasm-robotics:robotics/imus@0.1.0")?;
                inst.resource(
                    "imu",
                    wasmtime::component::ResourceType::host::<Imu>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostImu::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "imu-named",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (String,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::imu_named(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.calibrate",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Imu>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::calibrate(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.set-calibration",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (wasmtime::component::Resource<Imu>, CalibrationData)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::set_calibration(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.set-external-crystal",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Imu>, bool)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::set_external_crystal(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.set-axis-remap",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Imu>, AxisRemap)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::set_axis_remap(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.axis-remap",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Imu>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::axis_remap(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.set-axis-signs",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Imu>, AxisSigns)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::set_axis_signs(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.axis-signs",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Imu>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::axis_signs(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.set-power-mode",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Imu>, PowerMode)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::set_power_mode(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]imu.quaternion",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Imu>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostImu::quaternion(host, arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
}
pub mod exports {
    pub mod wasm_robotics {
        pub mod robotics {
            #[allow(clippy::all)]
            pub mod run {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub struct Guest {
                    run: wasmtime::component::Func,
                }
                impl Guest {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Guest> {
                        let run = *__exports.typed_func::<(u32,), ()>("run")?.func();
                        Ok(Guest { run })
                    }
                    pub fn call_run<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: u32,
                    ) -> wasmtime::Result<()> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (u32,),
                                (),
                            >::new_unchecked(self.run)
                        };
                        let () = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(())
                    }
                }
            }
        }
    }
}
const _: &str = "package wasm-robotics:robotics@0.1.0;\n\nuse types;\nuse imus;\n\n/// The `run` function takes the expected hardware resources as parameters.\ninterface run {\n    run: func(loop-count: u32);\n}\n\n\nworld imu-hc {\n    import imus;\n}\n\nworld imu-client {\n    import imus;\n    export run;\n}\n\nworld bp-hal {\n    import delay;\n    import i2c;\n}\n";
const _: &str = "/// Embedded APIs.\n///\n/// These APIs are based on [embedded-hal].\n///\n/// TODO?\n///  - embedded-hal-bus: Sharing SPI and I2C buses\n///  - embedded-can: Controller Area Network (CAN)\n///\n/// [embedded-hal]: https://crates.io/crates/embedded-hal\npackage wasm-robotics:robotics@0.1.0;\n\n/// Inter-Integrated Circuit (I\u{b2}C).\ninterface i2c {\n    /// An address value, in either 7-bit or 10-bit form, depending on the device.\n    type address = u16;\n\n    /// Operation errors.\n    variant error-code {\n        /// Bus error occurred. e.g. A START or a STOP condition is detected and\n        /// is not located after a multiple of 9 SCL clock pulses.\n        bus,\n\n        /// The arbitration was lost, e.g. electrical problems with the clock signal.\n        arbitration-loss,\n\n        /// A bus operation was not acknowledged, e.g. due to the addressed\n        /// device not being available on the bus or the device not being ready\n        /// to process requests at the moment.\n        no-acknowledge(no-acknowledge-source),\n\n        /// The peripheral receive buffer was overrun.\n        overrun,\n\n        /// A different error occurred.\n        other,\n\n        /// resource not available\n        not-found\n    }\n\n    /// No-acknowledge error source.\n    ///\n    /// In cases where it is possible, a device should indicate if a no\n    /// acknowledge response was received to an address versus a no acknowledge\n    /// to a data byte. Where it is not possible to differentiate, Unknown\n    /// should be indicated.\n    enum no-acknowledge-source {\n        /// The device did not acknowledge its address. The device may be\n        /// missing.\n        address,\n\n        /// The device did not acknowledge the data. It may not be ready to\n        /// process requests at the moment.\n        data,\n\n        /// Either the device did not acknowledge its address or the data, but\n        /// it is unknown which.\n        unknown,\n    }\n\n    /// An operation used by the `transaction` method.\n    variant operation {\n        /// Read the give number of bytes.\n        read(u64),\n\n        /// Write the given bytes.\n        write(list<u8>)\n    }\n\n    i2c-named: func(identifier: string) -> result<i2c, error-code>;\n\n    resource i2c {\n        constructor(identifier: string);\n        /// Execute the provided `operation`s on the I\u{b2}C bus.\n        transaction: func(\n            address: address,\n            operations: list<operation>\n        ) -> result<list<list<u8>>, error-code>;\n\n        /// Reads `len` bytes from address `address`.\n        read: func(address: address, len: u64) -> result<list<u8>, error-code>;\n\n        /// Writes bytes to target with address `address`.\n        write: func(address: address, data: list<u8>) -> result<_, error-code>;\n\n        /// Writes bytes to address `address` and then reads `read-len` bytes\n        /// in a single transaction.\n        write-read: func(\n           address: address,\n           write: list<u8>,\n           read-len: u64,\n        ) -> result<list<u8>, error-code>;\n    }\n}\n\n/// Digital I/O, for example GPIO pins.\n// interface digital {\n//     /// Operation errors.\n//     enum error-code {\n//         /// An error occurred.\n//         other,\n//     }\n\n//     /// Digital output pin state.\n//     enum pin-state {\n//         low,\n//         high,\n//     }\n\n//     /// Single digital input pin.\n//     resource input-pin {\n//         /// Is the input pin low?\n//         is-low: func() -> result<bool, error-code>;\n\n//         /// Is the input pin high?\n//         is-high: func() -> result<bool, error-code>;\n\n//         /// Wait until the pin is high. If it is already high, resolve\n//         /// immediately.\n//         wait-for-high: func() -> result<_, error-code>;\n\n//         /// Wait until the pin is low. If it is already low, resolve\n//         /// immediately.\n//         wait-for-low: func() -> result<_, error-code>;\n\n//         /// Wait for the pin to undergo a transition from low to high.\n//         ///\n//         /// If the pin is already high, this does *not* resolve immediately,\n//         /// it\u{2019}ll wait for the pin to go low and then high again.\n//         wait-for-rising-edge: func() -> result<_, error-code>;\n\n//         /// Wait for the pin to undergo a transition from high to low.\n//         ///\n//         /// If the pin is already low, this does *not* return immediately,\n//         /// it\u{2019}ll wait for the pin to go high and then low again.\n//         wait-for-falling-edge: func() -> result<_, error-code>;\n\n//         /// Wait for the pin to undergo any transition, i.e low to high OR high\n//         /// to low.\n//         wait-for-any-edge: func() -> result<_, error-code>;\n//     }\n\n//     /// Single digital input pin.\n//     resource output-pin {\n//         /// Drives the pin low.\n//         set-low: func() -> result<_, error-code>;\n\n//         /// Drives the pin high.\n//         set-high: func() -> result<_, error-code>;\n\n//         /// Drives the pin high or low depending on the provided value.\n//         set-state: func(state: pin-state) -> result<_, error-code>;\n//     }\n\n//     /// Push-pull output pin that can read its output state.\n//     resource stateful-output-pin {\n//         /// Is the pin in drive high mode?\n//         is-set-high: func() -> result<bool, error-code>;\n\n//         /// Is the pin in drive low mode?\n//         is-set-low: func() -> result<bool, error-code>;\n\n//         /// Toggle pin output.\n//         toggle: func() -> result<_, error-code>;\n//     }\n// }\n\n/// Delays.\ninterface delay {\n    // constructor \n    /// Delay with up to nanosecond precision.\n    resource delay {\n        // JAS - added new to support resource creation\n        constructor();\n        /// Pauses execution for at minimum `ns` nanoseconds. Pause can be\n        /// longer if the implementation requires it due to precision/timing\n        /// issues.\n        delay-ns: func(ns: u32);\n    }\n}\n\n/// Pulse Width Modulation (PWM).\n// interface pwm {\n//     /// Operation errors.\n//     enum error-code {\n//         /// An error occurred.\n//         other,\n//     }\n\n//     /// Single PWM channel / pin.\n//     resource set-duty-cycle {\n//         /// Get the maximum duty cycle value.\n//         ///\n//         /// This value corresponds to a 100% duty cycle.\n//         max-duty-cycle: func() -> u16;\n\n//         /// Set the duty cycle to `duty / max_duty`.\n//         ///\n//         /// Traps if the duty cycle value is greater than the maximum duty\n//         /// cycle value, as reported by `max-duty-cycle`.\n//         ///\n//         /// Passing the value 0 turns the duty cycle to always inactive.\n//         /// Passing the value returned by `max-duty-cycle` sets the duty cycle\n//         /// to always acctive.\n//         set-duty-cycle: func(duty: u16) -> result<_, error-code>;\n//     }\n// }\n\n// /// Serial Peripheral Interface (SPI) controller mode.\n// ///\n// /// This specifiation follows [OSHWA\'s recommended terminology].\n// ///\n// /// [OSHWA\'s recommended terminology]: https://www.oshwa.org/a-resolution-to-redefine-spi-signal-names/\n// interface spi {\n//     /// SPI mode.\n//     record mode {\n//         /// Clock polarity.\n//         polarity: polarity,\n\n//         // Clock phase.\n//         phase: phase,\n//     }\n\n//     /// Clock polarity.\n//     enum polarity {\n//         /// Clock signal low when idle.\n//         idle-low,\n\n//         /// Clock signal high when idle.\n//         idle-high,\n//     }\n\n//     /// Clock phase.\n//     enum phase {\n//         /// Data in \u{201c}captured\u{201d} on the first clock transition.\n//         capture-on-first-transition,\n\n//         /// Data in \u{201c}captured\u{201d} on the second clock transition.\n//         capture-on-second-transition,\n//     }\n\n//     /// SPI error kind.\n//     enum error-code {\n//         /// The peripheral receive buffer was overrun.\n//         overrun,\n\n//         /// Multiple devices on the SPI bus are trying to drive the chip\n//         /// select pin.\n//         mode-fault,\n\n//         /// Received data does not conform to the peripheral configuration.\n//         frame-format,\n\n//         /// An error occurred while asserting or deasserting the\n//         /// Chip Select pin.\n//         chip-select-fault,\n\n//         /// A different error occurred.\n//         other,\n//     }\n\n//     /// Word size.\n//     ///\n//     /// TODO: Support up to `u16` word sizes?\n//     type word = u8;\n\n//     /// SPI transaction operation.\n//     ///\n//     /// This allows composition of SPI operations into a single bus transaction.\n//     variant operation {\n//         /// Read data.\n//         read(u64),\n\n//         /// Write data from the provided list, discarding read data.\n//         write(list<word>),\n\n//         /// Read data, while writing data from the buffer.\n//         transfer(tuple<u64, list<word>>),\n\n//         /// Delay for at least the specified number of nanoseconds.\n//         delay-ns(u32),\n//     }\n\n//     /// Helper for CPOL = 0, CPHA = 0.\n//     mode0: func() -> mode;\n\n//     /// Helper for CPOL = 0, CPHA = 1.\n//     mode1: func() -> mode;\n\n//     /// Helper for CPOL = 1, CPHA = 0.\n//     mode2: func() -> mode;\n\n//     /// Helper for CPOL = 1, CPHA = 1.\n//     mode3: func() -> mode;\n\n//     /// SPI bus.\n//     ///\n//     /// `bus` represents exclusive ownership over the whole SPI bus, with\n//     /// serial clock (SCK), peripheral in/controller out (PICO), and\n//     /// peripheral out/controller in (POCI) pins.\n//     resource bus {\n//         /// Read words from the peripheral.\n//         ///\n//         /// The word value sent on PICO during reading is\n//         /// implementation-defined, typically 0x00, 0xFF, or configurable.\n//         ///\n//         /// Implementations are allowed to return before the operation is complete.\n//         read: func(len: u64) -> result<list<word>, error-code>;\n\n//         /// Write `words` to the peripheral, ignoring all the incoming words.\n//         ///\n//         /// Implementations are allowed to return before the operation is complete.\n//         write: func(words: list<word>) -> result<_, error-code>;\n\n//         /// Write and read simultaneously. `write` is written to the peripheral\n//         /// on PICO and words received on POCI are returned.\n//         ///\n//         /// It is allowed for `read-len` and `write`\'s length to be different,\n//         /// even zero length. The transfer runs for `max(read-len, write.len())`\n//         /// words. If `read-len` is shorter, incoming words after `read-len` has\n//         /// been filled will be discarded. If `write` is shorter, the value of\n//         /// words sent in PICO after all `write` has been sent is\n//         /// implementation-defined, typically `0x00`, `0xFF`, or configurable.\n//         ///\n//         /// Implementations are allowed to return before the operation is complete.\n//         transfer: func(\n//             read-len: u64,\n//             write: list<word>\n//         ) -> result<list<word>, error-code>;\n\n//         /// Wait until all operations have completed and the bus is idle.\n//         flush: func() -> result<_, error-code>;\n//     }\n\n//     /// SPI device.\n//     ///\n//     /// `device` represents ownership over a single SPI device on a (possibly\n//     /// shared) bus, selected with a CS (Chip Select) pin.\n//     resource device {\n//         /// Perform a transaction against the device.\n//         ///\n//         /// - Locks the bus\n//         /// - Asserts the CS (Chip Select) pin.\n//         /// - Performs all the operations.\n//         /// - Flushes the bus.\n//         /// - Deasserts the CS pin.\n//         /// - Unlocks the bus.\n//         ///\n//         /// The locking mechanism is implementation-defined. The only\n//         /// requirement is it must prevent two transactions from executing\n//         /// concurrently against the same bus. Examples of implementations are:\n//         /// critical sections, blocking mutexes, returning an error or\n//         /// panicking if the bus is already busy. On bus errors the\n//         /// implementation should try to deassert CS. If an error occurs while\n//         /// deasserting CS the bus error should take priority as the return\n//         /// value.\n//         transaction: func(\n//             operations: list<operation>\n//         ) -> result<list<list<word>>, error-code>;\n\n//         /// Do a read within a transaction.\n//         read: func(len: u64) -> result<list<word>, error-code>;\n\n//         /// Do a write within a transaction.\n//         write: func(buf: list<word>) -> result<_, error-code>;\n\n//         /// Do a transfer within a transaction.\n//         transfer: func(\n//             read-len: u64,\n//             write: list<word>\n//         ) -> result<list<word>, error-code>;\n//     }\n// }\n";
const _: &str = "package wasm-robotics:robotics@0.1.0;\n\ninterface imus {\n    record calibration-data {\n        acc-offset-x-lsb: u8,\n        acc-offset-x-msb: u8,\n        acc-offset-y-lsb: u8,\n        acc-offset-y-msb: u8,\n        acc-offset-z-lsb: u8,\n        acc-offset-z-msb: u8,\n        mag-offset-x-lsb: u8,\n        mag-offset-x-msb: u8,\n        mag-offset-y-lsb: u8,\n        mag-offset-y-msb: u8,\n        mag-offset-z-lsb: u8,\n        mag-offset-z-msb: u8,\n        gyr-offset-x-lsb: u8,\n        gyr-offset-x-msb: u8,\n        gyr-offset-y-lsb: u8,\n        gyr-offset-y-msb: u8,\n        gyr-offset-z-lsb: u8,\n        gyr-offset-z-msb: u8,\n        acc-radius-lsb: u8,\n        acc-radius-msb: u8,\n        mag-radius-lsb: u8,\n        mag-radius-msb: u8,   \n    }\n\n    record axis-signs {\n        x-negative: bool,\n        y-negative: bool,\n        z-negative: bool\n    }\n\n    enum axis-config {\n        axis-x,\n        axis-y,\n        axis-z\n    }\n\n    record axis-remap {\n        x: axis-config,\n        y: axis-config,\n        z: axis-config\n    }\n\n    enum power-mode {\n        NORMAL,\n        LOW-POWER,\n        SUSPEND\n    }\n\n    use types.{access-error,  quaternion};\n    imu-named: func(name: string) -> result<imu, access-error>; // name into HW map including alternate address for IMU TODO\n    // Subset of underlying BNO055 interface focusing on that which we need first\n    // Still need axis remapping and axis sign mapping\n    resource imu {\n        calibrate: func() -> result<calibration-data, access-error>;\n        set-calibration: func(data: calibration-data) -> result<_, access-error>;\n        // TODO: should this be part of the API which makes it very BNO055 specific? or more general and include this in the hardware map?\n        set-external-crystal: func(enabled: bool) -> result<_, access-error>;\n        /// \n        set-axis-remap: func(remap: axis-remap) -> result<_, access-error>;\n        axis-remap: func() -> result<axis-remap, access-error>;\n        set-axis-signs: func(signs: axis-signs) -> result<_, access-error>;\n        axis-signs: func() -> result<axis-signs, access-error>;\n        set-power-mode: func(mode: power-mode) -> result<_, access-error>;\n        quaternion: func() -> result<quaternion, access-error> ;  \n    }\n}\n";
const _: &str = "package wasm-robotics:robotics@0.1.0;\n\ninterface types {\n    variant access-error {\n        named-resource-not-found(string),\n        hardware-access-error(string)\n    }\n    \n    record quaternion {\n        x: f32,\n        y: f32,\n        z: f32,\n        w: f32,\n    }\n}\n\n\n";
pub struct Imu {
    bno055: bno055::Bno055<I2cdev>,
    name: String,
}
pub fn build_imu(name: &str) -> Result<Imu> {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut delay = Delay {};
    let mut imu = Bno055::new(dev).with_alternative_address();
    imu.init(&mut delay).expect("An error occurred while building the IMU");
    imu.set_mode(BNO055OperationMode::NDOF, &mut delay)
        .expect("An error occurred while setting the IMU mode");
    Ok(Imu {
        name: name.to_string(),
        bno055: imu,
    })
}
impl std::fmt::Debug for Imu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(" imu: {0:?}", self.name))
    }
}
impl imus::Host for ImuHC {
    fn imu_named(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Imu>, AccessError>> {
        Ok(
            self
                .imus
                .push(
                    build_imu(&name)
                        .map_err(|e| {
                            NamedResourceNotFound({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failure creating imu named {1} {0:?}",
                                        e,
                                        name,
                                    ),
                                );
                                res
                            })
                        })?,
                )
                .map_err(|e| {
                    NamedResourceNotFound({
                        let res = ::alloc::fmt::format(
                            format_args!("Failure creating imu named {1} {0:?}", e, name),
                        );
                        res
                    })
                }),
        )
    }
}
pub struct ImuHC {
    pub imus: wasmtime::component::ResourceTable,
}
#[automatically_derived]
impl ::core::fmt::Debug for ImuHC {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "ImuHC",
            "imus",
            &&self.imus,
        )
    }
}
#[automatically_derived]
impl ::core::default::Default for ImuHC {
    #[inline]
    fn default() -> ImuHC {
        ImuHC {
            imus: ::core::default::Default::default(),
        }
    }
}
impl imus::HostImu for ImuHC {
    fn calibrate(
        &mut self,
        imu_resource: component::Resource<imus::Imu>,
    ) -> std::result::Result<
        std::result::Result<
            CalibrationData,
            wasm_robotics::robotics::types::AccessError,
        >,
        anyhow::Error,
    > {
        let mut delay = Delay {};
        let imu = self.imus.get_mut(&imu_resource).expect("imu not found");
        let mut status = imu.bno055.get_calibration_status().unwrap();
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event crates/imu/src/lib.rs:87",
                        "sample_imu",
                        ::tracing::Level::DEBUG,
                        ::core::option::Option::Some("crates/imu/src/lib.rs"),
                        ::core::option::Option::Some(87u32),
                        ::core::option::Option::Some("sample_imu"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("- About to begin BNO055 IMU calibration...")
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        while !imu.bno055.is_fully_calibrated().unwrap() {
            status = imu.bno055.get_calibration_status().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(1000));
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event crates/imu/src/lib.rs:91",
                            "sample_imu",
                            ::tracing::Level::DEBUG,
                            ::core::option::Option::Some("crates/imu/src/lib.rs"),
                            ::core::option::Option::Some(91u32),
                            ::core::option::Option::Some("sample_imu"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::DEBUG
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::core::iter::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::core::option::Option::Some(
                                            &format_args!("Calibration status: {0:?}", status)
                                                as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
        }
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event crates/imu/src/lib.rs:93",
                        "sample_imu",
                        ::tracing::Level::DEBUG,
                        ::core::option::Option::Some("crates/imu/src/lib.rs"),
                        ::core::option::Option::Some(93u32),
                        ::core::option::Option::Some("sample_imu"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!(
                                            "The IMU\'s calibration status is: {0:?}",
                                            status,
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        match imu.bno055.calibration_profile(&mut delay) {
            Ok(data) => Ok(Ok(data.into())),
            Err(_) => ::core::panicking::panic("not yet implemented"),
        }
    }
    fn set_calibration(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        data: CalibrationData,
    ) -> std::result::Result<
        std::result::Result<(), wasm_robotics::robotics::types::AccessError>,
        anyhow::Error,
    > {
        let mut delay = Delay {};
        match self
            .imus
            .get_mut(&imu_resource)
            .expect("imu not found")
            .bno055
            .set_calibration_profile(data.into(), &mut delay)
        {
            Ok(_) => Ok(Ok(())),
            Err(_) => ::core::panicking::panic("not yet implemented"),
        }
    }
    fn quaternion(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<
        std::result::Result<Quaternion, wasm_robotics::robotics::types::AccessError>,
    > {
        Ok(
            self
                .imus
                .get_mut(&imu_resource)
                .expect("imu not found")
                .bno055
                .quaternion()
                .map_err(|e| HardwareAccessError({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                    res
                }))
                .map(|f| Ok(f.into()))?,
        )
    }
    fn drop(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<()> {
        let _ = self.imus.delete(imu_resource);
        Ok(())
    }
    /// TODO: should this be part of the API which makes it very BNO055 specific? or more general and include this in the hardware map?
    fn set_external_crystal(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        enabled: bool,
    ) -> wasmtime::Result<
        std::result::Result<(), wasm_robotics::robotics::types::AccessError>,
    > {
        let mut delay = Delay {};
        Ok(
            self
                .imus
                .get_mut(&imu_resource)
                .expect("imu not found")
                .bno055
                .set_external_crystal(enabled, &mut delay)
                .map_err(|e| HardwareAccessError({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                    res
                }))
                .map(|_| Ok(()))?,
        )
    }
    fn set_axis_remap(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        remap: AxisRemap,
    ) -> wasmtime::Result<Result<(), AccessError>> {
        Ok(
            self
                .imus
                .get_mut(&imu_resource)
                .expect("imu not found")
                .bno055
                .set_axis_remap(remap.into())
                .map_err(|e| HardwareAccessError({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                    res
                }))
                .map(|_| Ok(()))?,
        )
    }
    fn axis_remap(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<Result<AxisRemap, AccessError>> {
        Ok(
            self
                .imus
                .get_mut(&imu_resource)
                .expect("imu not found")
                .bno055
                .axis_remap()
                .map_err(|e| HardwareAccessError({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                    res
                }))
                .map(|f: BNO055AxisRemap| Ok(f.into()))?,
        )
    }
    fn set_axis_signs(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        signs: AxisSigns,
    ) -> wasmtime::Result<Result<(), AccessError>> {
        Ok(
            self
                .imus
                .get_mut(&imu_resource)
                .expect("imu not found")
                .bno055
                .set_axis_sign(signs.into())
                .map_err(|e| HardwareAccessError({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                    res
                }))
                .map(|_| Ok(()))?,
        )
    }
    fn axis_signs(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
    ) -> wasmtime::Result<Result<AxisSigns, AccessError>> {
        Ok(
            self
                .imus
                .get_mut(&imu_resource)
                .expect("imu not found")
                .bno055
                .axis_sign()
                .map_err(|e| HardwareAccessError({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                    res
                }))
                .map(|f| Ok(f.into()))?,
        )
    }
    fn set_power_mode(
        &mut self,
        imu_resource: wasmtime::component::Resource<HostImu>,
        mode: PowerMode,
    ) -> wasmtime::Result<Result<(), AccessError>> {
        Ok(
            self
                .imus
                .get_mut(&imu_resource)
                .expect("imu not found")
                .bno055
                .set_power_mode(mode.into())
                .map_err(|e| HardwareAccessError({
                    let res = ::alloc::fmt::format(format_args!("{0:?}", e));
                    res
                }))
                .map(|_| Ok(()))?,
        )
    }
}
impl From<mint_Quaternion<f32>> for Quaternion {
    fn from(val: mint_Quaternion<f32>) -> Self {
        Quaternion {
            x: val.v.x,
            y: val.v.y,
            z: val.v.z,
            w: val.s,
        }
    }
}
impl From<BNO055Calibration> for CalibrationData {
    fn from(val: BNO055Calibration) -> Self {
        CalibrationData {
            acc_offset_x_lsb: val.acc_offset_x_lsb,
            acc_offset_x_msb: val.acc_offset_x_msb,
            acc_offset_y_lsb: val.acc_offset_y_lsb,
            acc_offset_y_msb: val.acc_offset_y_msb,
            acc_offset_z_lsb: val.acc_offset_z_lsb,
            acc_offset_z_msb: val.acc_offset_z_msb,
            mag_offset_x_lsb: val.mag_offset_x_lsb,
            mag_offset_x_msb: val.mag_offset_x_msb,
            mag_offset_y_lsb: val.mag_offset_y_lsb,
            mag_offset_y_msb: val.mag_offset_y_msb,
            mag_offset_z_lsb: val.mag_offset_z_lsb,
            mag_offset_z_msb: val.mag_offset_z_msb,
            gyr_offset_x_lsb: val.gyr_offset_x_lsb,
            gyr_offset_x_msb: val.gyr_offset_x_msb,
            gyr_offset_y_lsb: val.gyr_offset_y_lsb,
            gyr_offset_y_msb: val.gyr_offset_y_msb,
            gyr_offset_z_lsb: val.gyr_offset_z_lsb,
            gyr_offset_z_msb: val.gyr_offset_z_msb,
            acc_radius_lsb: val.acc_radius_lsb,
            acc_radius_msb: val.acc_radius_msb,
            mag_radius_lsb: val.mag_radius_lsb,
            mag_radius_msb: val.mag_radius_msb,
        }
    }
}
impl From<CalibrationData> for BNO055Calibration {
    fn from(val: CalibrationData) -> Self {
        BNO055Calibration {
            acc_offset_x_lsb: val.acc_offset_x_lsb,
            acc_offset_x_msb: val.acc_offset_x_msb,
            acc_offset_y_lsb: val.acc_offset_y_lsb,
            acc_offset_y_msb: val.acc_offset_y_msb,
            acc_offset_z_lsb: val.acc_offset_z_lsb,
            acc_offset_z_msb: val.acc_offset_z_msb,
            mag_offset_x_lsb: val.mag_offset_x_lsb,
            mag_offset_x_msb: val.mag_offset_x_msb,
            mag_offset_y_lsb: val.mag_offset_y_lsb,
            mag_offset_y_msb: val.mag_offset_y_msb,
            mag_offset_z_lsb: val.mag_offset_z_lsb,
            mag_offset_z_msb: val.mag_offset_z_msb,
            gyr_offset_x_lsb: val.gyr_offset_x_lsb,
            gyr_offset_x_msb: val.gyr_offset_x_msb,
            gyr_offset_y_lsb: val.gyr_offset_y_lsb,
            gyr_offset_y_msb: val.gyr_offset_y_msb,
            gyr_offset_z_lsb: val.gyr_offset_z_lsb,
            gyr_offset_z_msb: val.gyr_offset_z_msb,
            acc_radius_lsb: val.acc_radius_lsb,
            acc_radius_msb: val.acc_radius_msb,
            mag_radius_lsb: val.mag_radius_lsb,
            mag_radius_msb: val.mag_radius_msb,
        }
    }
}
impl From<AxisSigns> for BNO055AxisSign {
    fn from(val: AxisSigns) -> Self {
        let mut value: BNO055AxisSign = BNO055AxisSign::empty();
        if val.x_negative {
            value |= BNO055AxisSign::X_NEGATIVE;
        }
        if val.y_negative {
            value |= BNO055AxisSign::Y_NEGATIVE;
        }
        if val.z_negative {
            value |= BNO055AxisSign::Z_NEGATIVE;
        }
        value
    }
}
impl From<BNO055AxisSign> for AxisSigns {
    fn from(val: BNO055AxisSign) -> Self {
        let value: u8 = val.bits();
        AxisSigns {
            x_negative: (value & BNO055AxisSign::X_NEGATIVE.bits()) != 0,
            y_negative: (value & BNO055AxisSign::Y_NEGATIVE.bits()) != 0,
            z_negative: (value & BNO055AxisSign::Z_NEGATIVE.bits()) != 0,
        }
    }
}
impl From<PowerMode> for BNO055PowerMode {
    fn from(val: PowerMode) -> Self {
        match val {
            PowerMode::Normal => BNO055PowerMode::NORMAL,
            PowerMode::LowPower => BNO055PowerMode::LOW_POWER,
            PowerMode::Suspend => BNO055PowerMode::SUSPEND,
        }
    }
}
impl From<AxisRemap> for BNO055AxisRemap {
    fn from(_val: AxisRemap) -> Self {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event crates/imu/src/lib.rs:348",
                        "sample_imu",
                        ::tracing::Level::INFO,
                        ::core::option::Option::Some("crates/imu/src/lib.rs"),
                        ::core::option::Option::Some(348u32),
                        ::core::option::Option::Some("sample_imu"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("Incompletely implemented!!! ") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        BNO055AxisRemap::builder()
            .swap_x_with(BNO055AxisConfig::AXIS_AS_Y)
            .build()
            .expect("axis remap")
    }
}
impl From<BNO055AxisRemap> for AxisRemap {
    fn from(val: BNO055AxisRemap) -> Self {
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event crates/imu/src/lib.rs:358",
                        "sample_imu",
                        ::tracing::Level::INFO,
                        ::core::option::Option::Some("crates/imu/src/lib.rs"),
                        ::core::option::Option::Some(358u32),
                        ::core::option::Option::Some("sample_imu"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("Incompletely implemented!!! ") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        AxisRemap {
            x: val.x().into(),
            y: val.y().into(),
            z: val.z().into(),
        }
    }
}
impl From<BNO055AxisConfig> for AxisConfig {
    fn from(val: BNO055AxisConfig) -> Self {
        match val {
            BNO055AxisConfig::AXIS_AS_X => AxisConfig::AxisX,
            BNO055AxisConfig::AXIS_AS_Y => AxisConfig::AxisY,
            BNO055AxisConfig::AXIS_AS_Z => AxisConfig::AxisZ,
            _ => ::core::panicking::panic("not yet implemented"),
        }
    }
}
