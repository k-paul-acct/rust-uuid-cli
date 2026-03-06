use clap::{Parser, ValueEnum};
use uuid::Uuid;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
enum UuidFormat {
    /// Format: 32 hexadecimal digits separated by hyphens.
    D,

    /// Format: 32 hexadecimal digits.
    N,

    /// Format: 32 hexadecimal digits separated by hyphens, enclosed in braces.
    B,

    /// Format: 32 hexadecimal digits separated by hyphens, enclosed in parentheses.
    P,

    /// Format: Four hexadecimal values enclosed in braces,
    /// where the fourth value is a subset of eight hexadecimal values that is also enclosed in braces.
    X,
}

#[derive(Clone, Copy, Debug)]
struct FormatArg {
    format: UuidFormat,
    is_upper: bool,
}

#[derive(Clone)]
struct FormatArgParser;

impl clap::builder::TypedValueParser for FormatArgParser {
    type Value = FormatArg;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::builder::EnumValueParser::<UuidFormat>::new();
        let format = inner.parse_ref(cmd, arg, value)?;

        let s = value.to_string_lossy();
        let is_upper = s.chars().any(|c| c.is_uppercase());

        Ok(FormatArg { format, is_upper })
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        Some(Box::new(
            UuidFormat::value_variants()
                .iter()
                .filter_map(|v| v.to_possible_value())
                .map(|pv| {
                    let name = format!("{} or {}", pv.get_name(), pv.get_name().to_uppercase());
                    let mut new_pv = clap::builder::PossibleValue::new(name);
                    if let Some(help) = pv.get_help() {
                        new_pv = new_pv.help(help.to_string());
                    }
                    new_pv
                }),
        ))
    }
}

/// Generate UUIDs.
#[derive(Parser)]
struct Cli {
    /// Number of UUIDs to generate.
    #[arg(default_value_t = 1)]
    quantity: usize,

    /// UUID version.
    #[arg(short, default_value = "4", value_parser = ["4", "7"])]
    version: String,

    /// UUID output format. Use uppercase letter (e.g. "D", "N") for uppercase output.
    #[arg(short, default_value = "d", ignore_case = true, value_parser = FormatArgParser)]
    format: FormatArg,
}

fn format_uuid(uuid: Uuid, format: UuidFormat, is_upper: bool) -> String {
    let output = match format {
        UuidFormat::D => uuid.to_string(),
        UuidFormat::N => uuid.as_simple().to_string(),
        UuidFormat::B => uuid.as_braced().to_string(),
        UuidFormat::P => format!("({})", uuid),
        UuidFormat::X => {
            let (b32, b16_1, b16_2, b8s) = uuid.as_fields();
            if is_upper {
                format!(
                    "{{{:#010X},{:#06X},{:#06X},{{{:#04X},{:#04X},{:#04X},{:#04X},{:#04X},{:#04X},{:#04X},{:#04X}}}}}",
                    b32, b16_1, b16_2, b8s[0], b8s[1], b8s[2], b8s[3], b8s[4], b8s[5], b8s[6], b8s[7],
                )
            } else {
                format!(
                    "{{{:#010x},{:#06x},{:#06x},{{{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x},{:#04x}}}}}",
                    b32, b16_1, b16_2, b8s[0], b8s[1], b8s[2], b8s[3], b8s[4], b8s[5], b8s[6], b8s[7],
                )
            }
        }
    };

    if is_upper && format != UuidFormat::X {
        output.to_uppercase()
    } else {
        output
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let FormatArg { format, is_upper } = args.format;

    let uuid_generator = match args.version.as_str() {
        "4" => || Uuid::new_v4(),
        "7" => || Uuid::now_v7(),
        _ => unreachable!("unknown version"),
    };

    for _ in 0..args.quantity {
        println!("{}", format_uuid(uuid_generator(), format, is_upper));
    }

    Ok(())
}
