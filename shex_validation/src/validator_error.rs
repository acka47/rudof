use std::fmt::Display;

use prefixmap::PrefixMapError;
use rbe::RbeError;
use shex_ast::compiled::preds::Preds;
use shex_ast::compiled::shape_expr::ShapeExpr;
use shex_ast::{
    compiled::shape_label::ShapeLabel, CompiledSchemaError, Node, Pred, ShapeExprLabel,
    ShapeLabelIdx,
};
use srdf::Object;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ValidatorError {
    #[error("SRDF Error: {error}")]
    SRDFError { error: String },

    #[error("Not found shape label {shape}")]
    NotFoundShapeLabel { shape: ShapeLabel },

    #[error("Error converting object to iri: {object}")]
    ConversionObjectIri { object: Object },

    #[error(transparent)]
    CompiledSchemaError(#[from] Box<CompiledSchemaError>),

    #[error("Failed regular expression")]
    RbeFailed(),

    #[error("Closed shape but found properties {remainder:?} which are not part of shape declared properties: {declared:?}")]
    ClosedShapeWithRemainderPreds { remainder: Preds, declared: Preds },

    #[error(transparent)]
    RbeError(#[from] RbeError<Pred, Node, ShapeLabelIdx>),

    #[error(transparent)]
    PrefixMapError(#[from] PrefixMapError),

    #[error("ShapeLabel not found {shape_label:?}: {err}")]
    ShapeLabelNotFoundError {
        shape_label: ShapeExprLabel,
        err: Box<CompiledSchemaError>,
    },

    #[error("And error: shape expression {shape_expr} failed for node {node}: {errors}")]
    ShapeAndError {
        shape_expr: ShapeExpr,
        node: Node,
        errors: ValidatorErrors,
    },

    #[error("Error reading config file from path {path}: {error}")]
    ValidatorConfigFromPathError { path: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    ValidatorConfigYamlError { path: String, error: String },
}

#[derive(Debug, Clone)]
pub struct ValidatorErrors {
    errs: Vec<ValidatorError>,
}

impl ValidatorErrors {
    pub fn new(errs: Vec<ValidatorError>) -> ValidatorErrors {
        ValidatorErrors { errs }
    }
}

impl Display for ValidatorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in self.errs.iter() {
            writeln!(f, "  {err}")?;
        }
        Ok(())
    }
}
