#![doc = include_str!("../mdocs/docs.md")]
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct WrapConfig {
    pub orig_name: String,
    pub fake_name: String,
    pub fake_ver: String,
    pub save_orig: bool,
}

#[derive(Debug)]
pub struct WrapResult {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

struct Rewriter {
    re_version: Regex,
    re_usage: Regex,
    cfg: WrapConfig,
}

impl Rewriter {
    fn new(cfg: WrapConfig) -> Self {
        let re_version = Regex::new(&format!(
            r"(?m)^([ \t]*){}([ \t]+)([^ \t\r\n]+)([^\r\n]*?)(\r?\n)?$",
            regex::escape(&cfg.orig_name)
        ))
        .expect("invalid version regex");

        let re_usage = Regex::new(&format!(
            r"(?im)(^|[^\w])([Uu]sage:?[ \t]+){}(\b)",
            regex::escape(&cfg.orig_name)
        ))
        .expect("invalid usage regex");

        Self {
            re_version,
            re_usage,
            cfg,
        }
    }

    fn rewrite_first(&self, line: &str) -> String {
        if let Some(caps) = self.re_version.captures(line) {
            let indent = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let sep = caps.get(2).map(|m| m.as_str()).unwrap_or(" ");
            let orig_ver = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let rest = caps.get(4).map(|m| m.as_str()).unwrap_or("");
            let eol = caps.get(5).map(|m| m.as_str()).unwrap_or("");

            let mut out = String::with_capacity(line.len() + 32);
            out.push_str(indent);
            out.push_str(&self.cfg.fake_name);
            out.push_str(sep);
            out.push_str(&self.cfg.fake_ver);

            if self.cfg.save_orig {
                out.push_str(" (on ");
                out.push_str(&self.cfg.orig_name);
                out.push(' ');
                out.push_str(orig_ver);
                out.push(')');
            }

            out.push_str(rest);
            out.push_str(eol);
            return out;
        }

        self.rewrite_usage(line)
    }

    fn rewrite_usage(&self, line: &str) -> String {
        self.re_usage
            .replace_all(line, |caps: &regex::Captures| {
                let prefix = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let usage = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                let boundary = caps.get(3).map(|m| m.as_str()).unwrap_or("");
                format!("{}{}{}{}", prefix, usage, self.cfg.fake_name, boundary)
            })
            .into_owned()
    }
}

pub fn run_streaming<S: AsRef<str>>(
    cfg: &WrapConfig,
    args: impl IntoIterator<Item = S>,
) -> io::Result<ExitStatus> {
    let mut child = spawn(cfg, args)?;

    let child_stdout = child.stdout.take().expect("stdout was piped");
    let child_stderr = child.stderr.take().expect("stderr was piped");

    let rewriter_out = Rewriter::new(cfg.clone());
    let rewriter_err = Rewriter::new(cfg.clone());

    let t_out = thread::spawn(move || -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        let rdr = BufReader::new(child_stdout);
        stream_and_rewrite(rdr, &mut stdout, &rewriter_out)
    });

    let t_err = thread::spawn(move || -> io::Result<()> {
        let mut stderr = io::stderr().lock();
        let rdr = BufReader::new(child_stderr);
        stream_and_rewrite(rdr, &mut stderr, &rewriter_err)
    });

    let status = child.wait()?;

    t_out.join().unwrap()?;
    t_err.join().unwrap()?;

    Ok(status)
}

pub fn run_capture<S: AsRef<str>>(
    cfg: &WrapConfig,
    args: impl IntoIterator<Item = S>,
) -> io::Result<WrapResult> {
    let mut child = spawn(cfg, args)?;

    let child_stdout = child.stdout.take().expect("stdout was piped");
    let child_stderr = child.stderr.take().expect("stderr was piped");

    let rewriter_out = Rewriter::new(cfg.clone());
    let rewriter_err = Rewriter::new(cfg.clone());

    let t_out = thread::spawn(move || -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        let rdr = BufReader::new(child_stdout);
        stream_and_rewrite(rdr, &mut buf, &rewriter_out)?;
        Ok(buf)
    });

    let t_err = thread::spawn(move || -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        let rdr = BufReader::new(child_stderr);
        stream_and_rewrite(rdr, &mut buf, &rewriter_err)?;
        Ok(buf)
    });

    let status = child.wait()?;
    let stdout = t_out.join().unwrap()?;
    let stderr = t_err.join().unwrap()?;

    Ok(WrapResult {
        status,
        stdout,
        stderr,
    })
}

