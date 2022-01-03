use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use std::f32::consts::PI;

/// Vector with direction and magnitude, mainly used for velocity
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    /// Direction of the vector in radians
    pub dir: f32,
    /// Magnitude of the vector
    pub mag: f32
}

impl Vector {
    /// Create a new vector from direction and magnitude
    pub fn new(dir: f32, mag: f32) -> Vector {
        Vector {dir, mag}
    }

    /// Create a new zero vector
    pub fn zero() -> Vector {
        Vector {dir: 0.0, mag: 0.0}
    }

    /// Create a new vector from x and y components
    pub fn from_components(x: f32, y: f32) -> Vector {
        Vector {
            mag: (x.powi(2) + y.powi(2)).sqrt(),
            dir: y.atan2(x)
        }
    }

    /// Get the x component of the vector
    pub fn x(&self) -> f32 {
        self.mag * self.dir.cos()
    }

    /// Get the y component of the vector
    pub fn y(&self) -> f32 {
        self.mag * self.dir.sin()
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        let x_mag = self.x() + rhs.x();
        let y_mag = self.y() + rhs.y();
        Vector::from_components(x_mag, y_mag)
    }
}

impl AddAssign<Vector> for Vector {

    fn add_assign(&mut self, rhs: Vector) {
        let x_mag = self.x() + rhs.x();
        let y_mag = self.y() + rhs.y();
        self.dir = y_mag.atan2(x_mag);
        self.mag = (x_mag.powi(2) + y_mag.powi(2)).sqrt();
    }
}


impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(
            self.dir - PI,
            self.mag
        )
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(
            self.dir,
            self.mag * rhs
        )
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(
            self.dir,
            self.mag / rhs
        )
    }
}
