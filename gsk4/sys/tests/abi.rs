// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gsk4_sys::*;
use std::env;
use std::error::Error;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gtk4"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed, self.failed, self.failed_to_compile
        )
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        "1",
        get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
        "failed to obtain correct constant value for 1"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_value, c_value
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        Layout {
            size: 1,
            alignment: 1
        },
        get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
        "failed to obtain correct layout for char type"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_layout, &c_layout
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<dyn Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout { size, alignment })
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<dyn Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") || !output.ends_with("###gir test###") {
        return Err(format!(
            "command {:?} return invalid output, {:?}",
            &abi_cmd, &output
        )
        .into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GskBlendMode",
        Layout {
            size: size_of::<GskBlendMode>(),
            alignment: align_of::<GskBlendMode>(),
        },
    ),
    (
        "GskColorStop",
        Layout {
            size: size_of::<GskColorStop>(),
            alignment: align_of::<GskColorStop>(),
        },
    ),
    (
        "GskCorner",
        Layout {
            size: size_of::<GskCorner>(),
            alignment: align_of::<GskCorner>(),
        },
    ),
    (
        "GskGLShaderClass",
        Layout {
            size: size_of::<GskGLShaderClass>(),
            alignment: align_of::<GskGLShaderClass>(),
        },
    ),
    (
        "GskGLUniformType",
        Layout {
            size: size_of::<GskGLUniformType>(),
            alignment: align_of::<GskGLUniformType>(),
        },
    ),
    (
        "GskParseLocation",
        Layout {
            size: size_of::<GskParseLocation>(),
            alignment: align_of::<GskParseLocation>(),
        },
    ),
    (
        "GskRenderNodeType",
        Layout {
            size: size_of::<GskRenderNodeType>(),
            alignment: align_of::<GskRenderNodeType>(),
        },
    ),
    (
        "GskRoundedRect",
        Layout {
            size: size_of::<GskRoundedRect>(),
            alignment: align_of::<GskRoundedRect>(),
        },
    ),
    (
        "GskScalingFilter",
        Layout {
            size: size_of::<GskScalingFilter>(),
            alignment: align_of::<GskScalingFilter>(),
        },
    ),
    (
        "GskSerializationError",
        Layout {
            size: size_of::<GskSerializationError>(),
            alignment: align_of::<GskSerializationError>(),
        },
    ),
    (
        "GskShadow",
        Layout {
            size: size_of::<GskShadow>(),
            alignment: align_of::<GskShadow>(),
        },
    ),
    (
        "GskTransformCategory",
        Layout {
            size: size_of::<GskTransformCategory>(),
            alignment: align_of::<GskTransformCategory>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GSK_BLEND_MODE_COLOR", "12"),
    ("(gint) GSK_BLEND_MODE_COLOR_BURN", "7"),
    ("(gint) GSK_BLEND_MODE_COLOR_DODGE", "6"),
    ("(gint) GSK_BLEND_MODE_DARKEN", "4"),
    ("(gint) GSK_BLEND_MODE_DEFAULT", "0"),
    ("(gint) GSK_BLEND_MODE_DIFFERENCE", "10"),
    ("(gint) GSK_BLEND_MODE_EXCLUSION", "11"),
    ("(gint) GSK_BLEND_MODE_HARD_LIGHT", "8"),
    ("(gint) GSK_BLEND_MODE_HUE", "13"),
    ("(gint) GSK_BLEND_MODE_LIGHTEN", "5"),
    ("(gint) GSK_BLEND_MODE_LUMINOSITY", "15"),
    ("(gint) GSK_BLEND_MODE_MULTIPLY", "1"),
    ("(gint) GSK_BLEND_MODE_OVERLAY", "3"),
    ("(gint) GSK_BLEND_MODE_SATURATION", "14"),
    ("(gint) GSK_BLEND_MODE_SCREEN", "2"),
    ("(gint) GSK_BLEND_MODE_SOFT_LIGHT", "9"),
    ("(gint) GSK_BLEND_NODE", "19"),
    ("(gint) GSK_BLUR_NODE", "22"),
    ("(gint) GSK_BORDER_NODE", "8"),
    ("(gint) GSK_CAIRO_NODE", "2"),
    ("(gint) GSK_CLIP_NODE", "16"),
    ("(gint) GSK_COLOR_MATRIX_NODE", "14"),
    ("(gint) GSK_COLOR_NODE", "3"),
    ("(gint) GSK_CONTAINER_NODE", "1"),
    ("(gint) GSK_CORNER_BOTTOM_LEFT", "3"),
    ("(gint) GSK_CORNER_BOTTOM_RIGHT", "2"),
    ("(gint) GSK_CORNER_TOP_LEFT", "0"),
    ("(gint) GSK_CORNER_TOP_RIGHT", "1"),
    ("(gint) GSK_CROSS_FADE_NODE", "20"),
    ("(gint) GSK_DEBUG_NODE", "23"),
    ("(gint) GSK_GL_SHADER_NODE", "24"),
    ("(gint) GSK_GL_UNIFORM_TYPE_BOOL", "4"),
    ("(gint) GSK_GL_UNIFORM_TYPE_FLOAT", "1"),
    ("(gint) GSK_GL_UNIFORM_TYPE_INT", "2"),
    ("(gint) GSK_GL_UNIFORM_TYPE_NONE", "0"),
    ("(gint) GSK_GL_UNIFORM_TYPE_UINT", "3"),
    ("(gint) GSK_GL_UNIFORM_TYPE_VEC2", "5"),
    ("(gint) GSK_GL_UNIFORM_TYPE_VEC3", "6"),
    ("(gint) GSK_GL_UNIFORM_TYPE_VEC4", "7"),
    ("(gint) GSK_INSET_SHADOW_NODE", "10"),
    ("(gint) GSK_LINEAR_GRADIENT_NODE", "4"),
    ("(gint) GSK_NOT_A_RENDER_NODE", "0"),
    ("(gint) GSK_OPACITY_NODE", "13"),
    ("(gint) GSK_OUTSET_SHADOW_NODE", "11"),
    ("(gint) GSK_RADIAL_GRADIENT_NODE", "6"),
    ("(gint) GSK_REPEATING_LINEAR_GRADIENT_NODE", "5"),
    ("(gint) GSK_REPEATING_RADIAL_GRADIENT_NODE", "7"),
    ("(gint) GSK_REPEAT_NODE", "15"),
    ("(gint) GSK_ROUNDED_CLIP_NODE", "17"),
    ("(gint) GSK_SCALING_FILTER_LINEAR", "0"),
    ("(gint) GSK_SCALING_FILTER_NEAREST", "1"),
    ("(gint) GSK_SCALING_FILTER_TRILINEAR", "2"),
    ("(gint) GSK_SERIALIZATION_INVALID_DATA", "2"),
    ("(gint) GSK_SERIALIZATION_UNSUPPORTED_FORMAT", "0"),
    ("(gint) GSK_SERIALIZATION_UNSUPPORTED_VERSION", "1"),
    ("(gint) GSK_SHADOW_NODE", "18"),
    ("(gint) GSK_TEXTURE_NODE", "9"),
    ("(gint) GSK_TEXT_NODE", "21"),
    ("(gint) GSK_TRANSFORM_CATEGORY_2D", "3"),
    ("(gint) GSK_TRANSFORM_CATEGORY_2D_AFFINE", "4"),
    ("(gint) GSK_TRANSFORM_CATEGORY_2D_TRANSLATE", "5"),
    ("(gint) GSK_TRANSFORM_CATEGORY_3D", "2"),
    ("(gint) GSK_TRANSFORM_CATEGORY_ANY", "1"),
    ("(gint) GSK_TRANSFORM_CATEGORY_IDENTITY", "6"),
    ("(gint) GSK_TRANSFORM_CATEGORY_UNKNOWN", "0"),
    ("(gint) GSK_TRANSFORM_NODE", "12"),
];
