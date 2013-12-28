#[crate_id="foo#0.1"];

/// The '# ' lines should be removed from the output, but the #[deriving] should be
/// retained.
///
/// ```rust
/// # #[deriving(Eq)] // invisible
/// # struct Foo; // invisible
///
/// #[deriving(Eq)] // Bar
/// struct Bar(Foo);
/// let x = Bar(Foo);
/// assert!(x == x);
/// ```
pub fn foo() {}
