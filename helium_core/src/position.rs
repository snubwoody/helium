use crate::size::Size;
use bytemuck::{Pod, Zeroable};
use std::ops::{Add, AddAssign, Mul, MulAssign, SubAssign};
use winit::dpi::PhysicalPosition;

/// The `x` and `y` position of any structure.
///
/// You can add and subtract `Positions`'s to and from each other
/// ```
/// use helium_core::Position;
///
/// let mut position = Position::new(200.0,200.0);
/// position -= Position::unit(50.0);
/// assert_eq!(position.x,150.0);
///
/// position += Position::unit(120.0);
/// assert_eq!(position.y,270.0);
/// ```
///
/// You can also add and subtract arbitrary values to and from `Position`'s
/// ```
/// use helium_core::Position;
///
/// let mut position = Position::new(200.0,200.0);
/// position += 55.0;
/// assert_eq!(position.x,255.0);
///
/// position -= 25.0;
/// assert_eq!(position.y,230.0);
/// ```
///
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Pod, Zeroable)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Translate the position by `x` and `y` amount.
	/// 
    /// # Example
	/// 
    /// ```
    /// use helium_core::Position;
    ///
    /// let mut position = Position::new(0.0,0.0);
    /// position.translate(40.0,100.0);
    ///
    /// assert_eq!(Position::new(40.0,100.0),position);
    /// ```
    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    /// Set the position
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    /// Creates a [`Position`] where both the x and y are
    /// a single value.
    ///
    /// # Example
    /// ```
    /// use helium_core::Position;
    ///
    /// let position = Position::unit(500.0);
    ///
    /// assert_eq!(position.x,position.y);
    /// assert_eq!(position.x,500.0);
    /// ```
    pub fn unit(value: f32) -> Self {
        Self { x: value, y: value }
    }
}

impl Add<Position> for Position{
	type Output = Self;
	fn add(mut self, rhs: Position) -> Self::Output {
		self.x += rhs.x;
		self.y += rhs.y;
		self
	}
}

impl AddAssign<Position> for Position {
	/// Performs the `+=` operation for two [`Position`]'s
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<I> AddAssign<I> for Position
where
    f32: AddAssign<I>,
    I: Copy,
{
    fn add_assign(&mut self, rhs: I) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<I> SubAssign<I> for Position
where
    f32: SubAssign<I>,
    I: Copy,
{
    fn sub_assign(&mut self, rhs: I) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<I> MulAssign<I> for Position
where 
	I: Copy,
	f32: MulAssign<I>
{
	
	fn mul_assign(&mut self, rhs: I) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<I> Mul<I> for Position
where 
	I:Copy,
	f32: MulAssign<I>
{
	type Output = Self;
	fn mul(mut self, rhs: I) -> Self::Output {
		self.x *= rhs;
		self.y *= rhs;
		self
	}
}

impl From<PhysicalPosition<f64>> for Position {
    fn from(position: PhysicalPosition<f64>) -> Self {
        Self {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

/// The bounds of any object that has a [`Size`]
/// and [`Position`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Bounds {
    pub x: [f32; 2],
    pub y: [f32; 2],
}

impl Bounds {
    pub fn new(position: Position, size: Size) -> Self {
        Self {
            x: [position.x, position.x + size.width],
            y: [position.y, position.y + size.height],
        }
    }

    /// Check if a [`Position`] is within the [`Bounds`].
    ///
    /// # Example
    /// ```
    /// use helium_core::{Position,Bounds,Size};
    ///
    /// let size = Size::new(250.0,100.0);
    /// let position = Position::new(10.0,0.0);
    ///
    /// let bounds = Bounds::new(position,size);
    ///
    /// assert!(bounds.within(&Position::new(50.0,45.5)));
    /// assert!(!bounds.within(&Position::new(1550.0,445.5)));
    /// ```
    pub fn within(&self, position: &Position) -> bool {
        // TODO change the name of this to has and move within to position
        if position.x > self.x[0]
            && position.x < self.x[1]
            && position.y > self.y[0]
            && position.y < self.y[1]
        {
            return true;
        }

        false
    }
}
