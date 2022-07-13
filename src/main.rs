use async_graphql::{self, MergedObject, Object};
use std::fmt::Display;

pub fn main() {
    println!("update async-graphql to 4.0.4 to see the SimpleObject error");
}

// This fails to compile when running on version 4.0.4
#[derive(MergedObject, Default)]
pub struct Query<E: GqlError>(QueryA<E>, QueryB<E>);

pub trait GqlError: std::fmt::Display + Send + Sync + Default + 'static {
    fn custom<T: Into<String>>(v: T) -> Self;
}

#[derive(Default, Clone, Debug)]
pub struct Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl GqlError for Error {
    fn custom<T: Into<String>>(v: T) -> Self {
        todo!()
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

#[Object]
impl<E: GqlError> QueryA<E> {
    async fn version(&self) -> async_graphql::Result<String> {
        Ok("0.1.0".into())
    }
}

#[Object]
impl<E: GqlError> QueryB<E> {
    async fn hello(&self) -> async_graphql::Result<String> {
        Ok("hello!".into())
    }
}

// #[cfg(feature = "sub")]
mod subscription {
    use crate::GqlError;
    use async_graphql::{
        futures_util::{self, Stream},
        MergedSubscription,
    };

    #[derive(Default, MergedSubscription)]
    pub struct Subscription<E: GqlError>(SubscriptionA<E>, SubscriptionB<E>);

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

    #[async_graphql::Subscription]
    impl<E: GqlError> SubscriptionA<E> {
        async fn events1(&self) -> impl Stream<Item = i32> {
            futures_util::stream::iter(0..10)
        }
    }

    #[async_graphql::Subscription]
    impl<E: GqlError> SubscriptionB<E> {
        async fn events2(&self) -> impl Stream<Item = i32> {
            futures_util::stream::iter(0..10)
        }
    }
}
