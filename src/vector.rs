#![allow(unused)]

use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

use num_traits::Signed;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub const fn new(data: [T; N]) -> Self {
        Self(data)
    }
}

// The four following impls give some easy accessors to the vector's data.
// Unfortunately, due to `#[feature(generic_const_exprs)]` being currently unstable,
// we cannot give these accessors for all the possible vector sizes.
// So we give these accessors manually for sizes up to 4 (included), with some sadly duplicated code.

impl<T> Vector<T, 1>
where
    T: Copy,
{
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }
}

impl<T> Vector<T, 2>
where
    T: Copy,
{
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }
}

impl<T> Vector<T, 3>
where
    T: Copy,
{
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    pub fn z(&self) -> T {
        self.0[2]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.0[2]
    }
}

impl<T> Vector<T, 4>
where
    T: Copy,
{
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    pub fn z(&self) -> T {
        self.0[2]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.0[2]
    }

    pub fn w(&self) -> T {
        self.0[3]
    }

    pub fn w_mut(&mut self) -> &mut T {
        &mut self.0[3]
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Sum + Mul<Output = T>,
{
    pub fn length(&self) -> T {
        self.0.map(|it| it * it).into_iter().sum::<T>()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Sum + Sub<Output = T> + Signed,
{
    pub fn manhattan_distance(&self, other: Self) -> T {
        self.0.into_iter().zip(other.0.into_iter()).map(|(a, b)| (b - a).abs()).sum::<T>()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Sum + Mul<Output = T> + Div<Output = T>,
{
    pub fn normalize(&self) -> Self {
        *self / self.length()
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.0;
        for (lhs, rhs) in data.iter_mut().zip(rhs.0) {
            *lhs = *lhs + rhs;
        }
        Self(data)
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.0.iter_mut().zip(rhs.0) {
            *lhs += rhs;
        }
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.0;
        for (lhs, rhs) in data.iter_mut().zip(rhs.0) {
            *lhs = *lhs - rhs;
        }
        Self(data)
    }
}

impl<T, const N: usize> SubAssign for Vector<T, N>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.0.iter_mut().zip(rhs.0) {
            *lhs -= rhs;
        }
    }
}

impl<T, const N: usize> Add<T> for Vector<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut data = self.0;
        for value in &mut data {
            *value = *value + rhs;
        }
        Self(data)
    }
}

impl<T, const N: usize> AddAssign<T> for Vector<T, N>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        for value in &mut self.0 {
            *value += rhs;
        }
    }
}

impl<T, const N: usize> Sub<T> for Vector<T, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let mut data = self.0;
        for value in &mut data {
            *value = *value - rhs;
        }
        Self(data)
    }
}

impl<T, const N: usize> SubAssign<T> for Vector<T, N>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        for value in &mut self.0 {
            *value -= rhs;
        }
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut data = self.0;
        for value in &mut data {
            *value = *value * rhs;
        }
        Self(data)
    }
}

impl<T, const N: usize> MulAssign<T> for Vector<T, N>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        for value in &mut self.0 {
            *value *= rhs;
        }
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut data = self.0;
        for value in &mut data {
            *value = *value / rhs;
        }
        Self(data)
    }
}

impl<T, const N: usize> DivAssign<T> for Vector<T, N>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        for value in &mut self.0 {
            *value /= rhs;
        }
    }
}

impl<T, const N: usize> Rem<T> for Vector<T, N>
where
    T: Rem<Output = T> + Copy,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        let mut data = self.0;
        for value in &mut data {
            *value = *value % rhs;
        }
        Self(data)
    }
}

impl<T, const N: usize> RemAssign<T> for Vector<T, N>
where
    T: RemAssign + Copy,
{
    fn rem_assign(&mut self, rhs: T) {
        for value in &mut self.0 {
            *value %= rhs;
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(data: [T; N]) -> Self {
        Self::new(data)
    }
}

impl<T, const N: usize> From<Vector<T, N>> for [T; N] {
    fn from(data: Vector<T, N>) -> Self {
        data.0
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([Default::default(); N])
    }
}

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
