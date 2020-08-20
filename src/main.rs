#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate log;
use serde::{Serialize, Deserialize};
extern crate simple_logger;
use once_cell::sync::OnceCell;

mod graph;

use crate::graph::{Graph, Node, NodeId, Result};

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: NodeId,
}

// Inspired by this implementation: https://github.com/awslabs/aws-lambda-rust-runtime/issues/123

static INSTANCE: OnceCell<Graph> = OnceCell::new();

#[derive(Serialize)]
struct ApiNode {
    // TODO: consider mapping this into a string of the full URL
    children: Vec<NodeId>,
    reward: u32,
}

impl From<&Node> for ApiNode {
    fn from(node: &Node) -> Self {
        ApiNode {
            reward: node.reward,
            children: node.children.clone(),
        }
    }
}


// The entry point of our bootstrap executable. This is the code that will run when Lambda starts
// our function:

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    // ERROR: we're getting the following error in AWS Lambda:
    //
    // Os { code: 2, kind: NotFound, message: "No such file or directory" }
    INSTANCE.set(Graph::new().unwrap()).ok();
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<ApiNode, HandlerError> {
    let graph = INSTANCE.get().expect("cannot get graph");

    if let Some(node) = graph.get(e.first_name) {
        Ok(ApiNode::from(node))
    } else {
        error!("No node found in request {}", c.aws_request_id);
        Err(c.new_error("cannot find the node"))
    }
}
