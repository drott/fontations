// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The ['gvar' header](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#gvar-header)
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct GvarMarker {
    glyph_variation_data_offsets_byte_len: usize,
}

impl GvarMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn axis_count_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn shared_tuple_count_byte_range(&self) -> Range<usize> {
        let start = self.axis_count_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn shared_tuples_offset_byte_range(&self) -> Range<usize> {
        let start = self.shared_tuple_count_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn glyph_count_byte_range(&self) -> Range<usize> {
        let start = self.shared_tuples_offset_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
    fn flags_byte_range(&self) -> Range<usize> {
        let start = self.glyph_count_byte_range().end;
        start..start + GvarFlags::RAW_BYTE_LEN
    }
    fn glyph_variation_data_array_offset_byte_range(&self) -> Range<usize> {
        let start = self.flags_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }
    fn glyph_variation_data_offsets_byte_range(&self) -> Range<usize> {
        let start = self.glyph_variation_data_array_offset_byte_range().end;
        start..start + self.glyph_variation_data_offsets_byte_len
    }
}

impl TopLevelTable for Gvar<'_> {
    /// `gvar`
    const TAG: Tag = Tag::new(b"gvar");
}

impl<'a> FontRead<'a> for Gvar<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<MajorMinor>();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        cursor.advance::<Offset32>();
        let glyph_count: u16 = cursor.read()?;
        let flags: GvarFlags = cursor.read()?;
        cursor.advance::<u32>();
        let glyph_variation_data_offsets_byte_len = (transforms::add(glyph_count, 1_usize))
            .checked_mul(<U16Or32 as ComputeSize>::compute_size(&flags)?)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(glyph_variation_data_offsets_byte_len);
        cursor.finish(GvarMarker {
            glyph_variation_data_offsets_byte_len,
        })
    }
}

/// The ['gvar' header](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#gvar-header)
pub type Gvar<'a> = TableRef<'a, GvarMarker>;

impl<'a> Gvar<'a> {
    /// Major/minor version number of the glyph variations table — set to (1,0).
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of variation axes for this font. This must be the
    /// same number as axisCount in the 'fvar' table.
    pub fn axis_count(&self) -> u16 {
        let range = self.shape.axis_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The number of shared tuple records. Shared tuple records can be
    /// referenced within glyph variation data tables for multiple
    /// glyphs, as opposed to other tuple records stored directly
    /// within a glyph variation data table.
    pub fn shared_tuple_count(&self) -> u16 {
        let range = self.shape.shared_tuple_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset from the start of this table to the shared tuple records.
    pub fn shared_tuples_offset(&self) -> Offset32 {
        let range = self.shape.shared_tuples_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`shared_tuples_offset`][Self::shared_tuples_offset].
    pub fn shared_tuples(&self) -> Result<SharedTuples<'a>, ReadError> {
        let data = self.data;
        let args = (self.shared_tuple_count(), self.axis_count());
        self.shared_tuples_offset().resolve_with_args(data, &args)
    }

    /// The number of glyphs in this font. This must match the number
    /// of glyphs stored elsewhere in the font.
    pub fn glyph_count(&self) -> u16 {
        let range = self.shape.glyph_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Bit-field that gives the format of the offset array that
    /// follows. If bit 0 is clear, the offsets are uint16; if bit 0 is
    /// set, the offsets are uint32.
    pub fn flags(&self) -> GvarFlags {
        let range = self.shape.flags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset from the start of this table to the array of
    /// GlyphVariationData tables.
    pub fn glyph_variation_data_array_offset(&self) -> u32 {
        let range = self.shape.glyph_variation_data_array_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offsets from the start of the GlyphVariationData array to each
    /// GlyphVariationData table.
    pub fn glyph_variation_data_offsets(&self) -> ComputedArray<'a, U16Or32> {
        let range = self.shape.glyph_variation_data_offsets_byte_range();
        self.data.read_with_args(range, &self.flags()).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Gvar<'a> {
    fn type_name(&self) -> &str {
        "Gvar"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("axis_count", self.axis_count())),
            2usize => Some(Field::new("shared_tuple_count", self.shared_tuple_count())),
            3usize => Some(Field::new(
                "shared_tuples_offset",
                FieldType::offset(self.shared_tuples_offset(), self.shared_tuples()),
            )),
            4usize => Some(Field::new("glyph_count", self.glyph_count())),
            5usize => Some(Field::new("flags", self.flags())),
            6usize => Some(Field::new(
                "glyph_variation_data_array_offset",
                self.glyph_variation_data_array_offset(),
            )),
            7usize => Some(Field::new(
                "glyph_variation_data_offsets",
                traversal::FieldType::Unknown,
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for Gvar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck :: AnyBitPattern)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct GvarFlags {
    bits: u16,
}

impl GvarFlags {
    /// If set, offsets to GlyphVariationData are 32 bits
    pub const LONG_OFFSETS: Self = Self { bits: 1 };
}

impl GvarFlags {
    ///  Returns an empty set of flags.
    #[inline]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Returns the set containing all flags.
    #[inline]
    pub const fn all() -> Self {
        Self {
            bits: Self::LONG_OFFSETS.bits,
        }
    }

    /// Returns the raw value of the flags currently stored.
    #[inline]
    pub const fn bits(&self) -> u16 {
        self.bits
    }

    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    #[inline]
    pub const fn from_bits(bits: u16) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self { bits })
        } else {
            None
        }
    }

    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    #[inline]
    pub const fn from_bits_truncate(bits: u16) -> Self {
        Self {
            bits: bits & Self::all().bits,
        }
    }

    /// Returns `true` if no flags are currently stored.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }

    /// Returns `true` if there are flags common to both `self` and `other`.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !(Self {
            bits: self.bits & other.bits,
        })
        .is_empty()
    }

