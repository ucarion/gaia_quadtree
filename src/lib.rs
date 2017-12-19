#[derive(PartialEq, Eq, Debug)]
pub struct Tile {
    pub offset: i16,
    pub level: u8,
    pub x: u8,
    pub y: u8,
}

#[derive(PartialEq, Eq, Debug)]
pub enum PositionInParent {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Tile {
    pub fn new_at_origin(level: u8, x: u8, y: u8) -> Tile {
        Tile {
            offset: 0,
            level,
            x,
            y,
        }
    }

    pub fn enclosing_point(level: u8, position: [f32; 2]) -> Tile {
        let offset = (position[0] / 2.0).floor();

        let width = Self::level_width(level);
        let x = (position[0] - offset * 2.0) / width;
        let y = position[1] / width;

        Tile {
            offset: offset as i16,
            level,
            x: x as u8,
            y: y as u8,
        }
    }

    pub fn parent(&self) -> Option<Tile> {
        if self.level == 0 {
            return None;
        }

        Some(Tile {
            offset: self.offset,
            level: self.level - 1,
            x: self.x / 2,
            y: self.y / 2,
        })
    }

    pub fn position_in_parent(&self) -> Option<PositionInParent> {
        if self.level == 0 {
            return None;
        }

        Some(match (self.x % 2 == 0, self.y % 2 == 0) {
            (true, true) => PositionInParent::TopLeft,
            (false, true) => PositionInParent::TopRight,
            (true, false) => PositionInParent::BottomLeft,
            (false, false) => PositionInParent::BottomRight,
        })
    }

    pub fn width(&self) -> f32 {
        Self::level_width(self.level)
    }

    pub fn top_left_position(&self) -> [f32; 2] {
        [
            2.0 * self.offset as f32 + self.x as f32 * self.width(),
            self.y as f32 * self.width(),
        ]
    }

    pub fn bottom_right_position(&self) -> [f32; 2] {
        let top_left = self.top_left_position();
        let width = self.width();

        [
            top_left[0] + width,
            top_left[1] + width,
        ]
    }

    fn level_width(level: u8) -> f32 {
        1.0 / 2.0f32.powi(level as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent() {
        assert_eq!(
            None,
            Tile {
                offset: 0,
                level: 0,
                x: 0,
                y: 0,
            }.parent()
        );

        assert_eq!(
            Some(Tile {
                offset: -123,
                level: 2,
                x: 4,
                y: 2,
            }),
            Tile {
                offset: -123,
                level: 3,
                x: 9,
                y: 4,
            }.parent()
        );
    }

    #[test]
    fn position_in_parent() {
        assert_eq!(
            None,
            Tile {
                offset: 0,
                level: 0,
                x: 0,
                y: 0,
            }.position_in_parent()
        );

        assert_eq!(
            Some(PositionInParent::TopRight),
            Tile {
                offset: -123,
                level: 3,
                x: 9,
                y: 4,
            }.position_in_parent()
        );
    }

    #[test]
    fn width() {
        assert_eq!(
            1.0,
            Tile {
                offset: 0,
                level: 0,
                x: 0,
                y: 0,
            }.width()
        );

        assert_eq!(
            0.125,
            Tile {
                offset: -123,
                level: 3,
                x: 9,
                y: 4,
            }.width()
        );
    }

    #[test]
    fn top_left_position() {
        assert_eq!(
            [0.0, 0.0],
            Tile {
                offset: 0,
                level: 0,
                x: 0,
                y: 0,
            }.top_left_position()
        );

        assert_eq!(
            [-244.875, 0.5],
            Tile {
                offset: -123,
                level: 3,
                x: 9,
                y: 4,
            }.top_left_position()
        );
    }

    #[test]
    fn bottom_right_position() {
        assert_eq!(
            [1.0, 1.0],
            Tile {
                offset: 0,
                level: 0,
                x: 0,
                y: 0,
            }.bottom_right_position()
        );

        assert_eq!(
            [-244.75, 0.625],
            Tile {
                offset: -123,
                level: 3,
                x: 9,
                y: 4,
            }.bottom_right_position()
        );
    }

    #[test]
    fn enclosing_point() {
        assert_eq!(
            Tile {
                offset: 0,
                level: 0,
                x: 0,
                y: 0,
            },
            Tile::enclosing_point(0, [0.0, 0.0])
        );

        assert_eq!(
            Tile {
                offset: -123,
                level: 3,
                x: 9,
                y: 4,
            },
            Tile::enclosing_point(3, [-244.875, 0.5])
        );
    }
}
