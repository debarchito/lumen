pub mod schema;
use crate::schema::protolumen_capnp::point;
use capnp::serialize_packed;

pub fn write_point(x: f32, y: f32) -> capnp::Result<()> {
    let mut message = capnp::message::Builder::new_default();
    let mut point = message.init_root::<point::Builder>();
    point.set_x(x);
    point.set_y(y);
    serialize_packed::write_message(&mut std::io::stdout(), &message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_point() {
        assert!(write_point(5.0, 10.0).is_ok());
    }
}
