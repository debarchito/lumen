@0xcb679bf170852530;

using Rust = import "rust.capnp";
$Rust.parentModule("schema");

struct Point {
    x @0 :Float32;
    y @1 :Float32;
}
