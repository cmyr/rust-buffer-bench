extern crate buffer_bench;
extern crate xi_rope;

use xi_rope::rope::Rope;

fn main() {
    buffer_bench::check_compliance::<String>();
    buffer_bench::check_compliance::<Rope>();
}
