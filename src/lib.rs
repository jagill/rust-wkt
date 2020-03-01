mod char_class;
mod geometry_type;
mod tokenizer;
pub mod wkt_reader;

mod test {
    use crate::geometry_type::GeometryType;
    use crate::wkt_reader::WktReader;

    #[test]
    fn test_wkt() {
        let reader = WktReader::new("POINT(1.234 1234.)");
        let geometry_type = reader.parse().unwrap();
        assert_eq!(geometry_type, GeometryType::Point);
    }
}
