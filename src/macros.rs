macro_rules! impl_u8_enum {
	() => {
		#[inline(always)]
		/// Turn [`Self`] into its [`u8`] representation
		pub const fn as_u8(self) -> u8 {
			unsafe { std::mem::transmute(self) }
		}

		#[inline(always)]
		/// Get the next [`Self`], saturating if we're at [`Self::LAST`]
		pub const fn next_saturating(self) -> Self {
			Self::add_saturating(self, 1)
		}

		#[inline(always)]
		/// Get the next [`Self`], wrapping if we're at [`Self::LAST`]
		pub const fn next_wrapping(self) -> Self {
			Self::add_wrapping(self, 1)
		}

		#[inline(always)]
		/// Get the previous [`Self`], saturating if we're at [`Self::FIRST`]
		pub const fn previous_saturating(self) -> Self {
			Self::sub_saturating(self, 1)
		}

		#[inline(always)]
		/// Get the previous [`Self`], wrapping if we're at [`Self::FIRST`]
		pub const fn previous_wrapping(self) -> Self {
			Self::sub_wrapping(self, 1)
		}

		#[inline(always)]
		/// Add onto [`Self`], saturating if we're at [`Self::LAST`]
		pub const fn add_saturating(self, rhs: u8) -> Self {
			Self::new_saturating(self.as_u8().saturating_add(rhs))
		}

		#[inline(always)]
		/// Subtract from [`Self`], saturating if we're at [`Self::FIRST`]
		pub const fn sub_saturating(self, rhs: u8) -> Self {
			Self::new_saturating(self.as_u8().saturating_sub(rhs))
		}

		#[inline(always)]
		/// Add onto [`Self`], wrapping if we're at [`Self::LAST`]
		pub const fn add_wrapping(self, rhs: u8) -> Self {
			Self::new_wrapping(self.as_u8().wrapping_add(rhs))
		}

		#[inline(always)]
		/// Subtract from [`Self`], wrapping if we're at [`Self::FIRST`]
		pub const fn sub_wrapping(self, rhs: u8) -> Self {
			Self::new_wrapping(self.as_u8().wrapping_sub(rhs))
		}
	};
}
pub(crate) use impl_u8_enum;

macro_rules! impl_impl_from_u8_enum {
	($self:ty, $from:ty) => {
		impl From<$from> for $self {
			#[inline(always)]
			/// Calls [`Self::new_saturating`]
			fn from(value: $from) -> Self {
				Self::new_saturating(value as u8)
			}
		}
		impl From<&$from> for $self {
			#[inline(always)]
			/// Calls [`Self::new_saturating`]
			fn from(value: &$from) -> Self {
				Self::new_saturating(*value as u8)
			}
		}
	};
}
pub(crate) use impl_impl_from_u8_enum;

macro_rules! impl_from_u8_enum {
    ($self:ty) => {
        impl_impl_from_u8_enum!($self, u8);
        impl_impl_from_u8_enum!($self, u16);
        impl_impl_from_u8_enum!($self, u32);
        impl_impl_from_u8_enum!($self, u64);
        impl_impl_from_u8_enum!($self, u128);
        impl_impl_from_u8_enum!($self, usize);
        impl_impl_from_u8_enum!($self, i8);
        impl_impl_from_u8_enum!($self, i16);
        impl_impl_from_u8_enum!($self, i32);
        impl_impl_from_u8_enum!($self, i64);
        impl_impl_from_u8_enum!($self, i128);
        impl_impl_from_u8_enum!($self, isize);
    }
}
pub(crate) use impl_from_u8_enum;

macro_rules! impl_traits {
	(
		$t:ty => $inner:ty |
		$($u:ty),* |
		$($i:ty),*
	) => {
		$(
		impl PartialEq<$u> for $t {
			fn eq(&self, other: &$u) -> bool {
				self.inner() == (*other as $inner)
			}
		}
		impl PartialEq<$i> for $t {
			fn eq(&self, other: &$i) -> bool {
				if other.is_negative() {
					false
				} else {
					self.inner() == (*other as $inner)
				}
			}
		}
		impl Into<$u> for $t {
			#[inline]
			fn into(self) -> $u {
				self.inner() as $u
			}
		}
		impl Into<$i> for $t {
			#[inline]
			fn into(self) -> $i {
				self.inner() as $i
			}
		}
		)*
	};
}
pub(crate) use impl_traits;