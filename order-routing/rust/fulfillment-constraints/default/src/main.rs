use shopify_function::prelude::*;
use shopify_function::Result;

use graphql_client;
use serde::{Deserialize, Serialize};

generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

#[derive(Serialize, Deserialize, Default, PartialEq)]
struct Config {}

#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let operations = vec![];

    // Build operations based on the input query response here.

    return Ok(output::FunctionResult { operations });
}

#[cfg(test)]
mod tests;