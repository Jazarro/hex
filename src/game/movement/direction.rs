use std::ops::Not;

use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq)]
pub struct Direction2D {
    pub x: Direction1D,
    pub y: Direction1D,
}

impl Direction2D {
    pub fn from_input(left: bool, right: bool, down: bool, up: bool) -> Self {
        Direction2D {
            x: Direction1D::from_input(left, right),
            y: Direction1D::from_input(down, up),
        }
    }

    pub fn new(signum_x: f32, signum_y: f32) -> Self {
        Direction2D {
            x: Direction1D::new(signum_x),
            y: Direction1D::new(signum_y),
        }
    }
    pub fn from(x: Direction1D, y: Direction1D) -> Self {
        Direction2D { x, y }
    }

    pub fn is_opposite(&self, other: &Direction2D) -> bool {
        self.x.is_opposite(&other.x) || self.y.is_opposite(&other.y)
    }

    pub fn is_neutral(&self) -> bool {
        self.x == Direction1D::Neutral && self.y == Direction1D::Neutral
    }

    pub fn signum(&self) -> Vec2 {
        Vec2::new(self.x.signum(), self.y.signum())
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq)]
pub enum Direction1D {
    Negative,
    Positive,
    Neutral,
}

impl Direction1D {
    pub fn from_input(negative: bool, positive: bool) -> Self {
        if negative ^ positive {
            if negative {
                Direction1D::Negative
            } else {
                Direction1D::Positive
            }
        } else {
            Direction1D::Neutral
        }
    }
    pub fn new(signum: f32) -> Self {
        if signum.abs() <= f32::EPSILON {
            Direction1D::Neutral
        } else if signum.is_sign_positive() {
            Direction1D::Positive
        } else {
            Direction1D::Negative
        }
    }
    pub fn is_opposite(&self, other: &Direction1D) -> bool {
        (*self == Direction1D::Negative && *other == Direction1D::Positive)
            || (*self == Direction1D::Positive && *other == Direction1D::Negative)
    }
    pub fn is_positive(&self) -> bool {
        self == &Direction1D::Positive
    }
    pub fn is_negative(&self) -> bool {
        self == &Direction1D::Negative
    }
    pub fn is_neutral(&self) -> bool {
        self == &Direction1D::Neutral
    }

    pub fn aligns_with(&self, direction: f32) -> bool {
        let other = Direction1D::new(direction);
        self != &Direction1D::Neutral && self == &other
    }
    pub fn signum(&self) -> f32 {
        match self {
            Direction1D::Positive => 1.,
            Direction1D::Negative => -1.,
            Direction1D::Neutral => 0.,
        }
    }
    pub fn signum_i(&self) -> i32 {
        match self {
            Direction1D::Positive => 1,
            Direction1D::Negative => -1,
            Direction1D::Neutral => 0,
        }
    }
}

impl Default for Direction1D {
    fn default() -> Self {
        Direction1D::Neutral
    }
}

impl Not for Direction1D {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction1D::Negative => Direction1D::Positive,
            Direction1D::Neutral => Direction1D::Neutral,
            Direction1D::Positive => Direction1D::Negative,
        }
    }
}
