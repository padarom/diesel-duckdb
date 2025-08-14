pub(crate) mod value;

use crate::DuckDB;
use diesel::sql_types::*;

pub struct DuckDBTypeMetadata {
    pub oid: u32
}

impl HasSqlType<SmallInt> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}

impl HasSqlType<Integer> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}

impl HasSqlType<BigInt> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}

impl HasSqlType<Float> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}

impl HasSqlType<Double> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}

impl HasSqlType<Text> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}

impl HasSqlType<Binary> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}

impl HasSqlType<Date> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}


impl HasSqlType<Time> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}


impl HasSqlType<Timestamp> for DuckDB {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        todo!()
    }
}
