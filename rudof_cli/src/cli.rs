use crate::input_spec::InputSpec;
use crate::{InputConvertFormat, OutputConvertFormat};
use clap::{Parser, Subcommand, ValueEnum};
use shacl_validation::validate::ShaclValidationMode;
use srdf::{RDFFormat, ReaderMode};
use std::fmt::Display;
use std::{fmt::Formatter, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
// #[command(name = "rudof")]
// #[command(author = "Jose Emilio Labra Gayo <labra@uniovi.es>")]
// #[command(version = "0.1")]
#[command(
    arg_required_else_help = true,
    long_about = r#"
 A tool to process and validate RDF data using shapes, and convert between different RDF data models"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show information about ShEx ShapeMaps
    Shapemap {
        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap file name")]
        shapemap: PathBuf,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result shapemap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        result_shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about ShEx schemas
    Shex {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result schema format",
            default_value_t = ShExFormat::ShExJ
        )]
        result_schema_format: ShExFormat,

        #[arg(short = 't', long = "show elapsed time", default_value_t = false)]
        show_time: bool,

        #[arg(long = "statistics", default_value_t = false)]
        show_statistics: bool,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Validate RDF data using ShEx or SHACL
    Validate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(short = 'M', long = "mode", 
            value_name = "Validation mode",
            default_value_t = ValidationMode::ShEx
        )]
        validation_mode: ValidationMode,

        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: InputSpec,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap file name")]
        shapemap: Option<PathBuf>,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(short = 'n', long = "node")]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            long = "max-steps",
            value_name = "max steps to run",
            default_value_t = 100
        )]
        max_steps: usize,

        /// Execution mode
        #[arg(
            short = 'S',
            long = "shacl-mode",
            value_name = "SHACL validation mode",
            default_value_t = ShaclValidationMode::Default,
            value_enum
        )]
        shacl_validation_mode: ShaclValidationMode,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Validate RDF using ShEx schemas
    ShexValidate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 's',
            long = "schema",
            value_name = "Schema file name, URI or -"
        )]
        schema: InputSpec,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap file name")]
        shapemap: Option<PathBuf>,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(short = 'n', long = "node")]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Validate RDF data using SHACL shapes
    ShaclValidate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 's',
            long = "shapes",
            value_name = "Shapes graph: file, URI or -"
        )]
        shapes: InputSpec,

        #[arg(
            short = 'f',
            long = "shapes-format",
            value_name = "Shapes file format",
            default_value_t = ShaclFormat::Turtle
        )]
        shapes_format: ShaclFormat,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        /// Execution mode
        #[arg(
            short = 'm',
            long = "mode",
            value_name = "Execution mode",
            default_value_t = ShaclValidationMode::Default,
            value_enum
        )]
        mode: ShaclValidationMode,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about RDF data
    Data {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        // #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        // data: PathBuf,
        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Ouput result format",
            default_value_t = DataFormat::Turtle
        )]
        result_format: DataFormat,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about a node in an RDF Graph
    Node {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(short = 'n', long = "node")]
        node: String,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'm',
            long = "show-node-mode",
            value_name = "Show Node Mode",
            default_value_t = ShowNodeMode::Outgoing
        )]
        show_node_mode: ShowNodeMode,

        #[arg(long = "show hyperlinks")]
        show_hyperlinks: bool,

        #[arg(short = 'p', long = "predicates")]
        predicates: Vec<String>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(short = 'c', long = "config", value_name = "Path to config file")]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about SHACL shapes
    Shacl {
        #[arg(
            short = 's',
            long = "shapes",
            value_name = "Shapes graph (file, URI or -)"
        )]
        shapes: InputSpec,

        #[arg(
            short = 'f',
            long = "shapes-format",
            value_name = "Shapes file format",
            default_value_t = ShaclFormat::Turtle
        )]
        shapes_format: ShaclFormat,

        #[arg(
            short = 'r',
            long = "result-shapes-format",
            value_name = "Result shapes format",
            default_value_t = ShaclFormat::Internal
        )]
        result_shapes_format: ShaclFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information and process DCTAP files
    #[command(name = "dctap")]
    DCTap {
        #[arg(short = 's', long = "source-file", value_name = "DCTap source file")]
        file: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "DCTap file format",
            default_value_t = DCTapFormat::CSV
        )]
        format: DCTapFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Ouput results format",
            default_value_t = DCTapResultFormat::Internal
        )]
        result_format: DCTapResultFormat,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Convert between different Data modeling technologies
    #[command(name = "convert")]
    Convert {
        #[arg(short = 'c', long = "config", value_name = "Path to config file")]
        config: Option<PathBuf>,

        #[arg(short = 'm', long = "input-mode", value_name = "Input mode")]
        input_mode: InputConvertMode,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,

        #[arg(short = 's', long = "source-file", value_name = "Source file name")]
        file: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "Input file format",
            default_value_t = InputConvertFormat::ShExC
        )]
        format: InputConvertFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result format",
            default_value_t = OutputConvertFormat::Default
        )]
        result_format: OutputConvertFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(short = 't', long = "target-folder", value_name = "Target folder")]
        target_folder: Option<PathBuf>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)"
        )]
        shape: Option<String>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(short = 'x', long = "export-mode", value_name = "Result mode")]
        output_mode: OutputConvertMode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShowNodeMode {
    Outgoing,
    Incoming,
    Both,
}

