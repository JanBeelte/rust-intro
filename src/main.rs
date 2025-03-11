mod traits;
mod errors;
mod thiserror;

use traits::trait_example;
use errors::error_example as manual_error_example;
use thiserror::error_example as thiserror_error_example;

fn main() {
    trait_example();
    manual_error_example();
    thiserror_error_example();
}
