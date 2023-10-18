#![doc = include_str!("../README.md")]

//---------------------------------------------------------------------------------------------------- Docs
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//---------------------------------------------------------------------------------------------------- Lints
#![forbid(
    future_incompatible,
    let_underscore,
    break_with_label_and_loop,
    coherence_leak_check,
    deprecated,
    duplicate_macro_attributes,
    exported_private_dependencies,
    for_loops_over_fallibles,
    large_assignments,
    overlapping_range_endpoints,
    private_in_public,
    semicolon_in_expressions_from_macros,
    redundant_semicolons,
    unconditional_recursion,
    unused_allocation,
    unused_doc_comments,
    unused_labels,
    unused_unsafe,
    while_true,
    keyword_idents,
    non_ascii_idents,
    noop_method_call,
    unreachable_pub,
    single_use_lifetimes,
    variant_size_differences,
    unused_mut,
)]
#![deny(
    missing_docs,
    unused_comparisons,
    nonstandard_style,
)]
#![allow(
    unused_braces,
)]

//----------------------------------------------------------------------------------------------------
mod date;
pub use date::*;

mod weekday;
pub use weekday::*;

mod month;
pub use month::*;

mod day;
pub use day::*;

mod macros;