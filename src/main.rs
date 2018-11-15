extern crate buffer_bench;
extern crate xi_rope;
extern crate xi_rope_master;

use xi_rope::rope::Rope as RopeOld;
use xi_rope_master::Rope;

fn main() {
    buffer_bench::check_compliance::<String>();
    buffer_bench::check_compliance::<RopeOld>();
    buffer_bench::check_compliance::<Rope>();
}
