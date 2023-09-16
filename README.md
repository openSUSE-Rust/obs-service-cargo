# OBS Source Service `obs-service-cargo`

This is a Rust written variant for https://github.com/openSUSE/obs-service-cargo_vendor and https://github.com/obs-service-cargo_audit.

> [!IMPORTANT]
> An informative tutorial for packaging Rust software in openSUSE can be found at https://en.opensuse.org/openSUSE:Packaging_Rust_Software.

## How to use OBS Service `cargo vendor`

Typical Rust projects may have a **workspace** manifest at the **root of their project directory**. Others don't and do not really require much intervention.

A good example would be the [zellij](https://zellij.dev) project. Users will just depend the first Cargo.toml found in that project. Therefore, they do not need to use the 
`cargotoml` parameter for the `_service` file.

```xml
<services>
  <service name="download_files" mode="manual" />
  <service name="cargo_vendor" mode="manual">
     <param name="src">zellij-0.37.2.tar.gz</param>
     <param name="compression">zst</param>
     <param name="update">true</param>
  </service>
  <service name="cargo_audit" mode="manual" />
</services>
```
## Using `cargotoml` parameter

Use only `cargotoml` in situations where you need to also vendor a subcrate. This is useful for certain projects with no root manifest like the warning below.

> [!WARNING]
> Certain projects may not have a root manifest file, thus, each directory may be a separate subproject e.g. https://github.com/ibm-s390-linux/s390-tools 
> and may need some thinking.
> 
> ```xml
> <services>
>   <service name="cargo_vendor" mode="manual">
>      <param name="srcdir">s390-tools</param>
>      <param name="compression">zst</param>
>      <param name="cargotoml">rust/utils/Cargo.toml</param>
>      <param name="update">true</param>
>   </service>
>   <service name="cargo_audit" mode="manual" />
> </services>
> ```

Once you are ready, run the following command locally:

```bash
osc service mr
```

Then add the generated tarball of vendored dependencies:

```bash
osc add vendor.tar.zst
```

> [!IMPORTANT]
> If using `cargotoml`, the vendored tarball is named after their parent directories e.g. `rust/pv/Cargo.toml` -> `pv.vendor.tar.zst`,
> and has its own `cargo_config` file as well e.g. `rust/pv/Cargo.toml` -> `pv_cargo_config`.

> [!IMPORTANT]
> Some Rust software such as the infamous https://github.com/elliot40404/bonk do not have any dependencies so they may not generate a vendored tarball.
> The service will give you an output of information about it by checking the manifest file.

# Parameters

```
OBS Source Service to vendor all crates.io and dependencies for Rust project locally

Usage: cargo_vendor [OPTIONS] --src <SRC> --outdir <OUTDIR>

Options:
      --src <SRC>                  Where to find sources. Source is either a directory or a source tarball AND cannot be both.
      --compression <COMPRESSION>  What compression algorithm to use. [default: zst] [possible values: gz, xz, zst]
      --tag <TAG>                  Tag some files for multi-vendor and multi-cargo_config projects
      --cargotoml <CARGOTOML>      Other cargo manifest files to sync with during vendor
      --update <UPDATE>            Update dependencies or not [default: true] [possible values: true, false]
      --outdir <OUTDIR>            Where to output vendor.tar* and cargo_config
      --color <WHEN>               Whether WHEN to color output or not [default: auto] [possible values: auto, always, never]
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version

```

# Limitations

There may be some corner/edge (whatever-you-call-it) cases that will not work with **OBS Service Cargo**. Please open a bug 
report at https://github.com/openSUSE/obs-service-cargo_vendor/issues. We will try to investigate those in the best of our abilities. The goal of this
project is to help automate some tasks when packaging Rust software. We won't assume we can automate where we can a locate a project's root manifest file `Cargo.toml`.
Thus, at best, please indicate it with `cargotoml` parameter. In the mean time, this will work, *hopefully*, in most projects since most projects have
a root manifest file.