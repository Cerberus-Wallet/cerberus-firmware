/// Iterator providing points for 1/8th of a circle (single octant)
///
/// The iterator supplies coordinates of pixels relative to the
/// circle's center point, along with an alpha value in
/// the range (0..255), indicating the proportion of the pixel
/// that lies inside the circle.

///  for (x, y, alpha) in octant_points(radius) {
///    ...
///  }

pub fn octant_points(radius: i16) -> OctantPoints {
    OctantPoints {
        radius,
        x: radius,
        y: 0,
        t1: radius / 16,
        first: true,
    }
}

pub struct OctantPoints {
    radius: i16,
    x: i16,
    y: i16,
    t1: i16,
    first: bool,
}

fn alpha(t1: i16, r1: i16) -> u8 {
    255 - ((t1 as i32 * 255) / r1 as i32) as u8
}

impl Iterator for OctantPoints {
    type Item = (i16, i16, u8, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.y {
            let cur_x = self.x;
            let cur_y = self.y;
            let cur_t1 = self.t1;
            let first = self.first;

            self.first = false;
            self.y += 1;
            self.t1 = self.t1 + self.y;
            let t2 = self.t1 - self.x;
            if t2 >= 0 {
                self.t1 = t2;
                self.x -= 1;
                self.first = true;
            }

            let last = cur_x != self.x;

            Some((cur_x, cur_y, alpha(cur_t1, self.radius), first, last))
        } else {
            None
        }
    }
}
