use clap::{AppSettings, ArgGroup, ArgEnum, Parser};

const AUTHOR: &str = "
Author : Fabrice Corraire <antidote1911@gmail.com>
Github : https://github.com/Antidote1911/
";

#[derive(Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(about, author=AUTHOR, version)]

#[clap(group(ArgGroup::new("mode").required(true)
.args(&["encrypt", "decrypt", "encryptstdin", "decryptstdin"]),
))]

#[clap(group(ArgGroup::new("passwordflags")
.args(&["password", "passwordfile"]),
))]

#[clap(group(ArgGroup::new("destination")
.args(&["output", "stdout"]),
))]
pub struct Cli {

    /// Specifies the file to encrypt.
    #[clap(long, short, value_name = "FILE_TO_ENCRYPT")]
    encrypt: Option<String>,

    /// Specifies the file to decrypt.
    #[clap(long, short, value_name = "FILE_TO_DECRYPT")]
    decrypt: Option<String>,

    /// Encrypt from stdin instead of a file. If an output filename is not specified with -o, the default will be `./encrypted.crypty`.
    #[clap(short='E', long="encryptstdin" ,requires = "passwordflags")]
    encryptstdin: bool,

    /// Decrypt from stdin instead of a file. If an output filename is not specified with -o, the default will be `./decrypted_stdin`.
    #[clap(short='D', long="decryptstdin", requires = "passwordflags")]
    decryptstdin: bool,

    /// Specifies the output file.
    #[clap(long, short, value_name = "PATH_TO_OUTPUT_FILE")]
    output: Option<String>,

    /// Encrypt or decrypt to stdout instead of to a file.
    #[clap(short='O', long="stdout", requires = "passwordflags")]
    stdout: bool,

    /// Optional, and not recommended due to the password appearing in shell history. Password for the file. This or the --password-file (-f) flag is required if using stdin and/or stdout.
    #[clap(short, long, value_name = "PASSWORD")]
    password: Option<String>,

    /// Optional, choose algorithm aessiv or chacha. Ignored in decryption mode
    #[clap(short, long, arg_enum,value_name = "ALGO", default_value = "aessiv")]
    algo: Algo,

    /// The password to encrypt/decrypt with will be read from a text file at the path provided. File should be valid UTF-8 and contain only the password with no newline. This or the --password (-p) flag is required if using stdin and/or stdout.
    #[clap(short='f', long, value_name = "PASSWORD_FILE")]
    passwordfile: Option<String>,

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Algo {
    Aessiv,
    Chacha,
}

impl Cli {
    pub fn algo(&self) -> Algo {
        self.algo
    }
    pub fn password(&self) -> Option<String> {
        self.password.clone()
    }
    pub fn passwordfile(&self) -> Option<&str> {
        self.passwordfile.as_deref()
    }
    pub fn output(&self) -> Option<&str> {
        self.output.as_deref()
    }
    pub fn encrypt(&self) -> Option<&str> {
        self.encrypt.as_deref()
    }
    pub fn decrypt(&self) -> Option<&str> {
        self.decrypt.as_deref()
    }
    pub fn stdout(&self) -> bool {
        self.stdout
    }
    pub fn encryptstdin(&self) -> bool {
        self.encryptstdin
    }
}