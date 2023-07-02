use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::path::Path;
use std::time::Duration;

use crate::Testable;

/// This structure is a boilerplate that provides easy-to-use methods for creating tests.
#[derive(Clone)]
pub struct IntTest<O>
where
    O: FnMut(i32, &[u8], &[u8]) -> bool,
{
    inner: Inner<O>,
}

#[derive(Clone)]
struct Inner<O>
where
    O: FnMut(i32, &[u8], &[u8]) -> bool,
{
    command: OsString,
    args: Vec<OsString>,
    envs: Vec<(OsString, OsString)>,
    env_clear: bool,
    dir: Option<OsString>,
    stdin: Option<String>,
    output: O,
    attribute: Attribute,
}

#[derive(Clone, Debug)]
/// Command attributes.
pub struct Attribute {
    /// Command name.
    pub name: String,
    /// False if you don't want that count as test.
    pub test: bool,
    /// Terminate tester program on fail test.
    pub terminate: bool,
    /// Show command output.
    pub show_output: Option<bool>,
    /// Set test timeout.
    pub timeout: Option<Duration>,
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            name: "default-test-name".into(),
            test: true,
            terminate: false,
            show_output: None,
            timeout: None,
        }
    }
}

impl Attribute {
    /// Name the test.
    pub fn name<N>(mut self, name: N) -> Self
    where
        N: ToString,
    {
        self.name = name.to_string().into();
        self
    }
    /// False if you don't want that count as test.
    pub fn test(mut self, value: bool) -> Self {
        self.test = value;
        self
    }
    /// Terminate tester program on fail test.
    pub fn terminate(mut self, value: bool) -> Self {
        self.terminate = value;
        self
    }
    /// Show command output.
    pub fn show_output(mut self, value: Option<bool>) -> Self {
        self.show_output = value;
        self
    }
    /// Set test timeout.
    pub fn timeout(mut self, value: Option<Duration>) -> Self {
        self.timeout = value;
        self
    }
}

impl<O> Testable for IntTest<O>
where
    O: FnMut(i32, &[u8], &[u8]) -> bool,
{
    fn command(&self) -> &OsStr {
        &self.inner.command
    }
    fn name(&self) -> &str {
        &self.inner.attribute.name
    }
    fn args(&self) -> Vec<&OsStr> {
        self.inner.args.iter().map(|e| e.as_ref()).collect()
    }
    fn envs(&self) -> Vec<(&OsStr, &OsStr)> {
        self.inner
            .envs
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect()
    }
    fn stdin(&self) -> Option<&str> {
        self.inner.stdin.as_deref()
    }
    fn env_clear(&self) -> bool {
        self.inner.env_clear
    }
    fn dir(&self) -> Option<&Path> {
        self.inner.dir.as_ref().map(|dir| dir.as_ref())
    }
    fn test(&self) -> bool {
        self.inner.attribute.test
    }
    fn terminate(&self) -> bool {
        self.inner.attribute.terminate
    }
    #[cfg(unix)]
    fn show_output(&self) -> Option<bool> {
        self.inner.attribute.show_output
    }
    #[cfg(unix)]
    fn timeout(&self) -> Option<Duration> {
        self.inner.attribute.timeout
    }
    fn output(&mut self, status: i32, stdout: &[u8], stderr: &[u8]) -> bool {
        let closure = &mut self.inner.output;
        closure(status, stdout, stderr)
    }
}

#[allow(dead_code)]
impl<'a> IntTest<fn(i32, &[u8], &[u8]) -> bool> {
    /// This builder is responsible for constructing and initializing instances of the IntTest structure.
    pub fn builder<C>(command: C) -> Self
    where
        C: AsRef<OsStr>,
    {
        Self {
            inner: Inner {
                command: command.as_ref().into(),
                args: Vec::new(),
                envs: Vec::new(),
                env_clear: false,
                dir: None,
                stdin: None,
                output: |_v: i32, _stdout: &[u8], _stderr: &[u8]| true,
                attribute: Attribute::default(),
            },
        }
    }
}

impl<O> Debug for IntTest<O>
where
    O: FnMut(i32, &[u8], &[u8]) -> bool,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Test")
            .field("command", &self.inner.command)
            .field("args", &self.inner.args)
            .field("envs", &self.inner.envs)
            .field("dir", &self.inner.dir)
            .field("stdin", &self.inner.stdin)
            .field("env_clear", &self.inner.env_clear)
            .field("attribute", &self.inner.attribute)
            // Not include output in debug output.
            .finish()
    }
}

#[allow(dead_code)]
impl<O> IntTest<O>
where
    O: FnMut(i32, &[u8], &[u8]) -> bool,
{
    /// Configure the standard input.
    pub fn stdin<I>(mut self, stdin: I) -> Self
    where
        I: ToString,
    {
        self.inner.stdin = Some(stdin.to_string());
        self
    }
    /// Provide the arguments for the command to execute.
    pub fn args<A, Y>(mut self, args: A) -> Self
    where
        A: AsRef<[Y]>,
        Y: AsRef<OsStr>,
    {
        self.inner.args = args.as_ref().iter().map(|e| e.into()).collect();
        self
    }
    /// Provide the environments vars for the command to execute.
    pub fn envs<V, Z>(mut self, envs: V) -> Self
    where
        V: AsRef<[(Z, Z)]>,
        Z: AsRef<OsStr>,
    {
        self.inner.envs = envs
            .as_ref()
            .iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }
    /// Configure the working dir.
    pub fn dir<P>(mut self, dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.inner.dir = Some(dir.as_ref().into());
        self
    }
    /// Clear all environments vars.
    pub fn env_clear(mut self, b: bool) -> Self {
        self.inner.env_clear = b;
        self
    }
    /// Set command attributes.
    pub fn set_attribute(mut self, attribute: Attribute) -> Self {
        self.inner.attribute = attribute;
        self
    }
    /// Get command attributes.
    pub fn clone_attribute(&self) -> Attribute {
        self.inner.attribute.clone()
    }
    /// This closure allows for the verification of the test's ExitStatus.
    pub fn output<NO>(self, output: NO) -> IntTest<NO>
    where
        NO: FnMut(i32, &[u8], &[u8]) -> bool,
    {
        IntTest {
            inner: Inner {
                command: self.inner.command,
                args: self.inner.args,
                envs: self.inner.envs,
                env_clear: self.inner.env_clear,
                dir: self.inner.dir,
                stdin: self.inner.stdin,
                output,
                attribute: self.inner.attribute,
            },
        }
    }
}
