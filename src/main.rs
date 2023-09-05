use clap::{App, Arg};
use dotenv;
use leopard::LeopardBuilder;
use std::fs;
// use std::io::Write;
// use tabwriter::TabWriter;

fn convert_speech_to_text_by_leopard(
    model_path: Option<&str>,
    // enable_automatic_punctuation: bool,
    // verbose: bool,
) -> std::io::Result<()> {
    let access_key =
        dotenv::var("ACCESS_KEY").expect("AccessKey is REQUIRED for Leopard operation");

    println!("access_key: {}", access_key);

    let audio_files = fs::read_dir("./audio")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();

    let audio_file = &audio_files[0];
    println!("audio file path: {}", audio_file.display().to_string());

    let mut leopard_builder: LeopardBuilder = LeopardBuilder::new();

    if let Some(model_path) = model_path {
        leopard_builder.model_path(model_path);
    }

    let leopard: leopard::Leopard = leopard_builder
        // .enable_automatic_punctuation(enable_automatic_punctuation)
        .access_key(access_key)
        .init()
        .expect("Failed to create Leopard");

    let leopard_transcript: leopard::LeopardTranscript = leopard.process_file(audio_file).unwrap();
    println!("---------------");
    println!("{}", leopard_transcript.transcript);
    println!("---------------");

    // if verbose {
    //     println!();
    //     let mut tw: TabWriter<Vec<u8>> = TabWriter::new(vec![]);
    //     writeln!(&mut tw, "Word\tStart Sec\tEnd Sec\tConfidence").unwrap();
    //     writeln!(&mut tw, "----\t---------\t-------\t----------").unwrap();
    //     leopard_transcript
    //         .words
    //         .iter()
    //         .for_each(|word: &leopard::LeopardWord| {
    //             writeln!(
    //                 &mut tw,
    //                 "{}\t{:.2}\t{:.2}\t{:.2}",
    //                 word.word, word.start_sec, word.end_sec, word.confidence
    //             )
    //             .unwrap();
    //         });
    //     tw.flush().unwrap();
    //     println!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
    // }

    fs::write(
        "output.txt",
        leopard_transcript.transcript.replace(". ", ".\n"),
    )?;

    Ok(())
}

fn main() {
    let matches: clap::ArgMatches = App::new("Picovoice Leopard Rust File Demo")
        .arg(
            Arg::with_name("model_path")
                .long("model_path")
                .short('m')
                .value_name("PATH")
                .help("Path to the file containing model parameter.")
                .takes_value(true),
        )
        // .arg(
        //     Arg::with_name("disable_automatic_punctuation")
        //         .long("disable_automatic_punctuation")
        //         .short('d')
        //         .help("Set to disable automatic punctuation insertion."),
        // )
        // .arg(
        //     Arg::with_name("verbose")
        //         .long("verbose")
        //         .short('v')
        //         .help("Set to enable printing of word metadata."),
        // )
        .get_matches();

    let model_path: Option<&str> = matches.value_of("model_path");

    // let enable_automatic_punctuation: bool = !matches.contains_id("disable_automatic_punctuation");

    // let verbose: bool = matches.contains_id("verbose");

    convert_speech_to_text_by_leopard(
        model_path, /* enable_automatic_punctuation */ /* verbose */
    )
    .unwrap();
}