    /// Returns `true` if all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    /// Inserts the specified flags in-place.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    /// Removes the specified flags in-place.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.bits &= !other.bits;
    }

    /// Toggles the specified flags in-place.
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.bits ^= other.bits;
    }

    /// Returns the intersection between the flags in `self` and
    /// `other`.
    ///
    /// Specifically, the returned set contains only the flags which are
    /// present in *both* `self` *and* `other`.
    ///
    /// This is equivalent to using the `&` operator (e.g.
    /// [`ops::BitAnd`]), as in `flags & other`.
    ///
    /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }

    /// Returns the union of between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags which are
    /// present in *either* `self` *or* `other`, including any which are
    /// present in both.
    ///
    /// This is equivalent to using the `|` operator (e.g.
    /// [`ops::BitOr`]), as in `flags | other`.
    ///
    /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Returns the difference between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags present in
    /// `self`, except for the ones present in `other`.
    ///
    /// It is also conceptually equivalent to the "bit-clear" operation:
    /// `flags & !other` (and this syntax is also supported).
    ///
    /// This is equivalent to using the `-` operator (e.g.
    /// [`ops::Sub`]), as in `flags - other`.
    ///
    /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::BitOr for GvarFlags {
    type Output = Self;

    /// Returns the union of the two sets of flags.
    #[inline]
    fn bitor(self, other: GvarFlags) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}

impl std::ops::BitOrAssign for GvarFlags {
    /// Adds the set of flags.
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.bits |= other.bits;
    }
}

impl std::ops::BitXor for GvarFlags {
    type Output = Self;

    /// Returns the left flags, but with all the right flags toggled.
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            bits: self.bits ^ other.bits,
        }
    }
}

impl std::ops::BitXorAssign for GvarFlags {
    /// Toggles the set of flags.
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.bits ^= other.bits;
    }
}

impl std::ops::BitAnd for GvarFlags {
    type Output = Self;

    /// Returns the intersection between the two sets of flags.
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
}

impl std::ops::BitAndAssign for GvarFlags {
    /// Disables all flags disabled in the set.
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.bits &= other.bits;
    }
}

impl std::ops::Sub for GvarFlags {
    type Output = Self;

    /// Returns the set difference of the two sets of flags.
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::SubAssign for GvarFlags {
    /// Disables all flags enabled in the set.
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.bits &= !other.bits;
    }
}

impl std::ops::Not for GvarFlags {
    type Output = Self;

    /// Returns the complement of this set of flags.
    #[inline]
    fn not(self) -> Self {
        Self { bits: !self.bits } & Self::all()
    }
}

impl std::fmt::Debug for GvarFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let members: &[(&str, Self)] = &[("LONG_OFFSETS", Self::LONG_OFFSETS)];
        let mut first = true;
        for (name, value) in members {
            if self.contains(*value) {
                if !first {
                    f.write_str(" | ")?;
                }
                first = false;
                f.write_str(name)?;
            }
        }
        if first {
            f.write_str("(empty)")?;
        }
        Ok(())
    }
}

impl std::fmt::Binary for GvarFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.bits, f)
    }
}

impl std::fmt::Octal for GvarFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.bits, f)
    }
}

impl std::fmt::LowerHex for GvarFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.bits, f)
    }
}

impl std::fmt::UpperHex for GvarFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.bits, f)
    }
}

impl font_types::Scalar for GvarFlags {
    type Raw = <u16 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u16>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> From<GvarFlags> for FieldType<'a> {
    fn from(src: GvarFlags) -> FieldType<'a> {
        src.bits().into()
    }
}

/// Array of tuple records shared across all glyph variation data tables.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct SharedTuplesMarker {
    axis_count: u16,
    tuples_byte_len: usize,
}

