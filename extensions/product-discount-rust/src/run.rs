use shopify_function::prelude::*;
use shopify_function::Result;

use serde::{Deserialize, Serialize};

// [START discount-ui-extension.run-configuration]
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Configuration {
    pub collections: Vec<String>,
    pub percentage: f64,
}

impl Configuration {
    const DEFAULT_COLLECTIONS: Vec<String> = vec![];
    const DEFAULT_PERCENTAGE: f64 = 0.0;
    fn from_str(value: &str) -> Self {
        serde_json::from_str(value).expect("Unable to parse configuration value from metafield")
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            collections: Self::DEFAULT_COLLECTIONS,
            percentage: Self::DEFAULT_PERCENTAGE,
        }
    }
}

#[shopify_function_target(query_path = "src/run.graphql", schema_path = "schema.graphql")]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    let no_discount = output::FunctionRunResult {
        discounts: vec![],
        discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
    };

    let config = match input.discount_node.metafield {
        Some(input::InputDiscountNodeMetafield { value }) => Configuration::from_str(&value),
        None => return Ok(no_discount),
    };

    let targets = input
        .cart
        .lines
        .iter()
        .filter(|line: &&input::InputCartLines| match &line.merchandise {
            input::InputCartLinesMerchandise::ProductVariant(product_variant) => {
                return !product_variant.product.in_any_collection;
            }
            _ => false,
        })
        .map(|line| {
            output::Target::CartLine(output::CartLineTarget {
                id: line.id.to_string(),
                quantity: None,
            })
        })
        .collect::<Vec<output::Target>>();

    if targets.is_empty() {
        eprintln!("No cart lines found in eligible collections.");
        return Ok(no_discount);
    }

    Ok(output::FunctionRunResult {
        discounts: vec![output::Discount {
            message: None,
            targets,
            value: output::Value::Percentage(output::Percentage {
                value: Decimal(config.percentage),
            }),
        }],
        discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
    })
}
// [END discount-ui-extension.run-configuration]
