/// All the names **not** allowed.
#[cfg(not(tarpaulin_include))]
const BANNED_NAMES: [&str; 47] = [
    "node_modules",
    "favicon.ico",
    // https://github.com/juliangruber/builtins/blob/main/index.js (used by npm)
    "assert",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "dns",
    "domain",
    "events",
    "fs",
    "http",
    "https",
    "module",
    "net",
    "os",
    "path",
    "punycode",
    "querystring",
    "readline",
    "repl",
    "stream",
    "string_decoder",
    "sys",
    "timers",
    "tls",
    "tty",
    "url",
    "util",
    "vm",
    "zlib",
    // also from npm `builtins`, but it's the version-locked modules
    "freelist",
    "v8",
    "process",
    "inspector",
    "async_hooks",
    "http2",
    "perf_hooks",
    "trace_events",
    "worker_threads",
    "node:test",
    // also from npm `builtins`, but it's the experimental modules
    "worker_threads",
    "wasi",
    "diagnostics_channel",
];

#[allow(clippy::ptr_arg)]
pub fn is_banned(name: &String) -> bool {
    BANNED_NAMES.contains(&name.to_lowercase().as_str())
}
