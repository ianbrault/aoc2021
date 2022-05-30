/*
** src/types/math.rs
*/

use std::ops::{Div, Mul};

macro_rules! bind_els {
    ($self:expr, $a:ident, $b:ident) => {
        let $a = $self.data[0];
        let $b = $self.data[1];
    };
    ($self:expr, $a:ident, $b:ident, $c:ident) => {
        let $a = $self.data[0];
        let $b = $self.data[1];
        let $c = $self.data[2];
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident) => {
        let $a = $self.data[0];
        let $b = $self.data[1];
        let $c = $self.data[2];
        let $d = $self.data[3];
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident) => {
        let $a = $self.data[0];
        let $b = $self.data[1];
        let $c = $self.data[2];
        let $d = $self.data[3];
        let $e = $self.data[4];
        let $f = $self.data[5];
        let $g = $self.data[6];
        let $h = $self.data[7];
        let $i = $self.data[8];
    };
}

pub struct FVector2 {
    pub data: [f64; 2],
}

impl FVector2 {
    pub fn new(a: f64, b: f64) -> Self {
        let data = [a, b];
        Self { data }
    }
}

impl Div<f64> for FVector2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        bind_els!(self, a, b);
        FVector2::new(a / rhs, b / rhs)
    }
}

pub struct FMatrix2x2 {
    data: [f64; 4],
}

impl FMatrix2x2 {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        let data = [a, b, c, d];
        Self { data }
    }

    pub fn determinant(&self) -> f64 {
        bind_els!(&self, a, b, c, d);
        (a * d) - (b * c)
    }

    pub fn solve_system(m: &Self, v: &FVector2) -> FVector2 {
        bind_els!(m, a, b, c, d);
        // note: save the division for last in case of integer division
        let det = m.determinant();
        let m_inv = Self::new(d, -b, -c, a);
        (m_inv * v) / det
    }
}

impl Mul<&FVector2> for FMatrix2x2 {
    type Output = FVector2;

    fn mul(self, rhs: &FVector2) -> Self::Output {
        bind_els!(self, a, b, c, d);
        bind_els!(rhs, e, f);
        FVector2::new((a * e) + (b * f), (c * e) + (d * f))
    }
}