fn spawn<S: AsRef<str>>(cfg: &WrapConfig, args: impl IntoIterator<Item = S>) -> io::Result<Child> {
    Command::new(&cfg.orig_name)
        .args(args.into_iter().map(|s| s.as_ref().to_owned()))
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

fn stream_and_rewrite<R: BufRead, W: Write>(
    mut rdr: R,
    mut out: W,
    rewriter: &Rewriter,
) -> io::Result<()> {
    let mut first_line_done = false;
    let mut line_buf = String::new();

    loop {
        line_buf.clear();
        let n = rdr.read_line(&mut line_buf)?;
        if n == 0 {
            break;
        }

        let rewritten = if !first_line_done {
            first_line_done = true;
            rewriter.rewrite_first(&line_buf)
        } else {
            rewriter.rewrite_usage(&line_buf)
        };

        out.write_all(rewritten.as_bytes())?;
        out.flush()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(save_orig: bool) -> WrapConfig {
        WrapConfig {
            orig_name: "rustc".into(),
            fake_name: "dustc".into(),
            fake_ver: "2.0.0".into(),
            save_orig,
        }
    }

    #[test]
    fn rewrites_version_line_with_save_orig() {
        let c = cfg(true);
        let rewriter = Rewriter::new(c);

        let line = "rustc 1.76.0 (07dca489a 2024-02-04)\n";
        let out = rewriter.rewrite_first(line);
        assert_eq!(
            out,
            "dustc 2.0.0 (on rustc 1.76.0) (07dca489a 2024-02-04)\n"
        );
    }

    #[test]
    fn rewrites_version_line_without_save_orig() {
        let c = cfg(false);
        let rewriter = Rewriter::new(c);

        let line = "rustc 1.76.0 (07dca489a 2024-02-04)\n";
        let out = rewriter.rewrite_first(line);
        assert_eq!(out, "dustc 2.0.0 (07dca489a 2024-02-04)\n");
    }

    #[test]
    fn rewrites_usage_line() {
        let c = cfg(false);
        let rewriter = Rewriter::new(c);

        let line = "Usage: rustc [OPTIONS] INPUT\n";
        let out = rewriter.rewrite_first(line);
        assert_eq!(out, "Usage: dustc [OPTIONS] INPUT\n");
    }

    #[test]
    fn rewrites_usage_case_insensitive() {
        let c = cfg(false);
        let rewriter = Rewriter::new(c);

        let line = "usage: rustc [OPTIONS] INPUT\n";
        let out = rewriter.rewrite_first(line);
        assert_eq!(out, "usage: dustc [OPTIONS] INPUT\n");
    }

    #[test]
    fn rewrites_usage_in_other_lines() {
        let c = cfg(false);
        let rewriter = Rewriter::new(c);

        let line = "Some text. Usage: rustc [OPTIONS]\n";
        let out = rewriter.rewrite_usage(line);
        assert_eq!(out, "Some text. Usage: dustc [OPTIONS]\n");
    }

    #[test]
    fn does_not_touch_unrelated_occurrences() {
        let c = cfg(true);
        let rewriter = Rewriter::new(c);

        let line = "error: aborting due to rustc internal error\n";
        let out = rewriter.rewrite_first(line);
        assert_eq!(out, line);

        let out2 = rewriter.rewrite_usage(line);
        assert_eq!(out2, line);
    }

    #[test]
    fn handles_crlf() {
        let c = cfg(false);
        let rewriter = Rewriter::new(c);

        let line = "rustc 1.76.0\r\n";
        let out = rewriter.rewrite_first(line);
        assert_eq!(out, "dustc 2.0.0\r\n");
    }

    #[test]
    fn preserves_indentation() {
        let c = cfg(false);
        let rewriter = Rewriter::new(c);

        let line = "  Usage: rustc [OPTIONS]\n";
        let out = rewriter.rewrite_usage(line);
        assert_eq!(out, "  Usage: dustc [OPTIONS]\n");
    }
}
