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

2. Run `osc` command locally:

```
$ osc service disabledrun
```

3. Add the generated tarball to the packages sources:
```
$ osc add vendor.tar.gz
```

4. In `app.spec`:
 - include the tarball as a source along with the main source tarball:
```
Source0:        app-%{version}.tar.xz
Source1:        vendor.tar.xz
```
 - At the `%prep` step, extract the vendor archive inside the app source code:
```
%prep
%setup -q -a1
```

Create a `$CARGO_HOME` directory, including a `config` file in it with the configuration
for the vendored sources:
```
mkdir .cargo
cat >.cargo/config <<EOF
[source.crates-io]
registry = 'https://github.com/rust-lang/crates.io-index'
replace-with = 'vendored-sources'
[source.vendored-sources]
directory = './vendor'
EOF
```

- At the `%build` step, export the previously created `$CARGO_HOME` and use `cargo build`
to build the application:
```
cargo build --path `pwd` # <cargo build options...>
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
