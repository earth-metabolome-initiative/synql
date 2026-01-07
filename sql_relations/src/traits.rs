//! Submodule providing trait definitions describing abstractions over
//! PostgreSQL relations.

mod same_as;
pub use same_as::{
    HorizontalSameAsColumnLike, HorizontalSameAsForeignKeyLike, HorizontalSameAsTableLike,
    SameAsIndexLike, SameAsTableLike, TriangularSameAsColumnLike, TriangularSameAsForeignKeyLike,
    TriangularSameAsTableLike, VerticalSameAsColumnLike, VerticalSameAsForeignKeyLike,
    VerticalSameAsTableLike,
};
