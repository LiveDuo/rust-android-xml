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
        // get table
        let buf = include_bytes!("../resources.arsc");
        let chunk = crate::res::Chunk::parse(&mut Cursor::new(buf)).unwrap();
        let packages = if let crate::res::Chunk::Table(_, p) = chunk { p } else { vec![] };
        let table = crate::table::Table { packages };

        // compile manifest
        let xml = include_str!("../test-data/AndroidManifest.xml");
        let _chunk = crate::xml::compile_xml(&xml, &table).unwrap();
        let mut cursor = Cursor::<Vec<u8>>::default();
        _chunk.write(&mut cursor).unwrap();
    }

}
