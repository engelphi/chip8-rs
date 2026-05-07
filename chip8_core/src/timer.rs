#[derive(Default, PartialEq, PartialOrd)]
pub(crate) struct Timer(u8);

impl std::ops::Deref for Timer {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::cmp::PartialEq<u8> for Timer {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl std::cmp::PartialOrd<u8> for Timer {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(other))
    }
}

impl std::ops::SubAssign<u8> for Timer {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs
    }
}
