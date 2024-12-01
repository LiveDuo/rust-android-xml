// Forked from https://github.com/rust-mobile/xbuild/tree/master/apk/src/compiler

pub mod attributes;
pub mod table;
pub mod xml;
pub mod res;

#[cfg(test)]
mod test {
    use std::io::Cursor;

    #[test]
    fn test() {
        let xml = include_str!("../test-data/AndroidManifest.xml");
        let table = crate::table::Table::load().unwrap();
        let chunk = crate::xml::compile_xml(&xml, &table).unwrap();
        let mut cursor = Cursor::<Vec<u8>>::default();
        chunk.write(&mut cursor).unwrap();
        assert_eq!(cursor.into_inner().is_empty(), false);
    }

}
