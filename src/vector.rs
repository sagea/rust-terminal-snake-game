use impl_ops::impl_op;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
  pub x: i32,
  pub y: i32,
}
impl Vector {
  pub fn new(x: i32, y: i32) -> Self {
    Vector { x, y }
  }

  pub fn set_x(&self, x: i32) -> Self {
    Vector { x, y: self.y }
  }

  pub fn set_y(&self, y: i32) -> Self {
    Vector { x: self.x, y: y }
  }
}

impl From<(i32, i32)> for Vector {
  fn from((x, y): (i32, i32)) -> Self {
    Vector { x, y }
  }
}

impl From<(u16, u16)> for Vector {
  fn from((x, y): (u16, u16)) -> Self {
    Vector { x: x as i32, y: y as i32 }
  }
}

#[macro_export]
macro_rules! v {
    ($x: expr) => {
        v!($x, $x)
    };
    ($x: expr, $y: expr) => {
      Vector::new($x, $y)
    }
}
type TupleVec = (i32, i32);
impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector { Vector { x: a.x + b.x, y: a.y + b.y } });
impl_op_ex!(+ |a: &Vector, b: &TupleVec| -> Vector { Vector { x: a.x + b.0, y: a.y + b.1 } });
impl_op_ex!(+ |a: &TupleVec, b: &Vector| -> Vector { Vector { x: a.0 + b.x, y: a.1 + b.y } });

impl_op_ex!(- |a: &Vector, b: &Vector| -> Vector { Vector { x: a.x - b.x, y: a.y - b.y } });
impl_op_ex!(- |a: &Vector, b: &TupleVec| -> Vector { Vector { x: a.x - b.0, y: a.y - b.1 } });
impl_op_ex!(- |a: &TupleVec, b: &Vector| -> Vector { Vector { x: a.0 - b.x, y: a.1 - b.y } });


impl_op_ex!(* |a: &Vector, b: &Vector| -> Vector { Vector { x: a.x * b.x, y: a.y * b.y } });
impl_op_ex!(* |a: &Vector, b: &TupleVec| -> Vector { Vector { x: a.x * b.0, y: a.y * b.1 } });
impl_op_ex!(* |a: &TupleVec, b: &Vector| -> Vector { Vector { x: a.0 * b.x, y: a.1 * b.y } });

impl_op_ex!(/ |a: &Vector, b: &Vector| -> Vector { Vector { x: a.x / b.x, y: a.y / b.y } });
impl_op_ex!(/ |a: &Vector, b: &TupleVec| -> Vector { Vector { x: a.x / b.0, y: a.y / b.1 } });
impl_op_ex!(/ |a: &TupleVec, b: &Vector| -> Vector { Vector { x: a.0 / b.x, y: a.1 / b.y } });


