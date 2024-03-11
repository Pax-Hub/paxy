pub fn init_log(
    preferred_log_dirpath: Option<PathBuf>,
    preferred_log_level_filter: Option<LevelFilter>,
) -> Result<(Handle, PathBuf), Error> {
    let log_filename = format!("{}.log", *app::APP_NAME);
    let log_dirpath = obtain_log_dirpath(preferred_log_dirpath)?;
    let log_file_appender =
        tracing_appender::rolling::daily(log_dirpath.clone(), log_filename.clone());
    let log_level_filter = preferred_log_level_filter.unwrap_or(LevelFilter::INFO);

    // Obtain writers to various logging destinations and worker guards (for
    // keeping the streams alive)
    let (non_blocking_file_writer, _file_writer_guard) =
        tracing_appender::non_blocking(log_file_appender);
    let (non_blocking_stdout_writer, _stdout_writer_guard) =
        tracing_appender::non_blocking(anstream::stdout());
    let (non_blocking_stderr_writer, _stderr_writer_guard) =
        tracing_appender::non_blocking(anstream::stderr());

    // Declare filtering rules for various logging destinations
    // In Regular mode, for stdout, permit messages of equal or lower verbosity
    // than the given filter level, permit messages of higher verbosity than
    // 'WARN', and omit PLAIN target, JSON target, and TEST target.
    let filter_stdout_regular = move |metadata: &Metadata<'_>| {
        metadata.level() <= &log_level_filter
            && metadata.level() > &Level::WARN
            && metadata.target() != "PLAIN"
            && metadata.target() != "JSON"
            && metadata.target() != "TEST"
    };
    // In Test mode, for stdout, permit messages of equal or lower verbosity
    // than the given filter level, permit messages of higher verbosity than
    // 'WARN', and permit all target messages.
    let filter_stdout_test = move |metadata: &Metadata<'_>| {
        metadata.level() <= &log_level_filter && metadata.level() > &Level::WARN
    };
    // In Plain mode, for stdout, print only 'INFO' messages, and permit only PLAIN
    // target messages.
    let filter_stdout_plain = move |metadata: &Metadata<'_>| {
        metadata.level() == &Level::INFO && metadata.target() == "PLAIN"
    };
    // In Json mode, for stdout, print only 'INFO' messages, and permit only JSON
    // target messages.
    let filter_stdout_json = move |metadata: &Metadata<'_>| {
        metadata.level() == &Level::INFO && metadata.target() == "JSON"
    };
    // In Regular mode, for stderr, permit messages of equal or lower verbosity
    // than 'WARN', and permit all targets except TEST.
    let filter_stderr_regular = move |metadata: &Metadata<'_>| {
        metadata.level() < &Level::INFO && metadata.target() != "TEST"
    };
    // In Test mode, for stderr, permit messages of equal or lower verbosity
    // than 'WARN', and permit all targets.
    let filter_stderr_test = move |metadata: &Metadata<'_>| metadata.level() < &Level::INFO;
    type FilterFunctionType = FilterFn<Box<dyn Fn(&Metadata<'_>) -> bool + Send + Sync>>;
    // Box the closure to allow for type match when switching between two similar
    // closures.
    let stdout_regular_filter: FilterFunctionType = filter_fn(Box::new(filter_stdout_regular));
    // Box the closure to allow for type match when switching between two similar
    // closures.
    let stdout_test_filter: FilterFunctionType = filter_fn(Box::new(filter_stdout_test));
    // Box the closure to allow for type match when switching between two similar
    // closures.
    let stdout_plain_filter: FilterFunctionType = filter_fn(Box::new(filter_stdout_plain));
    // Box the closure to allow for type match when switching between two similar
    // closures.
    let stdout_json_filter: FilterFunctionType = filter_fn(Box::new(filter_stdout_json));
    // Box the closure to allow for type match when switching between two similar
    // closures.
    let stderr_regular_filter: FilterFunctionType = filter_fn(Box::new(filter_stderr_regular));
    // Box the closure to allow for type match when switching between two similar
    // closures.
    let stderr_test_filter: FilterFunctionType = filter_fn(Box::new(filter_stderr_test));
    // Wrap the filter in reload::Layer and obtain handle to allow switching between
    // filters.
    let (stdout_filter, stdout_filter_reload_handle) = reload::Layer::new(stdout_regular_filter);
    let (stderr_filter, stderr_filter_reload_handle) = reload::Layer::new(stderr_regular_filter);

    // Closure to switch to non-standard logging for stdout, in json mode or plain
    // mode
    let switch_stdout = move |logging_mode: LoggingMode| match logging_mode {
        LoggingMode::Test => stdout_filter_reload_handle
            .modify(|filter: &mut FilterFunctionType| *filter = stdout_test_filter)
            .context(SwitchToTestSnafu {}),
        LoggingMode::Plain => stdout_filter_reload_handle
            .modify(|filter: &mut FilterFunctionType| *filter = stdout_plain_filter)
            .context(SwitchToPlainSnafu {}),
        LoggingMode::Json => stdout_filter_reload_handle
            .modify(|filter: &mut FilterFunctionType| *filter = stdout_json_filter)
            .context(SwitchToJsonSnafu {}),
        LoggingMode::Regular => Ok(()),
    };
    // Closure to switch to non-standard logging for stderr, in json mode or plain
    // mode
    let switch_stderr = move |logging_mode: LoggingMode| match logging_mode {
        LoggingMode::Test => stderr_filter_reload_handle
            .modify(|filter: &mut FilterFunctionType| *filter = stderr_test_filter)
            .context(SwitchToTestSnafu {}),
        LoggingMode::Regular | LoggingMode::Plain | LoggingMode::Json => Ok(()),
    };

    // Declare logging formats for various logging destinations
    let log_file_layer = fmt::Layer::new()
        .pretty()
        .with_ansi(true)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .with_writer(non_blocking_file_writer)
        .with_filter(LevelFilter::TRACE);
    let stdout_layer = fmt::Layer::new()
        .with_ansi(true)
        .with_file(false)
        .with_level(false)
        .with_line_number(false)
        .with_target(false)
        .without_time()
        .with_writer(non_blocking_stdout_writer)
        .with_filter(stdout_filter);
    let stderr_layer = fmt::Layer::new()
        .with_ansi(true)
        .with_file(false)
        .with_level(true)
        .with_line_number(false)
        .with_target(false)
        .without_time()
        .with_writer(non_blocking_stderr_writer)
        .with_filter(stderr_filter);

    // Compose various filtered logging destination layers and set them to receive
    // tracing messages
    let subscriber = tracing_subscriber::registry()
        .with(log_file_layer)
        .with(stdout_layer)
        .with(stderr_layer);
    tracing::subscriber::set_global_default(subscriber)
        .context(SetGlobalDefaultSubscriberSnafu {})?;

    Ok((
        Handle {
            _switch_stdout_inner: Some(Box::new(switch_stdout)),
            _switch_stderr_inner: Some(Box::new(switch_stderr)),
            worker_guards: vec![
                _file_writer_guard,
                _stdout_writer_guard,
                _stderr_writer_guard,
            ],
        },
        {
            let mut log_filepath = log_dirpath;
            log_filepath.push(log_filename + "*");
            log_filepath
        },
    ))
}

