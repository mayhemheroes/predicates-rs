// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Introspect into the state of a `Predicate`.

use std::borrow;
use std::fmt;
use std::slice;

/// Introspect the state of a `Predicate`.
pub trait PredicateReflection: fmt::Display {
    /// Parameters of the current `Predicate`.
    fn parameters<'a>(&'a self) -> Box<Iterator<Item = Parameter<'a>> + 'a> {
        let params = vec![];
        Box::new(params.into_iter())
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(&'a self) -> Box<Iterator<Item = Child<'a>> + 'a> {
        let params = vec![];
        Box::new(params.into_iter())
    }
}

/// A view of a `Predicate` parameter, provided by reflection.
///
/// ```rust
/// use predicates;
///
/// let param = predicates::reflection::Parameter::new("key", &10);
/// println!("{}", param);
/// ```
pub struct Parameter<'a>(&'a str, &'a fmt::Display);

impl<'a> Parameter<'a> {
    /// Create a new `Parameter`.
    pub fn new(key: &'a str, value: &'a fmt::Display) -> Self {
        Self { 0: key, 1: value }
    }

    /// Access the `Parameter` name.
    pub fn name(&self) -> &str {
        self.0
    }

    /// Access the `Parameter` value.
    pub fn value(&self) -> &fmt::Display {
        self.1
    }
}

impl<'a> fmt::Display for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<'a> fmt::Debug for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}

/// A view of a `Predicate` child, provided by reflection.
pub struct Child<'a>(&'a str, &'a PredicateReflection);

impl<'a> Child<'a> {
    /// Create a new `Predicate` child.
    pub fn new(key: &'a str, value: &'a PredicateReflection) -> Self {
        Self { 0: key, 1: value }
    }

    /// Access the `Child`'s name.
    pub fn name(&self) -> &str {
        self.0
    }

    /// Access the `Child` `Predicate`.
    pub fn value(&self) -> &PredicateReflection {
        self.1
    }
}

impl<'a> fmt::Display for Child<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<'a> fmt::Debug for Child<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}

/// A descriptive explanation for why a predicate failed.
pub struct Case<'a> {
    predicate: Option<&'a PredicateReflection>,
    result: bool,
    products: Vec<Product>,
    children: Vec<Case<'a>>,
}

impl<'a> Case<'a> {
    /// Create a new `Case` describing the result of a `Predicate`.
    pub fn new(predicate: Option<&'a PredicateReflection>, result: bool) -> Self {
        Self {
            predicate,
            result,
            products: Default::default(),
            children: Default::default(),
        }
    }

    /// Add an additional by product to a `Case`.
    pub fn add_product(mut self, product: Product) -> Self {
        self.products.push(product);
        self
    }

    /// Add an additional by product to a `Case`.
    pub fn add_child(mut self, child: Case<'a>) -> Self {
        self.children.push(child);
        self
    }

    /// The `Predicate` that produced this case.
    pub fn predicate(&self) -> Option<&PredicateReflection> {
        self.predicate
    }

    /// The result of this case.
    pub fn result(&self) -> bool {
        self.result
    }

    /// Access the by-products from determining this case.
    pub fn products(&self) -> CaseProducts {
        CaseProducts {
            0: self.products.iter(),
        }
    }

    /// Access the sub-cases.
    pub fn children(&self) -> CaseChildren {
        CaseChildren {
            0: self.children.iter(),
        }
    }
}

impl<'a> fmt::Debug for Case<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let predicate = if let Some(ref predicate) = self.predicate {
            format!("Some({})", predicate)
        } else {
            "None".to_owned()
        };
        f.debug_struct("Case")
            .field("predicate", &predicate)
            .field("result", &self.result)
            .field("products", &self.products)
            .field("children", &self.children)
            .finish()
    }
}

/// Iterator over a `Case`s by-products.
#[derive(Debug, Clone)]
pub struct CaseProducts<'a>(slice::Iter<'a, Product>);

impl<'a> Iterator for CaseProducts<'a> {
    type Item = &'a Product;

    fn next(&mut self) -> Option<&'a Product> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

/// Iterator over a `Case`s sub-cases.
#[derive(Debug, Clone)]
pub struct CaseChildren<'a>(slice::Iter<'a, Case<'a>>);

impl<'a> Iterator for CaseChildren<'a> {
    type Item = &'a Case<'a>;

    fn next(&mut self) -> Option<&'a Case<'a>> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize {
        self.0.count()
    }
}

/// A by-product of a predicate evaluation.
///
/// ```rust
/// use predicates;
///
/// let product = predicates::reflection::Product::new("key", "value");
/// println!("{}", product);
/// let product = predicates::reflection::Product::new(format!("key-{}", 5), 30);
/// println!("{}", product);
/// ```
pub struct Product(borrow::Cow<'static, str>, Box<fmt::Display>);

impl Product {
    /// Create a new `Product`.
    pub fn new<S, D>(key: S, value: D) -> Self
    where
        S: Into<borrow::Cow<'static, str>>,
        D: fmt::Display + 'static,
    {
        Self {
            0: key.into(),
            1: Box::new(value),
        }
    }

    /// Access the `Product` name.
    pub fn name(&self) -> &str {
        self.0.as_ref()
    }

    /// Access the `Product` value.
    pub fn value(&self) -> &fmt::Display {
        &self.1
    }
}

impl<'a> fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl<'a> fmt::Debug for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct DebugAdapter<T>
where
    T: fmt::Debug,
{
    pub(crate) debug: T,
}

impl<T> DebugAdapter<T>
where
    T: fmt::Debug,
{
    pub fn new(debug: T) -> Self {
        Self { debug }
    }
}

impl<T> fmt::Display for DebugAdapter<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.debug)
    }
}

impl<T> fmt::Debug for DebugAdapter<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.debug.fmt(f)
    }
}
