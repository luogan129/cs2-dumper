use std::io::{Result, Write};

use super::FileBuilder;

#[derive(Debug, PartialEq)]
pub struct PythonFileBuilder;

impl FileBuilder for PythonFileBuilder {
    fn extension(&mut self) -> &str {
        "py"
    }

    fn write_top_level(&mut self, _output: &mut dyn Write) -> Result<()> {
        Ok(())
    }

    fn write_namespace(&mut self, output: &mut dyn Write, name: &str) -> Result<()> {
        write!(output, "class {}:\n", name)?;

        Ok(())
    }

    fn write_variable(
        &mut self,
        output: &mut dyn Write,
        name: &str,
        value: usize,
        comment: Option<&str>,
    ) -> Result<()> {
        match comment {
            Some(comment) => write!(output, "    {} = {:#X} # {}\n", name, value, comment),
            None => write!(output, "    {} = {:#X}\n", name, value),
        }
    }

    fn write_closure(&mut self, output: &mut dyn Write, eof: bool) -> Result<()> {
        if !eof {
            write!(output, "\n")?;
        }

        Ok(())
    }
}