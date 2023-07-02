use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;
#[cfg(unix)]
use std::time::Duration;

/// This trait is used to configure tests.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait Testable: Debug
{
    // Required methods
    /// This command will be executed.
    fn command(&self) -> &OsStr; // init

    // Provided methods
    // Default inputs are provided
    /// Set command arguments.
    fn args(&self) -> Vec<&OsStr> { Vec::new() } // set_args
    /// Clear all environment variables if true.
    fn env_clear(&self) -> bool { false } // clear_env
    /// Set environment variables.
    fn envs(&self) -> Vec<(&OsStr, &OsStr)> { Vec::new() }  // set_envs
    /// Set command current dir.
    fn dir(&self) -> Option<&Path> { None }
    /// Set command standard intput.
    fn stdin(&self) -> Option<&str> { None } // set_stdin

    // Specifications methods
    /// The name of the test.
    fn name(&self) -> &str;
    /// Set to false if it is not a test.
    fn test(&self) -> bool { true } // not_a_test
    /// Terminate all tests on failure.
    fn terminate(&self) -> bool { false } // exit_test_on_failure
    /// Show command output.
    #[cfg(unix)]
    fn show_output(&self) -> Option<bool> { None } // show_output, hide_output
    /// If provided, set a timeout for the test.
    #[cfg(unix)]
    fn timeout(&self) -> Option<Duration> { None } // set_timeout

    // Evaluation methods
    /// Check the test output
    fn output(&mut self, _status: i32, _stdout: &[u8], _stderr: &[u8]) -> bool { true }

    // Custom evaluation methods
    /// Additional test 1.
    fn custom_1(&mut self) -> bool { true }
    /// Additional test 2.
    fn custom_2(&mut self) -> bool { true }
    /// Additional test 3.
    fn custom_3(&mut self) -> bool { true }
    /// Additional test 4.
    fn custom_4(&mut self) -> bool { true }
    /// Additional test 5.
    fn custom_5(&mut self) -> bool { true }
}