impl Display for ShowNodeMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShowNodeMode::Outgoing => write!(dest, "outgoing"),
            ShowNodeMode::Incoming => write!(dest, "incoming"),
            ShowNodeMode::Both => write!(dest, "both"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShExFormat {
    Internal,
    Simple,
    ShExC,
    ShExJ,
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl Display for ShExFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExFormat::Internal => write!(dest, "internal"),
            ShExFormat::Simple => write!(dest, "simple"),
            ShExFormat::ShExC => write!(dest, "shexc"),
            ShExFormat::ShExJ => write!(dest, "shexj"),
            ShExFormat::Turtle => write!(dest, "turtle"),
            ShExFormat::NTriples => write!(dest, "ntriples"),
            ShExFormat::RDFXML => write!(dest, "rdfxml"),
            ShExFormat::TriG => write!(dest, "trig"),
            ShExFormat::N3 => write!(dest, "n3"),
            ShExFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShapeMapFormat {
    Compact,
    Internal,
}

impl Display for ShapeMapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeMapFormat::Compact => write!(dest, "compact"),
            ShapeMapFormat::Internal => write!(dest, "internal"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DataFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl From<DataFormat> for RDFFormat {
    fn from(val: DataFormat) -> Self {
        match val {
            DataFormat::Turtle => RDFFormat::Turtle,
            DataFormat::NTriples => RDFFormat::NTriples,
            DataFormat::RDFXML => RDFFormat::RDFXML,
            DataFormat::TriG => RDFFormat::TriG,
            DataFormat::N3 => RDFFormat::N3,
            DataFormat::NQuads => RDFFormat::NQuads,
        }
    }
}

impl Display for DataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DataFormat::Turtle => write!(dest, "turtle"),
            DataFormat::NTriples => write!(dest, "ntriples"),
            DataFormat::RDFXML => write!(dest, "rdfxml"),
            DataFormat::TriG => write!(dest, "trig"),
            DataFormat::N3 => write!(dest, "n3"),
            DataFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShaclFormat {
    Internal,
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl Display for ShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclFormat::Internal => write!(dest, "internal"),
            ShaclFormat::Turtle => write!(dest, "turtle"),
            ShaclFormat::NTriples => write!(dest, "NTriples"),
            ShaclFormat::RDFXML => write!(dest, "rdfxml"),
            ShaclFormat::TriG => write!(dest, "trig"),
            ShaclFormat::N3 => write!(dest, "n3"),
            ShaclFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapFormat {
    CSV,
}

impl Display for DCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapFormat::CSV => write!(dest, "csv"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapResultFormat {
    Internal,
    JSON,
}

impl Display for DCTapResultFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapResultFormat::Internal => write!(dest, "internal"),
            DCTapResultFormat::JSON => write!(dest, "json"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ValidationMode {
    ShEx,
    SHACL,
}

impl Display for ValidationMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationMode::ShEx => write!(dest, "shex"),
            ValidationMode::SHACL => write!(dest, "shacl"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum InputConvertMode {
    SHACL,
    ShEx,
    DCTAP,
}

impl Display for InputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertMode::SHACL => write!(dest, "shacl"),
            InputConvertMode::ShEx => write!(dest, "shex"),
            InputConvertMode::DCTAP => write!(dest, "dctap"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertMode {
    SPARQL,
    ShEx,
    UML,
    HTML,
}

impl Display for OutputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertMode::SPARQL => write!(dest, "sparql"),
            OutputConvertMode::ShEx => write!(dest, "shex"),
            OutputConvertMode::UML => write!(dest, "uml"),
            OutputConvertMode::HTML => write!(dest, "html"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default, Debug)]
#[clap(rename_all = "lower")]
pub enum RDFReaderMode {
    Lax,

    #[default]
    Strict,
}

impl From<RDFReaderMode> for ReaderMode {
    fn from(value: RDFReaderMode) -> Self {
        match value {
            RDFReaderMode::Strict => ReaderMode::Strict,
            RDFReaderMode::Lax => ReaderMode::Lax,
        }
    }
}

impl Display for RDFReaderMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            RDFReaderMode::Strict => write!(dest, "strict"),
            RDFReaderMode::Lax => write!(dest, "lax"),
        }
    }
}
