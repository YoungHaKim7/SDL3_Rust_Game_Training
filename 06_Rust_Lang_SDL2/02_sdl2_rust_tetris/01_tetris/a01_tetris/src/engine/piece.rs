use cgmath::Vector2;

pub struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

pub enum Rotation {
    N,
    S,
    E,
    W,
}

impl Piece {
    const CELL_COUNT: usize = 4;
    pub fn cells(&self) -> Option<impl Iterator<Item = Vector2<usize>>> {
        self.kind.cells()
    }
}
impl Kind {
    pub const ALL: [Self; 7] = [
        Self::O,
        Self::I,
        Self::T,
        Self::L,
        Self::J,
        Self::S,
        Self::Z,
    ];

    // pub fn cells(&self) -> [Vector2<usize>; Piece::CELL_COUNT] {
    pub fn cells(&self) -> impl IntoIterator<Item = &'static Vector2<isize>> {
        match self {
            Kind::O => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            Kind::I => &[(-1, 0), (0, 0), (1, 0), (2, 0)],
            Kind::T => &[(-1, 0), (0, 0), (1, 0), (0, 1)],
            Kind::L => &[(-1, 0), (0, 0), (1, 0), (1, 1)],
            Kind::J => &[(-1, 1), (-1, 0), (0, 0), (1, 0)],
            Kind::S => &[(-1, 0), (0, 0), (0, 1), (1, 1)],
            Kind::Z => &[(-1, 1), (0, 1), (0, 0), (1, 0)],
        }
        .iter()
        .map(From::from())
    }
}

impl<S> std::ops::Mul<Rotation> for Vector2<S>
where
    S: std::ops::Neg<Output = S>,
{
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        // let Vector2 {x, y}
        match rotation {
            Rotation::N => self,
            Rotation::S => Vector2::new(-self.x, -self.y),
            Rotation::E => Vector2::new(self.y, -self.x),
            Rotation::W => todo!(),
        }
    }
}
