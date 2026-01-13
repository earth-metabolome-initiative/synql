//! Submodule providing syn functionalities for `ForeignKeyLike` objects.

use proc_macro2::TokenStream;
use quote::quote;
use sql_traits::traits::{ColumnLike, ForeignKeyLike};
use syn::Path;

use crate::{
    structs::Workspace,
    traits::{ColumnSynLike, TableSynLike},
};

/// Trait defining syn functionalities for `ForeignKeyLike` objects.
pub trait ForeignKeySynLike: ForeignKeyLike {
    /// Returns the syn representation of the foreign key using
    /// the `fk` or the `fpk` macro, depending on whether the foreign key
    /// is a foreign primary key or not.
    fn to_syn(&self, database: &Self::DB, workspace: &Workspace) -> TokenStream {
        let foreign_table_ident = self.referenced_table(database).table_ident();
        let foreign_table_crate_ident = self.referenced_table(database).crate_ident(workspace);
        let host_column_paths = self
            .host_columns(database)
            .map(|col| col.column_snake_ident())
            .collect::<Vec<syn::Ident>>();
        let foreign_column_paths = self
            .referenced_columns(database)
            .map(|col| {
                let col_ident = col.column_snake_ident();
                if self.is_self_referential(database) {
                    syn::parse_quote!(#foreign_table_ident::#col_ident)
                } else {
                    syn::parse_quote!(::#foreign_table_crate_ident::#foreign_table_ident::#col_ident)
                }
            })
            .collect::<Vec<syn::Path>>();
        let foreign_table_ident: Path = if self.is_self_referential(database) {
            foreign_table_ident.into()
        } else {
            syn::parse_quote!(::#foreign_table_crate_ident::#foreign_table_ident)
        };

        if !self.is_composite(database)
            && let Some(first_host_column) = self.host_columns(database).next()
            && first_host_column.non_composite_foreign_keys(database).count() == 1
        {
            quote! {
                #[table_model(foreign_key(#(#host_column_paths)*, #foreign_table_ident))]
            }
        } else {
            quote! {
                #[table_model(foreign_key((#(#host_column_paths,)*), (#(#foreign_column_paths),*)))]
            }
        }
    }
}

impl<FK: ForeignKeyLike> ForeignKeySynLike for FK {}
