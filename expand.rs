#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use async_graphql::{self, MergedObject, Object};
use std::fmt::Display;
pub fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["hello world\n"], &[]));
    };
}
pub struct Query<E: GqlError>(QueryA<E>, QueryB<E>);
#[allow(clippy::all, clippy::pedantic)]
impl<E: GqlError> async_graphql::resolver_utils::ContainerType for Query<E> {
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn resolve_field<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::Context<'life2>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = async_graphql::ServerResult<
                        ::std::option::Option<async_graphql::Value>,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                async_graphql::ServerResult<::std::option::Option<async_graphql::Value>>,
            > {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let __ret: async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> = {
                async_graphql::MergedObject(
                    &__self.1,
                    async_graphql::MergedObject(&__self.0, async_graphql::MergedObjectTail),
                )
                .resolve_field(ctx)
                .await
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn find_entity<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::Context<'life2>,
        params: &'life3 async_graphql::Value,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = async_graphql::ServerResult<
                        ::std::option::Option<async_graphql::Value>,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                async_graphql::ServerResult<::std::option::Option<async_graphql::Value>>,
            > {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let params = params;
            let __ret: async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> = {
                async_graphql::MergedObject(
                    &__self.1,
                    async_graphql::MergedObject(&__self.0, async_graphql::MergedObjectTail),
                )
                .find_entity(ctx, params)
                .await
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
#[allow(clippy::all, clippy::pedantic)]
impl<E: GqlError> async_graphql::OutputType for Query<E> {
    fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
        ::std::borrow::Cow::Borrowed("Query")
    }
    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> ::std::string::String {
        registry.create_output_type::<Self, _>(
            async_graphql::registry::MetaTypeId::Object,
            |registry| {
                let mut fields = ::std::default::Default::default();
                let mut cache_control = ::std::default::Default::default();
                if let async_graphql::registry::MetaType::Object {
                    fields: obj_fields,
                    cache_control: obj_cache_control,
                    ..
                } = registry.create_fake_output_type::<async_graphql::MergedObject<
                    QueryB<E>,
                    async_graphql::MergedObject<QueryA<E>, async_graphql::MergedObjectTail>,
                >>() {
                    fields = obj_fields;
                    cache_control = obj_cache_control;
                }
                async_graphql::registry::MetaType::Object {
                    name: ::std::borrow::ToOwned::to_owned("Query"),
                    description: ::std::option::Option::None,
                    fields,
                    cache_control,
                    extends: false,
                    keys: ::std::option::Option::None,
                    visible: ::std::option::Option::None,
                    is_subscription: false,
                    rust_typename: ::std::any::type_name::<Self>(),
                }
            },
        )
    }
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn resolve<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::ContextSelectionSet<'life2>,
        _field: &'life3 async_graphql::Positioned<async_graphql::parser::types::Field>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = async_graphql::ServerResult<async_graphql::Value>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<async_graphql::ServerResult<async_graphql::Value>>
            {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let _field = _field;
            let __ret: async_graphql::ServerResult<async_graphql::Value> =
                { async_graphql::resolver_utils::resolve_container(ctx, __self).await };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
impl<E: GqlError> async_graphql::ObjectType for Query<E> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<E: ::core::default::Default + GqlError> ::core::default::Default for Query<E> {
    #[inline]
    fn default() -> Query<E> {
        Query(
            ::core::default::Default::default(),
            ::core::default::Default::default(),
        )
    }
}
pub trait GqlError: std::fmt::Display + Send + Sync + Default + 'static {
    fn custom<T: Into<String>>(v: T) -> Self;
}
pub struct Error {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Error {
    #[inline]
    fn default() -> Error {
        Error {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Error {
    #[inline]
    fn clone(&self) -> Error {
        Error {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Error")
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::core::panicking::panic("not yet implemented")
    }
}
impl GqlError for Error {
    fn custom<T: Into<String>>(v: T) -> Self {
        ::core::panicking::panic("not yet implemented")
    }
}
pub struct QueryA<E: GqlError> {
    err_marker: std::marker::PhantomData<E>,
}
pub struct QueryB<E: GqlError> {
    err_marker: std::marker::PhantomData<E>,
}
impl<E: GqlError> Default for QueryA<E> {
    fn default() -> Self {
        Self {
            err_marker: std::marker::PhantomData::default(),
        }
    }
}
impl<E: GqlError> Default for QueryB<E> {
    fn default() -> Self {
        Self {
            err_marker: std::marker::PhantomData::default(),
        }
    }
}
impl<E: GqlError> QueryA<E> {
    async fn version(&self, _: &async_graphql::Context<'_>) -> async_graphql::Result<String> {
        Ok("0.1.0".into())
    }
}
#[allow(clippy::all, clippy::pedantic, clippy::suspicious_else_formatting)]
#[allow(unused_braces, unused_variables, unused_parens, unused_mut)]
impl<E: GqlError> async_graphql::resolver_utils::ContainerType for QueryA<E> {
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn resolve_field<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::Context<'life2>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = async_graphql::ServerResult<
                        ::std::option::Option<async_graphql::Value>,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                async_graphql::ServerResult<::std::option::Option<async_graphql::Value>>,
            > {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let __ret: async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> = {
                if ctx.item.node.name.node == "version" {
                    let f = async move {
                        {
                            let res = __self.version(ctx).await;
                            res.map_err(|err| {
                                ::std::convert::Into::<async_graphql::Error>::into(err)
                                    .into_server_error(ctx.item.pos)
                            })
                        }
                    };
                    let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
                    let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
                    return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                        .await
                        .map(::std::option::Option::Some);
                }
                ::std::result::Result::Ok(::std::option::Option::None)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn find_entity<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::Context<'life2>,
        params: &'life3 async_graphql::Value,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = async_graphql::ServerResult<
                        ::std::option::Option<async_graphql::Value>,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                async_graphql::ServerResult<::std::option::Option<async_graphql::Value>>,
            > {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let params = params;
            let __ret: async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> = {
                let params = match params {
                    async_graphql::Value::Object(params) => params,
                    _ => return ::std::result::Result::Ok(::std::option::Option::None),
                };
                let typename =
                    if let ::std::option::Option::Some(async_graphql::Value::String(typename)) =
                        params.get("__typename")
                    {
                        typename
                    } else {
                        return ::std::result::Result::Err(async_graphql::ServerError::new(
                            r#""__typename" must be an existing string."#,
                            ::std::option::Option::Some(ctx.item.pos),
                        ));
                    };
                ::std::result::Result::Ok(::std::option::Option::None)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
#[allow(clippy::all, clippy::pedantic)]
impl<E: GqlError> async_graphql::OutputType for QueryA<E> {
    fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
        ::std::borrow::Cow::Borrowed("QueryA")
    }
    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> ::std::string::String {
        let ty = registry.create_output_type::<Self, _>(
            async_graphql::registry::MetaTypeId::Object,
            |registry| async_graphql::registry::MetaType::Object {
                name: ::std::borrow::Cow::into_owned(::std::borrow::Cow::Borrowed("QueryA")),
                description: ::std::option::Option::None,
                fields: {
                    let mut fields = async_graphql::indexmap::IndexMap::new();
                    fields.insert(
                        ::std::borrow::ToOwned::to_owned("version"),
                        async_graphql::registry::MetaField {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            description: ::std::option::Option::None,
                            args: {
                                let mut args = async_graphql::indexmap::IndexMap::new();
                                args
                            },
                            ty: <String as async_graphql::OutputType>::create_type_info(registry),
                            deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                            cache_control: async_graphql::CacheControl {
                                public: true,
                                max_age: 0usize,
                            },
                            external: false,
                            provides: ::std::option::Option::None,
                            requires: ::std::option::Option::None,
                            visible: ::std::option::Option::None,
                            compute_complexity: ::std::option::Option::None,
                        },
                    );
                    fields
                },
                cache_control: async_graphql::CacheControl {
                    public: true,
                    max_age: 0usize,
                },
                extends: false,
                keys: ::std::option::Option::None,
                visible: ::std::option::Option::None,
                is_subscription: false,
                rust_typename: ::std::any::type_name::<Self>(),
            },
        );
        ty
    }
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn resolve<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::ContextSelectionSet<'life2>,
        _field: &'life3 async_graphql::Positioned<async_graphql::parser::types::Field>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = async_graphql::ServerResult<async_graphql::Value>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<async_graphql::ServerResult<async_graphql::Value>>
            {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let _field = _field;
            let __ret: async_graphql::ServerResult<async_graphql::Value> =
                { async_graphql::resolver_utils::resolve_container(ctx, __self).await };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
impl<E: GqlError> async_graphql::ObjectType for QueryA<E> {}
impl<E: GqlError> QueryB<E> {
    async fn hello(&self, _: &async_graphql::Context<'_>) -> async_graphql::Result<String> {
        Ok("hello!".into())
    }
}
#[allow(clippy::all, clippy::pedantic, clippy::suspicious_else_formatting)]
#[allow(unused_braces, unused_variables, unused_parens, unused_mut)]
impl<E: GqlError> async_graphql::resolver_utils::ContainerType for QueryB<E> {
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn resolve_field<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::Context<'life2>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = async_graphql::ServerResult<
                        ::std::option::Option<async_graphql::Value>,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                async_graphql::ServerResult<::std::option::Option<async_graphql::Value>>,
            > {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let __ret: async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> = {
                if ctx.item.node.name.node == "hello" {
                    let f = async move {
                        {
                            let res = __self.hello(ctx).await;
                            res.map_err(|err| {
                                ::std::convert::Into::<async_graphql::Error>::into(err)
                                    .into_server_error(ctx.item.pos)
                            })
                        }
                    };
                    let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
                    let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
                    return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                        .await
                        .map(::std::option::Option::Some);
                }
                ::std::result::Result::Ok(::std::option::Option::None)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn find_entity<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::Context<'life2>,
        params: &'life3 async_graphql::Value,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = async_graphql::ServerResult<
                        ::std::option::Option<async_graphql::Value>,
                    >,
                > + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                async_graphql::ServerResult<::std::option::Option<async_graphql::Value>>,
            > {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let params = params;
            let __ret: async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> = {
                let params = match params {
                    async_graphql::Value::Object(params) => params,
                    _ => return ::std::result::Result::Ok(::std::option::Option::None),
                };
                let typename =
                    if let ::std::option::Option::Some(async_graphql::Value::String(typename)) =
                        params.get("__typename")
                    {
                        typename
                    } else {
                        return ::std::result::Result::Err(async_graphql::ServerError::new(
                            r#""__typename" must be an existing string."#,
                            ::std::option::Option::Some(ctx.item.pos),
                        ));
                    };
                ::std::result::Result::Ok(::std::option::Option::None)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
#[allow(clippy::all, clippy::pedantic)]
impl<E: GqlError> async_graphql::OutputType for QueryB<E> {
    fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
        ::std::borrow::Cow::Borrowed("QueryB")
    }
    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> ::std::string::String {
        let ty = registry.create_output_type::<Self, _>(
            async_graphql::registry::MetaTypeId::Object,
            |registry| async_graphql::registry::MetaType::Object {
                name: ::std::borrow::Cow::into_owned(::std::borrow::Cow::Borrowed("QueryB")),
                description: ::std::option::Option::None,
                fields: {
                    let mut fields = async_graphql::indexmap::IndexMap::new();
                    fields.insert(
                        ::std::borrow::ToOwned::to_owned("hello"),
                        async_graphql::registry::MetaField {
                            name: ::std::borrow::ToOwned::to_owned("hello"),
                            description: ::std::option::Option::None,
                            args: {
                                let mut args = async_graphql::indexmap::IndexMap::new();
                                args
                            },
                            ty: <String as async_graphql::OutputType>::create_type_info(registry),
                            deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                            cache_control: async_graphql::CacheControl {
                                public: true,
                                max_age: 0usize,
                            },
                            external: false,
                            provides: ::std::option::Option::None,
                            requires: ::std::option::Option::None,
                            visible: ::std::option::Option::None,
                            compute_complexity: ::std::option::Option::None,
                        },
                    );
                    fields
                },
                cache_control: async_graphql::CacheControl {
                    public: true,
                    max_age: 0usize,
                },
                extends: false,
                keys: ::std::option::Option::None,
                visible: ::std::option::Option::None,
                is_subscription: false,
                rust_typename: ::std::any::type_name::<Self>(),
            },
        );
        ty
    }
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn resolve<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        ctx: &'life1 async_graphql::ContextSelectionSet<'life2>,
        _field: &'life3 async_graphql::Positioned<async_graphql::parser::types::Field>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = async_graphql::ServerResult<async_graphql::Value>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<async_graphql::ServerResult<async_graphql::Value>>
            {
                return __ret;
            }
            let __self = self;
            let ctx = ctx;
            let _field = _field;
            let __ret: async_graphql::ServerResult<async_graphql::Value> =
                { async_graphql::resolver_utils::resolve_container(ctx, __self).await };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
impl<E: GqlError> async_graphql::ObjectType for QueryB<E> {}
mod subscription {
    use crate::GqlError;
    use async_graphql::{
        futures_util::{self, Stream},
        MergedSubscription,
    };
    pub struct Subscription<E: GqlError>(SubscriptionA<E>, SubscriptionB<E>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<E: ::core::default::Default + GqlError> ::core::default::Default for Subscription<E> {
        #[inline]
        fn default() -> Subscription<E> {
            Subscription(
                ::core::default::Default::default(),
                ::core::default::Default::default(),
            )
        }
    }
    #[allow(clippy::all, clippy::pedantic)]
    impl async_graphql::SubscriptionType for Subscription {
        fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
            ::std::borrow::Cow::Borrowed("Subscription")
        }
        fn create_type_info(
            registry: &mut async_graphql::registry::Registry,
        ) -> ::std::string::String {
            registry.create_subscription_type::<Self, _>(|registry| {
                let mut fields = ::std::default::Default::default();
                if let async_graphql::registry::MetaType::Object {
                    fields: obj_fields, ..
                } = registry.create_fake_subscription_type::<async_graphql::MergedObject<
                    SubscriptionB<E>,
                    async_graphql::MergedObject<SubscriptionA<E>, async_graphql::MergedObjectTail>,
                >>() {
                    fields = obj_fields;
                }
                async_graphql::registry::MetaType::Object {
                    name: ::std::borrow::ToOwned::to_owned("Subscription"),
                    description: ::std::option::Option::None,
                    fields,
                    cache_control: ::std::default::Default::default(),
                    extends: false,
                    keys: ::std::option::Option::None,
                    visible: ::std::option::Option::None,
                    is_subscription: true,
                    rust_typename: ::std::any::type_name::<Self>(),
                }
            })
        }
        fn create_field_stream<'__life>(
            &'__life self,
            ctx: &'__life async_graphql::Context<'__life>,
        ) -> ::std::option::Option<
            ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn async_graphql::futures_util::stream::Stream<Item = async_graphql::Response>
                        + ::std::marker::Send
                        + '__life,
                >,
            >,
        > {
            ::std::option::Option::None
                .or_else(|| async_graphql::SubscriptionType::create_field_stream(&self.0, ctx))
                .or_else(|| async_graphql::SubscriptionType::create_field_stream(&self.1, ctx))
        }
    }
    pub struct SubscriptionA<E: GqlError> {
        err_marker: std::marker::PhantomData<E>,
    }
    impl<E: GqlError> Default for SubscriptionA<E> {
        fn default() -> Self {
            Self {
                err_marker: std::marker::PhantomData::default(),
            }
        }
    }
    pub struct SubscriptionB<E: GqlError> {
        err_marker: std::marker::PhantomData<E>,
    }
    impl<E: GqlError> Default for SubscriptionB<E> {
        fn default() -> Self {
            Self {
                err_marker: std::marker::PhantomData::default(),
            }
        }
    }
    impl<E: GqlError> SubscriptionA<E> {
        async fn events1(
            &self,
            _: &async_graphql::Context<'_>,
        ) -> async_graphql::Result<impl Stream<Item = i32>> {
            {
                let value = (move || async move { futures_util::stream::iter(0..10) })().await;
                ::std::result::Result::Ok(value)
            }
        }
    }
    #[allow(clippy::all, clippy::pedantic)]
    #[allow(unused_braces, unused_variables)]
    impl<E: GqlError> async_graphql::SubscriptionType for SubscriptionA<E> {
        fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
            ::std::borrow::Cow::Borrowed("SubscriptionA")
        }
        #[allow(bare_trait_objects)]
        fn create_type_info(
            registry: &mut async_graphql::registry::Registry,
        ) -> ::std::string::String {
            registry . create_subscription_type :: < Self , _ > (| registry | async_graphql :: registry :: MetaType :: Object { name : :: std :: borrow :: ToOwned :: to_owned ("SubscriptionA") , description : :: std :: option :: Option :: None , fields : { let mut fields = async_graphql :: indexmap :: IndexMap :: new () ; fields . insert (:: std :: borrow :: ToOwned :: to_owned ("events1") , async_graphql :: registry :: MetaField { name : :: std :: borrow :: ToOwned :: to_owned ("events1") , description : :: std :: option :: Option :: None , args : { let mut args = async_graphql :: indexmap :: IndexMap :: new () ; args } , ty : < < dyn Stream < Item = i32 > as async_graphql :: futures_util :: stream :: Stream > :: Item as async_graphql :: OutputType > :: create_type_info (registry) , deprecation : async_graphql :: registry :: Deprecation :: NoDeprecated , cache_control : :: std :: default :: Default :: default () , external : false , requires : :: std :: option :: Option :: None , provides : :: std :: option :: Option :: None , visible : :: std :: option :: Option :: None , compute_complexity : :: std :: option :: Option :: None , }) ; fields } , cache_control : :: std :: default :: Default :: default () , extends : false , keys : :: std :: option :: Option :: None , visible : :: std :: option :: Option :: None , is_subscription : true , rust_typename : :: std :: any :: type_name :: < Self > () , })
        }
        fn create_field_stream<'__life>(
            &'__life self,
            ctx: &'__life async_graphql::Context<'_>,
        ) -> ::std::option::Option<
            ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn async_graphql::futures_util::stream::Stream<Item = async_graphql::Response>
                        + ::std::marker::Send
                        + '__life,
                >,
            >,
        > {
            if ctx.item.node.name.node == "events1" {
                let stream = async_graphql::futures_util::stream::TryStreamExt::try_flatten(
                    async_graphql::futures_util::stream::once((move || async move {
                        let field_name =
                            ::std::clone::Clone::clone(&ctx.item.node.response_key().node);
                        let field = ::std::sync::Arc::new(::std::clone::Clone::clone(&ctx.item));
                        let f = async {
                            self.events1(ctx).await.map_err(|err| {
                                ::std::convert::Into::<async_graphql::Error>::into(err)
                                    .into_server_error(ctx.item.pos)
                                    .with_path(<[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            async_graphql::PathSegment::Field(
                                                ::std::borrow::ToOwned::to_owned(&*field_name),
                                            ),
                                        ]),
                                    ))
                            })
                        };
                        let stream = f.await.map_err(|err| ctx.set_error_path(err))?;
                        let pos = ctx.item.pos;
                        let schema_env = ::std::clone::Clone::clone(&ctx.schema_env);
                        let query_env = ::std::clone::Clone::clone(&ctx.query_env);
                        let stream = async_graphql::futures_util::stream::StreamExt::then(
                            stream,
                            {
                                let field_name = ::std::clone::Clone::clone(&field_name);
                                move |msg| {
                                    let schema_env = ::std::clone::Clone::clone(&schema_env);
                                    let query_env = ::std::clone::Clone::clone(&query_env);
                                    let field = ::std::clone::Clone::clone(&field);
                                    let field_name = ::std::clone::Clone::clone(&field_name);
                                    async move {
                                        let ctx_selection_set = query_env.create_context(
                                            &schema_env,
                                            ::std::option::Option::Some(
                                                async_graphql::QueryPathNode {
                                                    parent: ::std::option::Option::None,
                                                    segment: async_graphql::QueryPathSegment::Name(
                                                        &field_name,
                                                    ),
                                                },
                                            ),
                                            &field.node.selection_set,
                                        );
                                        let mut execute_fut = async {
                                            # [allow (bare_trait_objects)] let ri = async_graphql :: extensions :: ResolveInfo { path_node : ctx_selection_set . path_node . as_ref () . unwrap () , parent_type : "SubscriptionA" , return_type : & < < dyn Stream < Item = i32 > as async_graphql :: futures_util :: stream :: Stream > :: Item as async_graphql :: OutputType > :: qualified_type_name () , name : field . node . name . node . as_str () , alias : field . node . alias . as_ref () . map (| alias | alias . node . as_str ()) , } ;
                                            let resolve_fut = async {
                                                async_graphql::OutputType::resolve(
                                                    &msg,
                                                    &ctx_selection_set,
                                                    &*field,
                                                )
                                                .await
                                                .map(::std::option::Option::Some)
                                            };
                                            let mut resolve_fut = resolve_fut;
                                            #[allow(unused_mut)]
                                            let mut resolve_fut = unsafe {
                                                ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                                    &mut resolve_fut,
                                                )
                                            };
                                            let mut resp = query_env
                                                .extensions
                                                .resolve(ri, &mut resolve_fut)
                                                .await
                                                .map(|value| {
                                                    let mut map =
                                                        async_graphql::indexmap::IndexMap::new();
                                                    map.insert(
                                                        ::std::clone::Clone::clone(&field_name),
                                                        value.unwrap_or_default(),
                                                    );
                                                    async_graphql::Response::new(
                                                        async_graphql::Value::Object(map),
                                                    )
                                                })
                                                .unwrap_or_else(|err| {
                                                    async_graphql::Response::from_errors(
                                                        <[_]>::into_vec(
                                                            #[rustc_box]
                                                            ::alloc::boxed::Box::new([err]),
                                                        ),
                                                    )
                                                });
                                            use ::std::iter::Extend;
                                            resp.errors.extend(::std::mem::take(
                                                &mut *query_env.errors.lock().unwrap(),
                                            ));
                                            resp
                                        };
                                        let mut execute_fut = execute_fut;
                                        #[allow(unused_mut)]
                                        let mut execute_fut = unsafe {
                                            ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                                &mut execute_fut,
                                            )
                                        };
                                        ::std::result::Result::Ok(
                                            query_env
                                                .extensions
                                                .execute(
                                                    query_env.operation_name.as_deref(),
                                                    &mut execute_fut,
                                                )
                                                .await,
                                        )
                                    }
                                }
                            },
                        );
                        async_graphql::ServerResult::Ok(stream)
                    })()),
                );
                let stream = async_graphql::futures_util::StreamExt::map(stream, |res| match res {
                    ::std::result::Result::Ok(resp) => resp,
                    ::std::result::Result::Err(err) => {
                        async_graphql::Response::from_errors(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([err]),
                        ))
                    }
                });
                return ::std::option::Option::Some(::std::boxed::Box::pin(stream));
            }
            ::std::option::Option::None
        }
    }
    impl<E: GqlError> SubscriptionB<E> {
        async fn events2(
            &self,
            _: &async_graphql::Context<'_>,
        ) -> async_graphql::Result<impl Stream<Item = i32>> {
            {
                let value = (move || async move { futures_util::stream::iter(0..10) })().await;
                ::std::result::Result::Ok(value)
            }
        }
    }
    #[allow(clippy::all, clippy::pedantic)]
    #[allow(unused_braces, unused_variables)]
    impl<E: GqlError> async_graphql::SubscriptionType for SubscriptionB<E> {
        fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
            ::std::borrow::Cow::Borrowed("SubscriptionB")
        }
        #[allow(bare_trait_objects)]
        fn create_type_info(
            registry: &mut async_graphql::registry::Registry,
        ) -> ::std::string::String {
            registry . create_subscription_type :: < Self , _ > (| registry | async_graphql :: registry :: MetaType :: Object { name : :: std :: borrow :: ToOwned :: to_owned ("SubscriptionB") , description : :: std :: option :: Option :: None , fields : { let mut fields = async_graphql :: indexmap :: IndexMap :: new () ; fields . insert (:: std :: borrow :: ToOwned :: to_owned ("events2") , async_graphql :: registry :: MetaField { name : :: std :: borrow :: ToOwned :: to_owned ("events2") , description : :: std :: option :: Option :: None , args : { let mut args = async_graphql :: indexmap :: IndexMap :: new () ; args } , ty : < < dyn Stream < Item = i32 > as async_graphql :: futures_util :: stream :: Stream > :: Item as async_graphql :: OutputType > :: create_type_info (registry) , deprecation : async_graphql :: registry :: Deprecation :: NoDeprecated , cache_control : :: std :: default :: Default :: default () , external : false , requires : :: std :: option :: Option :: None , provides : :: std :: option :: Option :: None , visible : :: std :: option :: Option :: None , compute_complexity : :: std :: option :: Option :: None , }) ; fields } , cache_control : :: std :: default :: Default :: default () , extends : false , keys : :: std :: option :: Option :: None , visible : :: std :: option :: Option :: None , is_subscription : true , rust_typename : :: std :: any :: type_name :: < Self > () , })
        }
        fn create_field_stream<'__life>(
            &'__life self,
            ctx: &'__life async_graphql::Context<'_>,
        ) -> ::std::option::Option<
            ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn async_graphql::futures_util::stream::Stream<Item = async_graphql::Response>
                        + ::std::marker::Send
                        + '__life,
                >,
            >,
        > {
            if ctx.item.node.name.node == "events2" {
                let stream = async_graphql::futures_util::stream::TryStreamExt::try_flatten(
                    async_graphql::futures_util::stream::once((move || async move {
                        let field_name =
                            ::std::clone::Clone::clone(&ctx.item.node.response_key().node);
                        let field = ::std::sync::Arc::new(::std::clone::Clone::clone(&ctx.item));
                        let f = async {
                            self.events2(ctx).await.map_err(|err| {
                                ::std::convert::Into::<async_graphql::Error>::into(err)
                                    .into_server_error(ctx.item.pos)
                                    .with_path(<[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            async_graphql::PathSegment::Field(
                                                ::std::borrow::ToOwned::to_owned(&*field_name),
                                            ),
                                        ]),
                                    ))
                            })
                        };
                        let stream = f.await.map_err(|err| ctx.set_error_path(err))?;
                        let pos = ctx.item.pos;
                        let schema_env = ::std::clone::Clone::clone(&ctx.schema_env);
                        let query_env = ::std::clone::Clone::clone(&ctx.query_env);
                        let stream = async_graphql::futures_util::stream::StreamExt::then(
                            stream,
                            {
                                let field_name = ::std::clone::Clone::clone(&field_name);
                                move |msg| {
                                    let schema_env = ::std::clone::Clone::clone(&schema_env);
                                    let query_env = ::std::clone::Clone::clone(&query_env);
                                    let field = ::std::clone::Clone::clone(&field);
                                    let field_name = ::std::clone::Clone::clone(&field_name);
                                    async move {
                                        let ctx_selection_set = query_env.create_context(
                                            &schema_env,
                                            ::std::option::Option::Some(
                                                async_graphql::QueryPathNode {
                                                    parent: ::std::option::Option::None,
                                                    segment: async_graphql::QueryPathSegment::Name(
                                                        &field_name,
                                                    ),
                                                },
                                            ),
                                            &field.node.selection_set,
                                        );
                                        let mut execute_fut = async {
                                            # [allow (bare_trait_objects)] let ri = async_graphql :: extensions :: ResolveInfo { path_node : ctx_selection_set . path_node . as_ref () . unwrap () , parent_type : "SubscriptionB" , return_type : & < < dyn Stream < Item = i32 > as async_graphql :: futures_util :: stream :: Stream > :: Item as async_graphql :: OutputType > :: qualified_type_name () , name : field . node . name . node . as_str () , alias : field . node . alias . as_ref () . map (| alias | alias . node . as_str ()) , } ;
                                            let resolve_fut = async {
                                                async_graphql::OutputType::resolve(
                                                    &msg,
                                                    &ctx_selection_set,
                                                    &*field,
                                                )
                                                .await
                                                .map(::std::option::Option::Some)
                                            };
                                            let mut resolve_fut = resolve_fut;
                                            #[allow(unused_mut)]
                                            let mut resolve_fut = unsafe {
                                                ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                                    &mut resolve_fut,
                                                )
                                            };
                                            let mut resp = query_env
                                                .extensions
                                                .resolve(ri, &mut resolve_fut)
                                                .await
                                                .map(|value| {
                                                    let mut map =
                                                        async_graphql::indexmap::IndexMap::new();
                                                    map.insert(
                                                        ::std::clone::Clone::clone(&field_name),
                                                        value.unwrap_or_default(),
                                                    );
                                                    async_graphql::Response::new(
                                                        async_graphql::Value::Object(map),
                                                    )
                                                })
                                                .unwrap_or_else(|err| {
                                                    async_graphql::Response::from_errors(
                                                        <[_]>::into_vec(
                                                            #[rustc_box]
                                                            ::alloc::boxed::Box::new([err]),
                                                        ),
                                                    )
                                                });
                                            use ::std::iter::Extend;
                                            resp.errors.extend(::std::mem::take(
                                                &mut *query_env.errors.lock().unwrap(),
                                            ));
                                            resp
                                        };
                                        let mut execute_fut = execute_fut;
                                        #[allow(unused_mut)]
                                        let mut execute_fut = unsafe {
                                            ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                                &mut execute_fut,
                                            )
                                        };
                                        ::std::result::Result::Ok(
                                            query_env
                                                .extensions
                                                .execute(
                                                    query_env.operation_name.as_deref(),
                                                    &mut execute_fut,
                                                )
                                                .await,
                                        )
                                    }
                                }
                            },
                        );
                        async_graphql::ServerResult::Ok(stream)
                    })()),
                );
                let stream = async_graphql::futures_util::StreamExt::map(stream, |res| match res {
                    ::std::result::Result::Ok(resp) => resp,
                    ::std::result::Result::Err(err) => {
                        async_graphql::Response::from_errors(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([err]),
                        ))
                    }
                });
                return ::std::option::Option::Some(::std::boxed::Box::pin(stream));
            }
            ::std::option::Option::None
        }
    }
}
