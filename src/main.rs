use std::io::Write;
use std::path::PathBuf;
use clap::{App, Arg};
use leopard::LeopardBuilder;
use tabwriter::TabWriter;

fn leopard_demo(
    input_audio_path: PathBuf,
    access_key: &str,
    model_path: Option<&str>,
    enable_automatic_punctuation: bool,
    verbose: bool,
) {
    let mut leopard_builder: LeopardBuilder = LeopardBuilder::new();

    if let Some(model_path) = model_path {
        leopard_builder.model_path(model_path);
    }

    let leopard: leopard::Leopard = leopard_builder
        .enable_automatic_punctuation(enable_automatic_punctuation)
        .access_key(access_key)
        .init()
        .expect("Failed to create Leopard");

    let leopard_transcript: leopard::LeopardTranscript = leopard.process_file(input_audio_path).unwrap();
    println!("{}", leopard_transcript.transcript);
    if verbose {
        println!();
        let mut tw: TabWriter<Vec<u8>> = TabWriter::new(vec![]);
        writeln!(&mut tw, "Word\tStart Sec\tEnd Sec\tConfidence").unwrap();
        writeln!(&mut tw, "----\t---------\t-------\t----------").unwrap();
        leopard_transcript.words.iter().for_each(|word: &leopard::LeopardWord| {
            writeln!(
                &mut tw,
                "{}\t{:.2}\t{:.2}\t{:.2}",
                word.word, word.start_sec, word.end_sec, word.confidence
            )
            .unwrap();
        });
        tw.flush().unwrap();
        println!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
    }
}

fn main() {
    let matches: clap::ArgMatches = App::new("Picovoice Leopard Rust File Demo")
        .arg(
            Arg::with_name("input_audio_path")
                .long("input_audio_path")
                .short('i')
                .value_name("PATH")
                .help("Path to input audio file (mono, WAV, 16-bit, 16kHz).")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("access_key")
                .long("access_key")
                .short('a')
                .value_name("ACCESS_KEY")
                .help("AccessKey obtained from Picovoice Console (https://console.picovoice.ai/)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("model_path")
                .long("model_path")
                .short('m')
                .value_name("PATH")
                .help("Path to the file containing model parameter.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("disable_automatic_punctuation")
                .long("disable_automatic_punctuation")
                .short('d')
                .help("Set to disable automatic punctuation insertion."),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short('v')
                .help("Set to enable printing of word metadata."),
        )
        .get_matches();

    let input_audio_path: PathBuf = PathBuf::from(matches.value_of("input_audio_path").unwrap());

    let access_key: &str = matches
        .value_of("access_key")
        .expect("AccessKey is REQUIRED for Leopard operation");

    let model_path: Option<&str> = matches.value_of("model_path");

    let enable_automatic_punctuation: bool = !matches.contains_id("disable_automatic_punctuation");

    let verbose: bool = matches.contains_id("verbose");

    leopard_demo(
        input_audio_path,
        access_key,
        model_path,
        enable_automatic_punctuation,
        verbose,
    );

}
