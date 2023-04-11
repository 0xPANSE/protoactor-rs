// build.rs

use prost_build::Config;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("cargo:rerun-if-changed=src/proto.proto");

    Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("src/proto")
        .compile_protos(&["src/proto.proto"], &[""])?;

    // note: probably not needed since PID will be wrapped into ActorRef
    //       because of type safety of user messages. So it can be generated
    //       as any other message in proto file.
    // Post-process the generated code to remove the PID message
    // let proto_rs_path = "src/proto/actor.rs";
    // if Path::new(proto_rs_path).exists() {
    //     post_process_generated_code(proto_rs_path)?;
    // } else {
    //     eprintln!("Warning: Generated Rust file not found: {}", proto_rs_path);
    // }

    Ok(())
}

/// Post-process the generated code to remove the PID message including its attributes.
fn post_process_generated_code(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);

    let mut output = String::new();
    let mut buffer = String::new();
    let mut inside_block = false;
    let mut pid_found = false;

    for line in reader.lines() {
        let line = line?;

        // If the line starts with "#[", we are inside a block that may contain the PID message.
        if line.starts_with("#[") {
            inside_block = true;
        }

        if inside_block {
            // Buffer the current line while inside a block.
            buffer.push_str(&line);
            buffer.push('\n');

            // If the current line contains "pub struct Pid {", set pid_found to true.
            if line.contains("pub struct Pid {") {
                pid_found = true;
            }

            // If the current line contains "}", we reached the end of a block.
            if line.contains("}") {
                // If pid_found is false, the buffered content does not contain the PID message,
                // so we write the buffer to the output.
                if !pid_found {
                    write!(output, "{}", buffer)?;
                }

                // Clear the buffer, reset inside_block and pid_found for the next block.
                buffer.clear();
                inside_block = false;
                pid_found = false;
            }
        } else {
            // If we are not inside a block, write the line directly to the output.
            writeln!(output, "{}", line)?;
        }
    }

    // Write the output to the file.
    let mut output_file = File::create(file_path)?;
    output_file.write_all(output.as_bytes())?;

    Ok(())
}