fn obtain_log_dirpath(preferred_log_dirpath: Option<PathBuf>) -> Result<PathBuf, Error> {
    let obtain_fallback_log_dirpath = || {
        let xdg_app_dirs =
            directories::BaseDirs::new().context(RetreiveLoggingUserAppBaseDirectoriesSnafu {})?;
        fs::create_dir_all(xdg_app_dirs.data_dir()).context(CreateLogDirectorySnafu {
            path: {
                let mut state_dirpath = xdg_app_dirs
                    .data_dir()
                    .to_owned();
                state_dirpath.push(*app::APP_NAME);
                state_dirpath
            },
        })?;
        Ok(xdg_app_dirs
            .data_dir()
            .to_owned())
    };
    Ok(match preferred_log_dirpath {
        Some(preferred_log_dirpath) => {
            if !fs::metadata(&preferred_log_dirpath)
                .map(|m| m.permissions())
                .map(|p| p.readonly())
                .unwrap_or(true)
            {
                preferred_log_dirpath
            } else {
                obtain_fallback_log_dirpath()?
            }
        }
        None => obtain_fallback_log_dirpath()?,
    })
}

type OutputModeSwitchFunction = Box<dyn FnOnce(LoggingMode) -> Result<(), Error>>;

pub struct Handle {
    _switch_stdout_inner: Option<OutputModeSwitchFunction>,
    _switch_stderr_inner: Option<OutputModeSwitchFunction>,
    pub worker_guards: Vec<WorkerGuard>,
}

impl Handle {
    pub fn switch_to_test(&mut self) -> Result<(), Error> {
        self.switch_output_mode(LoggingMode::Test)
    }

    pub fn switch_to_plain(&mut self) -> Result<(), Error> {
        self.switch_output_mode(LoggingMode::Plain)
    }

    pub fn switch_to_json(&mut self) -> Result<(), Error> {
        self.switch_output_mode(LoggingMode::Json)
    }

    fn switch_output_mode(&mut self, logging_mode: LoggingMode) -> Result<(), Error> {
        _ = self
            ._switch_stdout_inner
            .take()
            .map(|function_handle| function_handle(logging_mode))
            .ok_or(Error::SwitchFnNotAssigned {})?;
        _ = self
            ._switch_stderr_inner
            .take()
            .map(|function_handle| function_handle(logging_mode))
            .ok_or(Error::SwitchFnNotAssigned {})?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LoggingMode {
    Regular,
    Test,
    Plain,
    Json,
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(
        display("could not retreive the XDG base directories for the user"),
        visibility(pub)
    )]
    RetreiveLoggingUserAppBaseDirectories {},

    #[non_exhaustive]
    #[snafu(
        display("could not create the log directory at {:?}: {source}", path),
        visibility(pub)
    )]
    CreateLogDirectory {
        path: PathBuf,
        source: std::io::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not set the global default tracing subscriber: {source}"),
        visibility(pub)
    )]
    SetGlobalDefaultSubscriber {
        source: tracing::subscriber::SetGlobalDefaultError,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not switch to the Test output format: {source}"),
        visibility(pub)
    )]
    SwitchToTest {
        source: tracing_subscriber::reload::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not switch to the Plain output format: {source}"),
        visibility(pub)
    )]
    SwitchToPlain {
        source: tracing_subscriber::reload::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not switch to the JSON output format: {source}"),
        visibility(pub)
    )]
    SwitchToJson {
        source: tracing_subscriber::reload::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display(
            "The function/closure to switch the output mode has not been assigned. This is a bug."
        ),
        visibility(pub)
    )]
    SwitchFnNotAssigned {},
}

// region: IMPORTS

use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};
use tracing::{Level, Metadata};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    filter::{filter_fn, FilterFn, LevelFilter},
    fmt,
    layer::SubscriberExt,
    reload,
    Layer,
};

use crate::app;

// endregion: IMPORTS
