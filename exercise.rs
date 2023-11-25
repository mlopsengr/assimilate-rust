use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fmt::{self, Display, Formatter};
use std:fs::{self, remove_file, File};
use std:io::Read;
use std::path::PathBuf;
use std::process::{self, Command};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_EDITION_ARGS: &[&str] = &["--edition", "2021"];
