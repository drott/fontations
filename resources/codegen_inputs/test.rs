// This file is a test input that can be rebuilt when making changes to the
// codegen tool itself.
//
// There is a separate codegen plan at resources/test_plan.toml that can be run
// to only rebuild the test outputs.

#![parse_module(read_fonts::codegen_test)]

table KindsOfOffsets {
    /// The major/minor version of the GDEF table
    #[version]
    #[default(MajorMinor::VERSION_1_1)]
    version: MajorMinor,
    /// A normal offset
    nonnullable_offset: Offset16<Dummy>,
    /// An offset that is nullable, but always present
    #[nullable]
    nullable_offset: Offset16<Dummy>,
    /// count of the array at array_offset
    array_offset_count: u16,
    /// An offset to an array:
    #[read_offset_with($array_offset_count)]
    array_offset: Offset16<[u16]>,
    /// A normal offset that is versioned
    #[available(MajorMinor::VERSION_1_1)]
    versioned_nonnullable_offset: Offset16<Dummy>,
    /// An offset that is nullable and versioned
    #[available(MajorMinor::VERSION_1_1)]
    #[nullable]
    versioned_nullable_offset: Offset16<Dummy>,
}

table KindsOfArraysOfOffsets {
    /// The version
    #[version]
    #[compile(MajorMinor::VERSION_1_1)]
    version: MajorMinor,
    /// The number of items in each array
    count: u16,
    /// A normal array offset
    #[count($count)]
    nonnullable_offsets: [Offset16<Dummy>],
    /// An offset that is nullable, but always present
    #[nullable]
    #[count($count)]
    nullable_offsets: [Offset16<Dummy>],
    /// A normal offset that is versioned
    #[available(MajorMinor::VERSION_1_1)]
    #[count($count)]
    versioned_nonnullable_offsets: [Offset16<Dummy>],
    /// An offset that is nullable and versioned
    #[available(MajorMinor::VERSION_1_1)]
    #[nullable]
    #[count($count)]
    versioned_nullable_offsets: [Offset16<Dummy>],
}

table KindsOfArrays {
    #[version]
    #[default(1)]
    version: u16,
    /// the number of items in each array
    count: u16,
    /// an array of scalars
    #[count($count)]
    scalars: [u16],
    /// an array of records
    #[count($count)]
    records: [Shmecord],
    /// a versioned array of scalars
    #[available(1)]
    #[count($count)]
    versioned_scalars: [u16],
    /// a versioned array of scalars
    #[available(1)]
    #[count($count)]
    versioned_records: [Shmecord],
}

table Dummy {
    value: u16,
}

record Shmecord {
    length: u16,
    breadth: u32,
}
