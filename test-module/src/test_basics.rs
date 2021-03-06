use emacs::{defun, CallEnv, Env, IntoLisp, Result, Value};
use emacs::func::Manage;

use super::MODULE_PREFIX;

fn using_fset(env: &Env) -> Result<()> {
    make_prefix!(prefix, *MODULE_PREFIX);

    fn sum_and_diff(env: &CallEnv) -> Result<Value<'_>> {
        let x: i64 = env.parse_arg(0)?;
        let y: i64 = env.parse_arg(1)?;
        env.list(&[
            (x + y).into_lisp(env)?,
            (x - y).into_lisp(env)?
        ])
    }

    env.fset(
        prefix!("sum-and-diff"),
        emacs::lambda!(env, sum_and_diff, 2..2)?,
    )?;

    Ok(())
}

#[defun(mod_in_name = false)]
fn to_lowercase_or_nil(env: &Env, input: Option<String>) -> Result<Value<'_>> {
    let output = input.map(|s| s.to_lowercase());
    // This tests IntoLisp for Option<&str>.
    output.as_ref().into_lisp(env)
}

pub fn init(env: &Env) -> Result<()> {
    using_fset(env)?;

    fn sum(env: &CallEnv) -> Result<i64> {
        let x: i64 = env.parse_arg(0)?;
        let y: i64 = env.parse_arg(1)?;
        Ok(x + y)
    }

    emacs::__export_functions! {
        env, *MODULE_PREFIX, {
            "sum" => (sum, 2..2),
        }
    }

    Ok(())
}

// Test that raw identifiers are handled correctly. Note that it must be a reserved keyword,
// otherwise syn parses it into a non-raw identifier.
#[defun]
fn r#match() -> Result<()> {
    Ok(())
}

#[defun(mod_in_name = false)]
fn identity_i8(i: i8) -> Result<i8> {
    Ok(i)
}

#[defun(mod_in_name = false)]
fn identity_u8(i: u8) -> Result<u8> {
    Ok(i)
}

#[defun(mod_in_name = false)]
fn u64_overflow() -> Result<u64> {
    Ok(u64::max_value())
}

#[defun(mod_in_name = false)]
fn ignore_args(_: &Env, _: u8, _: u16) -> Result<()> {
    Ok(())
}

#[defun(mod_in_name = false)]
fn copy_string_contents(v: Value, size: usize) -> Result<String> {
    let mut buffer = vec![0u8; size];
    let s = v.copy_string_contents(&mut buffer)?;
    Ok(String::from_utf8_lossy(s).to_string())
}
