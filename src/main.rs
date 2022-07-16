use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "mentor", version = "0.1.0", author = "Ryan Wenger")]
#[clap(about = "A program for simulating CFG's, automata, and regular expressions")]
pub struct Cli {
    /// Specify the type of the description file, rather than automatically deducing it from the
    /// file extension
    #[clap(long, short = 't')]
    //TODO: validate with something like #[structopt(raw(hidden = "true"), parse(try_from_str), default_value = None)]
    r#type: Option<String>,

/**
Path to description file{n}If
the `--type` option is not specified, the file's type will be automatically deduced
from its extension.{n}The
following extensions are accepted:{n}
    dfa   - Deterministic finite automaton{n}
    nfa   - Nondeterministic finite automation{n}
    pda   - Pushdown automaton{n}
    re    - Regular expression{n}
    cfg   - Context-free grammar{n}
    tm    - Turing machine{n}
*/
    desc_file: PathBuf,

    /// Subcommand
    #[clap(subcommand)]
    subcommand: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Display help for a specific subcommand
    #[clap(arg_required_else_help = true, subcommand_negates_reqs = true)]
    Help {
        subcommand: String,
    },

    /**
    Read the description file then determine if the CFG, automaton or regex accepts each string.
    */
    Accept(Accept), /*{
        /// display output trace
        #[clap(long, short = 't')]
        trace: bool,

        /// strings to test
        test_strings: Vec<String>,
    },*/

    /**
    Convert the input to the specified output type.
    Valid conversions are between re<->nfa<->dfa.
    */
    #[clap(arg_required_else_help = true)]
    Convert {
        /// Type to convert to
        #[clap(long, short = 'o')]
        output_type: String,
    },

    /**
    Attempt to determine whether the CFG, automaton, or regex given in the description file
    recognizes the same language as <target>, where <target> is another description file
    of the same type as the input.
    */
    #[clap(arg_required_else_help = true)]
    Compare {
        /// Description file to compare against
        target: PathBuf,
    },

    /**
    Read the description file as an automaton then produce a visual representation
    of the automaton. If no arguments are given, the output will be shown in a desktop window.
    This command does not work on CFG's.
    */
    Graph(Graph),

    /**
    Generate the first <num-words> members in the language of the grammar/PDA.
    */
    #[clap(arg_required_else_help = true)]
    Generate {
        /// Number of words to generate
        num_words: u64,
    },

    /**
    Generate all words in the language up to <length> characters long. Only valid for CFGs and PDAs.
    */
    #[clap(arg_required_else_help = true)]
    GenerateLength {
        /// Maximum length to generate
        length: u64,
    },
}

// Read the description file then determine if the CFG, automaton or regex accepts each string.
#[derive(Debug, Args)]
#[clap(arg_required_else_help = true)]
struct Accept {
    /// display output trace
    #[clap(long, short = 't')]
    trace: bool,

    /// strings to test
    test_strings: Vec<String>,
}

/*
Read the description file as an automaton then produce a visual representation
of the automaton. If no arguments are given, the output will be shown in a separate window.
This command does not work on CFG's.
*/
#[derive(Debug, Args)]
#[clap(arg_required_else_help = false)]
struct Graph {
    /// Write output to the specified PDF file rather than displaying it on-screen
    output_file: Option<PathBuf>,

    /// Display on-screen output in separate window. This is the default behavior if no
    /// `<output-file>` is specified.
    #[clap(long, short = 'd')]
    display: bool, // display = display || output_file.is_none()
}

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        Cmd::Accept(accept) => println!("Accept: {:?}", accept),
        other => println!("Other: {:?}", other),
    };
}

// #[allow(dead_code)]
// const EXPECTED_HELP: &str = r#"mentor 0.1.0
// Ryan Wenger
// A very simple utility to search for a string across multiple files
//
// USAGE:
//     mentor [OPTIONS]
//     mentor help <SUBCOMMAND>
//
// ARGS:
//     <SUBCOMMAND>    Subcommand to display help for
//
// OPTIONS:
//     -h, --help                  Print this help information
//     -V, --version               Print version information
//     -t, --input-type            Specify the type of the description file rather
//                                 than automatically deducing it
// "#;
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use clap::IntoApp;
//     use std::io::Cursor;
//
//     #[test]
//     fn test_help() {
//         let mut app = Cli::into_app();
//         let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
//         app.write_help(&mut cursor).unwrap();
//         let help = String::from_utf8(cursor.into_inner()).unwrap();
//         println!("{}", help);
//         assert_eq!(help, EXPECTED_HELP);
//     }
// }
