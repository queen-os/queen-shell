use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::{
    commands::{ CommandRef, Command},
    context::Context,
    shell::Shell,
    error::{ProximateShellError, ShellError},
    parser,
    parser::{
        command::{
            classified::{
                external::{ExternalArgs, ExternalCommand},
                internal::InternalCommand,
                ClassifiedCommand, ClassifiedPipeline, Commands,
            },
            parse_command_tail,
        },
        span::HasSpan,
        token::{SpannedToken, Token},
    },
};


pub async fn cli(shell: Arc<dyn Shell>) -> Result<(), ShellError> {
    let mut context = create_default_context(shell.clone());

    loop {
        shell.print(&format!("{}> ", shell.path()));
        let line = shell.readline().await;
        shell.print(&line);
    }

    Ok(())
}

#[inline]
fn create_default_context(shell: Arc<dyn Shell>) -> Context {
    let mut context = Context::new(shell);

    #[inline]
    fn command(c: impl Command + 'static) -> CommandRef {
        Arc::new(c)
    }

    {
        use crate::commands::*;
        context.add_commands(vec![
            command(Ls),
            command(Cd),
        ])
    }

    context
}
