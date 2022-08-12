//! # Basic electrical formulas and conventions.
//! 
//! This is going to hold a series of functions and structure for basic electrical simulations.
//! See the notes area for more information.
//! Also note that crate "uom" calculates a lot of the basics.

//! ## Small series of fundamental formulas and types.

use uom::si::f64::*;

/// Helper function to quickly create seconds.
#[inline(always)]
pub fn nseconds(seconds: f64) -> Time {
    Time::new::<uom::si::time::second>(seconds)
}

/// Calculates the current of a given point given the amount of charge and time.
/// This thing is entirely redundant, but shows off uom pretty well.
#[inline(always)]
pub fn calc_current(charge: ElectricCharge, time: Time) -> ElectricCurrent {
    charge / time
}