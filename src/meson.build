cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_options += ['--target-dir', meson.project_build_root() / 'src']

if get_option('profile') == 'release'
  cargo_options += ['--release']
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_env = environment()
cargo_env.set('CARGO_HOME', meson.project_build_root() / 'cargo-home')

# Check if cross-compiling via CARGO_BUILD_TARGET environment variable
cargo_build_target = run_command('sh', '-c', 'echo $CARGO_BUILD_TARGET', check: false).stdout().strip()
if cargo_build_target != ''
  cargo_options += ['--target=' + cargo_build_target]
  binary_path = meson.project_build_root() / 'src' / cargo_build_target / rust_target / 'bootmate'
else
  binary_path = meson.project_build_root() / 'src' / rust_target / 'bootmate'
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'bootmate',
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    cargo,
    'build',
    cargo_options,
    '&&',
    'cp',
    binary_path,
    '@OUTPUT@',
  ],
  env: cargo_env,
)
