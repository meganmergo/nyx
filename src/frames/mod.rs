extern crate nalgebra as na;

use na::Vector6;
use std::fmt::Display;

/// The `Orbit` trait is common to all orbit definitions. Each orbit definition however may define
/// additional constructors. However, these constructors are necessary to ensure that the dynamics
/// module will support propagating specific orbital element defitions.
pub trait Orbit {
    /// Returns the gravitational parameter (mu) associated with this orbit.
    pub fn gm() -> f64;
    /// Returns this orbit as a Vector6 of f64. All orbits can be representated as 6 elements.
    /// This is used for compatibility with the `propagator` module.
    pub fn to_state() -> Vector6<f64>;
    /// Creates a new type from this state. This is use for  compatibility with the `propagator` module.
    // XXX: Should this be an update(s: Vector6)? That way the frame is implied.
    pub fn from_state(s: Vector<f64>) -> Self;
    /// Borrow the frame in which this orbit is defined.
    pub fn frame(&self) -> &Frame;
}

/// The `Frame` trait defines a reference frame.
pub trait Frame: Display{
    /// Returns whether this frame is inertial or not.
    pub fn inertial() -> bool;
}
