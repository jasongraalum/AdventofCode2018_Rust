use core::cmp::Ordering;

#[derive(Eq)]
pub struct XYPair {
  pub x : usize,
  pub y : usize,
}

impl Ord for XYPair {
    fn cmp(&self, other: &XYPair) -> Ordering {
        if self.y == other.y {
          self.x.cmp(&other.x)
        }
        else {
          self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for XYPair {
    fn partial_cmp(&self, other: &XYPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for XYPair {
    fn eq(&self, other: &XYPair) -> bool {
        self.x == other.x && self.y == other.y
    }
}