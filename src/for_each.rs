use crate::{Element, Node, Text};
use std::convert::Infallible;
use std::ops::ControlFlow;

// public API for for_each

pub enum WalkResult<T> {
    /// Continue walking.
    Continue,
    /// Skip the children of this [`Element`].
    ///
    /// Is identical to `Continue` for [`Text`] and [`Comment`].
    Skip,
    /// Break walking.
    Break(T),
}

#[allow(async_fn_in_trait)]
pub trait ForEach<'a> {
    fn root(&self) -> &[Node<'a>];

    #[inline]
    fn for_each<F, E, T, U>(&self, f: F) -> Result<ControlFlow<T, ()>, E>
    where
        F: Fn(&Node<'a>) -> U,
        U: Into<Result<WalkResult<T>, E>>,
    {
        inner_for_each(self.root(), &f)
    }

    #[inline]
    async fn for_each_async<F, E, T, U>(&self, f: F) -> Result<ControlFlow<T, ()>, E>
    where
        F: AsyncFn(&Node<'a>) -> U,
        U: Into<Result<WalkResult<T>, E>>,
    {
        inner_for_each_async(self.root(), &f).await
    }

    fn root_mut(&mut self) -> &mut [Node<'a>];

    #[inline]
    fn for_each_mut<F, E, T, U>(&mut self, mut f: F) -> Result<ControlFlow<T, ()>, E>
    where
        F: FnMut(&mut Node<'a>) -> U,
        U: Into<Result<WalkResult<T>, E>>,
    {
        inner_for_each_mut(self.root_mut(), &mut f)
    }

    #[inline]
    async fn for_each_mut_async<F, E, T, U>(&mut self, mut f: F) -> Result<ControlFlow<T, ()>, E>
    where
        F: AsyncFnMut(&mut Node<'a>) -> U,
        U: Into<Result<WalkResult<T>, E>>,
    {
        inner_for_each_mut_async(self.root_mut(), &mut f).await
    }
}

// helpers for the closures

#[inline]
pub fn element<'a, F, E, U, T>(f: F) -> impl Fn(&Node<'a>) -> Result<WalkResult<T>, E>
where
    F: Fn(&Element<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    move |n| {
        if let Node::Element(e) = n {
            f(e).into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

#[inline]
pub fn element_async<'a, F, E, U, T>(f: F) -> impl AsyncFn(&Node<'a>) -> Result<WalkResult<T>, E>
where
    F: AsyncFn(&Element<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    async move |n| {
        if let Node::Element(e) = n {
            f(e).await.into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

#[inline]
pub fn element_mut<'a, F, E, U, T>(
    mut f: F,
) -> impl FnMut(&mut Node<'a>) -> Result<WalkResult<T>, E>
where
    F: FnMut(&mut Element<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    move |n| {
        if let Node::Element(e) = n {
            f(e).into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

#[inline]
pub fn element_mut_async<'a, F, E, U, T>(
    mut f: F,
) -> impl AsyncFnMut(&mut Node<'a>) -> Result<WalkResult<T>, E>
where
    F: AsyncFnMut(&mut Element<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    async move |n| {
        if let Node::Element(e) = n {
            f(e).await.into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

#[inline]
pub fn text<'a, F, E, U, T>(f: F) -> impl Fn(&Node<'a>) -> Result<WalkResult<T>, E>
where
    F: Fn(&Text<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    move |n| {
        if let Node::Text(t) = n {
            f(t).into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

#[inline]
pub fn text_async<'a, F, E, U, T>(f: F) -> impl AsyncFn(&Node<'a>) -> Result<WalkResult<T>, E>
where
    F: AsyncFn(&Text<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    async move |n| {
        if let Node::Text(t) = n {
            f(t).await.into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

#[inline]
pub fn text_mut<'a, F, E, U, T>(mut f: F) -> impl FnMut(&mut Node<'a>) -> Result<WalkResult<T>, E>
where
    F: FnMut(&mut Text<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    move |n| {
        if let Node::Text(t) = n {
            f(t).into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

#[inline]
pub fn text_mut_async<'a, F, E, U, T>(
    mut f: F,
) -> impl AsyncFnMut(&mut Node<'a>) -> Result<WalkResult<T>, E>
where
    F: AsyncFnMut(&mut Text<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    async move |n| {
        if let Node::Text(t) = n {
            f(t).await.into()
        } else {
            Ok(WalkResult::Continue)
        }
    }
}

// to allow to use WalkResult<T> OR Result<WalkResult<T>, E>

impl<T> From<WalkResult<T>> for Result<WalkResult<T>, Infallible> {
    #[inline]
    fn from(val: WalkResult<T>) -> Self {
        Ok(val)
    }
}

// to signal not used error/break

pub trait NoErr<T> {
    fn no_err(self) -> ControlFlow<T, ()>;
}

impl<T> NoErr<T> for Result<ControlFlow<T, ()>, Infallible> {
    #[inline]
    fn no_err(self) -> ControlFlow<T, ()> {
        match self {
            Ok(t) => t,
            Err(_) => unreachable!(),
        }
    }
}

pub trait NoBrk {
    fn no_brk(self) -> ();
}

impl NoBrk for ControlFlow<(), ()> {
    #[inline]
    fn no_brk(self) {}
}

// for_each functions

fn inner_for_each<'a, F, E, T, U>(tree: &[Node<'a>], f: &F) -> Result<ControlFlow<T, ()>, E>
where
    F: Fn(&Node<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    for n in tree {
        match f(n).into()? {
            WalkResult::Continue => {
                if let Node::Element(e) = n {
                    match inner_for_each(e.children.as_slice(), f)? {
                        ControlFlow::Continue(()) => {}
                        ControlFlow::Break(t) => return Ok(ControlFlow::Break(t)),
                    }
                }
            }
            WalkResult::Skip => (),
            WalkResult::Break(t) => return Ok(ControlFlow::Break(t)),
        }
    }
    Ok(ControlFlow::Continue(()))
}

fn inner_for_each_mut<'a, F, E, T, U>(
    tree: &mut [Node<'a>],
    f: &mut F,
) -> Result<ControlFlow<T, ()>, E>
where
    F: FnMut(&mut Node<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    for n in tree {
        match f(n).into()? {
            WalkResult::Continue => {
                if let Node::Element(e) = n {
                    match inner_for_each_mut(e.children.as_mut_slice(), f)? {
                        ControlFlow::Continue(()) => {}
                        ControlFlow::Break(t) => return Ok(ControlFlow::Break(t)),
                    }
                }
            }
            WalkResult::Skip => (),
            WalkResult::Break(t) => return Ok(ControlFlow::Break(t)),
        }
    }
    Ok(ControlFlow::Continue(()))
}

async fn inner_for_each_async<'a, F, E, T, U>(
    tree: &[Node<'a>],
    f: &F,
) -> Result<ControlFlow<T, ()>, E>
where
    F: AsyncFn(&Node<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    for n in tree {
        match f(n).await.into()? {
            WalkResult::Continue => {
                if let Node::Element(e) = n {
                    match Box::pin(inner_for_each_async(e.children.as_slice(), f)).await? {
                        ControlFlow::Continue(()) => {}
                        ControlFlow::Break(t) => return Ok(ControlFlow::Break(t)),
                    }
                }
            }
            WalkResult::Skip => (),
            WalkResult::Break(t) => return Ok(ControlFlow::Break(t)),
        }
    }
    Ok(ControlFlow::Continue(()))
}

async fn inner_for_each_mut_async<'a, F, E, T, U>(
    tree: &mut [Node<'a>],
    f: &mut F,
) -> Result<ControlFlow<T, ()>, E>
where
    F: AsyncFnMut(&mut Node<'a>) -> U,
    U: Into<Result<WalkResult<T>, E>>,
{
    for n in tree {
        match f(n).await.into()? {
            WalkResult::Continue => {
                if let Node::Element(e) = n {
                    match Box::pin(inner_for_each_mut_async(e.children.as_mut_slice(), f)).await? {
                        ControlFlow::Continue(()) => {}
                        ControlFlow::Break(t) => return Ok(ControlFlow::Break(t)),
                    }
                }
            }
            WalkResult::Skip => (),
            WalkResult::Break(t) => return Ok(ControlFlow::Break(t)),
        }
    }
    Ok(ControlFlow::Continue(()))
}
