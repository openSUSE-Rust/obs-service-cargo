# OBS Source Service `obs-service-cargo_vendor`

<!--
This is the Git repository for [`devel:languages:rust/obs-service-cargo_vendor`](https://build.opensuse.org/package/show/devel:languages:rust/obs-service-cargo_vendor),
an [Open Build Service (OBS)](https://build.opensuse.org) [Source Service](https://openbuildservice.org/help/manuals/obs-user-guide/cha.obs.source_service.html)
to locally vendor Rust crates and dependencies.
-->

The authoritative source of this project is https://github.com/openSUSE/obs-service-cargo_vendor.

`obs-service-cargo_vendor` uses the `Cargo.toml` file present in a Rust application
to run `cargo vendor`, that populates a folder with the sources of the Rust dependencies.
This folder is compressed in a `vendor.tar[.<tar compression>]` archive,
and copied in the RPM package directory source.
The archive can be commited to [OBS](https://build.opensuse.org) to facilitate builds
of Rust application packages for [openSUSE](https://www.opensuse.org),
[SUSE](https://www.suse.com), and numerous other distributions.

The vendored sources allow building a Rust application using `cargo build`, without requiring
access to the network.

## Usage for packagers

Follow this steps when creating a new RPM package for a Rust application:

1. Add to the `_service` file this snippet:
```
<services>
  <service name="cargo_vendor" mode="disabled">
  </service>
</services>
```

> **Note**:
>
> It is assumed that the Rust application is distributed as a tarball named
> `app-0.1.0.tar[.<tar compression>]`, unpacking to `app-0.1.0/`.
> The `<tar compression>` extension can be specified using the `compression` parameter,
> and defaults to `gz`.
>
> `obs-service-cargo_vendor` will autodetect tarball archives of the form `app-0.1.0.tar[.<tar compression>]`,
> where the RPM packaging uses spec file `app.spec`.
>
> The archive name can alternatively be specified using service parameter `archive`.
>
> In some instances a package may be using SCM, in which case a directory is used rather than an archive, `cargo_vendor` can also handle this if archive name and compression args are supplied, eg:
>
> `<param name="archive">tokei</param>`
>
> `<param name="archive">tokei</param>`
>
> `cargo_vendor` will also auto find and use the first directory containing a `Cargo.toml` if the archive args is not used.

2. Run `osc` command locally:

```
$ osc service disabledrun
```

3. Add the generated tarball to the packages sources:
```
$ osc add vendor.tar.gz
```

4. In `app.spec`:
 - include the tarball as a source, along with the cargo_config and the main source tarball:
```
Source0:        app-%{version}.tar.xz
Source1:        vendor.tar.xz
Source2:        cargo_config
```
 - At the `%prep` step, extract the vendor archive inside the app source code:
```
%prep
%setup -q -a1
```

Create a `$CARGO_HOME` directory, including the `config` file in it with the configuration
for the vendored sources:
```
mkdir .cargo
cp %{SOURCE2} .cargo/config
```

- At the `%build` step, export the previously created `$CARGO_HOME` and use `cargo build`
to build the application:
```
cargo build --release # <cargo build options...>
```

## Options

There are a few options that you can supply to the service. The default behaviour without these options is to autodetect the archive type, and in the case where a directory is found with a `Cargo.toml` in the root, then the default compression of `gzip` is used.

- `<param name="strategy">vendor</param>`

The default here is `vendor` which will use `cargo vendor` to fetch the crate dependencies. There are currently no alternatives to `vendor`.

- `<param name="archive">archivename.tar.gz</param>`

The name of the required archive. The option is used in the case where there may be multiple archives available in the package build. This can also be used to specify a directory - useful in the case of using the `obs_scm` service.

- `<param name="compression">xz</param>`

The compression to use for the `vendor.tar`. If the option is not supplied it will default to `gz` or the same compression as the source archive. Available compressions are those supported by `tar`.

#### Example

```
<services>
  <service name="cargo_vendor" mode="disabled">
    <param name="strategy">vendor</param>
    <param name="archive">some_git_repo</param>
    <param name="compression">xz</param>
  </service>
</services>
```

## Transition note

Until `obs-service-cargo_vendor` is available on [OBS](https://build.opensuse.org),
the `vendor.tar[.<tar compression>]` should be committed along with the Rust application
source tarball.

## License

GNU General Public License v2.0 or later

## Attributions

This project is a derivative work of `obs-service-go_modules` available at
https://github.com/openSUSE/obs-service-go_modules.
