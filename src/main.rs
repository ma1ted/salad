mod salad;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

#[derive(Debug, StructOpt)]
#[structopt(name = "Salad", about = "Ceasar cipher encoder and decoder")]
struct Cli {
    /// The input text
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Decode an encrypted cipher
    #[structopt(short, long)]
    decrypt: bool,

    /// Number of times to shift each character along
    #[structopt(short, long)]
    shift: i32,

    /// Preserve whitespace
    #[structopt(short, long="preserve")]
    preserve_whitespace: bool,

    /// Use the extended characterset (32 - 123)
    #[structopt(short, long="extended")]
    extended: bool,

    /// Output file, stdout if not present
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>
}

fn main() {
    let args = Cli::from_args();
    // println!("{:#?}", args);

    let content: String;
    if args.input.exists() {
        content = fs::read_to_string(&args.input).unwrap();
    } else {
        content =  args.input.into_os_string().into_string().unwrap();
    }

    let res: String;
    if args.decrypt {
        if args.extended {
            res = salad::decrypt_extended(&content, args.shift, args.preserve_whitespace); 
        } else {
            res = salad::decrypt(&content, args.shift, args.preserve_whitespace);
        }
    } else {
        if args.extended {
            res = salad::encrypt_extended(&content, args.shift, args.preserve_whitespace);
        } else {
            res = salad::encrypt(&content, args.shift, args.preserve_whitespace);
        }
    }

    if args.output != None {
        fs::write(args.output.unwrap(), res).unwrap();
    } else {
        println!("{}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let original = "abcdefghijklmnopqrstuvwxyz";
        let shift = 10;

        let cyphertext = salad::encrypt(original, shift, false);
        let cleartext = salad::decrypt(&cyphertext, shift, false);

        assert_eq!(original, cleartext);
    }
}
