/*
 * File:    betterpd4diff.rs
 * Brief:   TODO
 *
 * Copyright (C) 2024 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

/*!
 * TODO rustdoc for this file here
*/

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use common::*;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO (also pub(crate) use the_macro statements here too)

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

//TODO includes "type"-defs, structs, enums, unions, etc

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

fn main() {
    for line in ParsedLineIterator::from_path("/home/jzj/Downloads/example_traces/golden_bubble_sort.trace").unwrap() {
        println!("{:?}", line);
    }
    todo!()
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO