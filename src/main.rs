mod errors;
mod ownership;
mod thiserror;
mod traits;

use errors::error_example as manual_error_example;
use ownership::ownership_example;
use thiserror::error_example as thiserror_error_example;
use traits::trait_example;

fn main() {
    ownership_example();
    trait_example();
    manual_error_example();
    thiserror_error_example();
}