impl SharedTuplesMarker {
    fn tuples_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + self.tuples_byte_len
    }
}

impl ReadArgs for SharedTuples<'_> {
    type Args = (u16, u16);
}

impl<'a> FontReadWithArgs<'a> for SharedTuples<'a> {
    fn read_with_args(data: FontData<'a>, args: &(u16, u16)) -> Result<Self, ReadError> {
        let (shared_tuple_count, axis_count) = *args;
        let mut cursor = data.cursor();
        let tuples_byte_len = (shared_tuple_count as usize)
            .checked_mul(<Tuple as ComputeSize>::compute_size(&axis_count)?)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(tuples_byte_len);
        cursor.finish(SharedTuplesMarker {
            axis_count,
            tuples_byte_len,
        })
    }
}

impl<'a> SharedTuples<'a> {
    /// A constructor that requires additional arguments.
    ///
    /// This type requires some external state in order to be
    /// parsed.
    pub fn read(
        data: FontData<'a>,
        shared_tuple_count: u16,
        axis_count: u16,
    ) -> Result<Self, ReadError> {
        let args = (shared_tuple_count, axis_count);
        Self::read_with_args(data, &args)
    }
}

/// Array of tuple records shared across all glyph variation data tables.
pub type SharedTuples<'a> = TableRef<'a, SharedTuplesMarker>;

impl<'a> SharedTuples<'a> {
    pub fn tuples(&self) -> ComputedArray<'a, Tuple<'a>> {
        let range = self.shape.tuples_byte_range();
        self.data.read_with_args(range, &self.axis_count()).unwrap()
    }

    pub(crate) fn axis_count(&self) -> u16 {
        self.shape.axis_count
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for SharedTuples<'a> {
    fn type_name(&self) -> &str {
        "SharedTuples"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new(
                "tuples",
                traversal::FieldType::computed_array("Tuple", self.tuples(), self.offset_data()),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for SharedTuples<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// The [GlyphVariationData](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#the-glyphvariationdata-table-array) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct GlyphVariationDataHeaderMarker {
    tuple_variation_headers_byte_len: usize,
}

impl GlyphVariationDataHeaderMarker {
    fn tuple_variation_count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + TupleVariationCount::RAW_BYTE_LEN
    }
    fn serialized_data_offset_byte_range(&self) -> Range<usize> {
        let start = self.tuple_variation_count_byte_range().end;
        start..start + Offset16::RAW_BYTE_LEN
    }
    fn tuple_variation_headers_byte_range(&self) -> Range<usize> {
        let start = self.serialized_data_offset_byte_range().end;
        start..start + self.tuple_variation_headers_byte_len
    }
}

impl<'a> FontRead<'a> for GlyphVariationDataHeader<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<TupleVariationCount>();
        cursor.advance::<Offset16>();
        let tuple_variation_headers_byte_len = cursor.remaining_bytes();
        cursor.advance_by(tuple_variation_headers_byte_len);
        cursor.finish(GlyphVariationDataHeaderMarker {
            tuple_variation_headers_byte_len,
        })
    }
}

/// The [GlyphVariationData](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#the-glyphvariationdata-table-array) table
pub type GlyphVariationDataHeader<'a> = TableRef<'a, GlyphVariationDataHeaderMarker>;

impl<'a> GlyphVariationDataHeader<'a> {
    /// A packed field. The high 4 bits are flags, and the low 12 bits
    /// are the number of tuple variation tables for this glyph. The
    /// number of tuple variation tables can be any number between 1
    /// and 4095.
    pub fn tuple_variation_count(&self) -> TupleVariationCount {
        let range = self.shape.tuple_variation_count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset from the start of the GlyphVariationData table to the
    /// serialized data
    pub fn serialized_data_offset(&self) -> Offset16 {
        let range = self.shape.serialized_data_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`serialized_data_offset`][Self::serialized_data_offset].
    pub fn serialized_data(&self) -> Result<FontData<'a>, ReadError> {
        let data = self.data;
        self.serialized_data_offset().resolve(data)
    }

    /// Array of tuple variation headers.
    pub fn tuple_variation_headers(&self) -> VarLenArray<'a, TupleVariationHeader> {
        let range = self.shape.tuple_variation_headers_byte_range();
        VarLenArray::read(self.data.split_off(range.start).unwrap()).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for GlyphVariationDataHeader<'a> {
    fn type_name(&self) -> &str {
        "GlyphVariationDataHeader"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new(
                "tuple_variation_count",
                traversal::FieldType::Unknown,
            )),
            1usize => Some(Field::new(
                "serialized_data_offset",
                traversal::FieldType::Unknown,
            )),
            2usize => Some(Field::new(
                "tuple_variation_headers",
                traversal::FieldType::Unknown,
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for GlyphVariationDataHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
