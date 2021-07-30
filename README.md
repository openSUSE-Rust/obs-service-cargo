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

## Usage for packagers (with SCM service)

Follow this steps when creating a new RPM package for a Rust application:

1. Add to the `_service` file this snippet:

```
<services>
  <service name="obs_scm" mode="disabled">
    ...
  </service>
  <service name="cargo_vendor" mode="disabled">
    <param name="srcdir">projectname</param>
  </service>
</services>
```

2. Run `osc` command locally:

```
$ osc service ra
```

3. A set of steps is displayed from this command to guide you in modifying your .spec. An example is:

```
Your spec file should be modified per the following example:

---BEGIN---
%global rustflags '-Clink-arg=-Wl,-z,relro,-z,now'

Source1:    vendor.tar.xz
Source2:    cargo_config

%prep
%setup -qa1
mkdir .cargo
cp %{SOURCE2} .cargo/config

%build
RUSTFLAGS=%{rustflags} cargo build --release

%install
RUSTFLAGS=%{rustflags} cargo install --root=%{buildroot}%{_prefix} --path .
```

4. Add the generated tarball to the packages sources:

```
$ osc add vendor.tar.xz
```

5. Perform a local build to confirm the changes work as expected:

```
$ osc build
```

## Manual (without SCM service)

If you are not using SCM, you can use the cargo vendor service manually.

1. Extract your source archive into your working directory.

```
$ tar -xv archive.tar.xz
```

2. Examine the folder name it extracted, IE archive-v1.0.0

3. Set srcdir to match

```
<services>
  <service name="cargo_vendor" mode="disabled">
    <param name="srcdir">archive-v1.0.0</param>
  </service>
</services>
```

4. Continue from Usage for packagers - Step 2.

Note you will not be able to have a server side cargo vendor service with this configuration, so
you should keep `mode="disabled"`

## Options

- `<param name="srcdir">projectname</param>`

The location to search for the Cargo.toml which we will vendor from. Generally this is your project
name from the SCM checkout, or the extracted archive top dir, but it may differ depending on your
configuration.

- `<param name="compression">xz</param>`

The compression to use for the `vendor.tar`. If the option is not supplied it will default to `xz`.
Available compressions are those supported by `tar`.

- `<param name="update" />`

If present, cargo update will be run before vendoring to ensure that the latest version of compatible
dependencies is used.

- `<param name="strategy">vendor</param>`

The default here is `vendor` which will use `cargo vendor` to fetch the crate dependencies. There
are currently no alternatives to `vendor`.

#### Example

```
<services>
  <service name="obs_scm" mode="disabled">
    ...
  </service>
  <service name="cargo_vendor" mode="disabled">
    <param name="strategy">vendor</param>
    <param name="srcdir">projectname</param>
    <param name="compression">xz</param>
    <param name="update">true</param>
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
