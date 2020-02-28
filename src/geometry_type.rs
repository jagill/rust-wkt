#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GeometryType {
    Point,
    MultiPoint,
    LineString,
    MultiLineString,
    Polygon,
    MultiPolygon,
}

impl GeometryType {
    pub fn of(s: &str) -> Result<Self, String> {
        let lower_s = s.to_ascii_lowercase();
        match lower_s.as_str() {
            "point" => Ok(GeometryType::Point),
            "multipoint" => Ok(GeometryType::MultiPoint),
            "linestring" => Ok(GeometryType::LineString),
            "multilinestring" => Ok(GeometryType::MultiLineString),
            "polygon" => Ok(GeometryType::Polygon),
            "multipolygon" => Ok(GeometryType::MultiPolygon),
            _ => Err(format!("Unknown geometry type: {}", s)),
        }
    }
}
