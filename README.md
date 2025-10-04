# OBS Source Service `obs-service-cargo`

[![Build OBS Service Cargo](https://github.com/openSUSE/obs-service-cargo_vendor/actions/workflows/build.yml/badge.svg)](https://github.com/openSUSE/obs-service-cargo_vendor/actions/workflows/build.yml)
[![build result](https://build.opensuse.org/projects/devel:languages:rust/packages/obs-service-cargo/badge.svg?type=percent)](https://build.opensuse.org/package/show/devel:languages:rust/obs-service-cargo)

> [!IMPORTANT]
> The original obs-service-cargo_audit is now deprecated as the vendoring process now
> includes audit.
>
> [!IMPORTANT]
> An informative tutorial for packaging Rust software in openSUSE can be found at <https://en.opensuse.org/openSUSE:Packaging_Rust_Software>.

## Quick Start

A Rust project has a root manifest, usually located at
the top-most level directory of the project.

A good example would be the [zellij](https://zellij.dev) project. Users
will just depend the first Cargo.toml found in that project. Therefore,
they do not need to use the `cargotoml` parameter for the `_service` file.

```xml
<services>
  <service name="download_files" mode="manual" />
  <service name="cargo_vendor" mode="manual">
     <param name="src">zellij-0.37.2.tar.gz</param>
     <param name="compression">zst</param>
     <param name="update">true</param>
  </service>
</services>
```

> [!IMPORTANT]
> Although that's how Rust projects are usually structured, it's still important to
> check because that's not always the case, especially, for those that
> have different subprojects or that are monorepos e.g. s390-tools.

## Methods on how to "vendor"

There are two methods on how to vendor Rust dependencies with this service:

- **vendor** method
- **registry** method

The **vendor** method uses `cargo vendor` under the hood, generating vendored tarballs
with the filename `vendor.tar.zst` since **zstd** is the default compression format.

The files inside the vendored tarball contains the following:

- a lockfile `Cargo.lock`. Sometimes it does not exist if the project directory is super different e.g. flux
- other lockfiles and their respective directories. [See more here](#about-lockfiles)
- a `.cargo/config`
- the crates that were fetched during the vendor process.

When extracted, it will have the following paths

```
.
├── .cargo/
│   └── config
├── Cargo.lock
├──.<Path to other Cargo.locks in their respective subcrates/subprojects>
└── vendor/
```

This means, a `%prep` section may look like this

```
%prep
%autosetup -a1
```

No need to copy a `cargo_config` or a lockfile to somewhere else or add it as part of
the sources in the specfile. *They are all part of the vendored tarball now*.

The **registry** method uses `cargo fetch` under the hood, generating vendored tarballs
with the filename `registry.tar.zst` since **zstd** is the default compression format.

If we extract the contents of `registry.tar.zst`, you will get a tree like this

```
.
├── .cargo
│   └── registry
├──.<Path to other Cargo.locks in their respective subcrates/subprojects>
└── Cargo.lock
```

The prep section will still be similar to **vendor** method.

> [!IMPORTANT]
> In this scenario, **it is required** set `$CARGO_HOME` where `.cargo` is located.
> Hence, the build section, the check section and the install section of your specfile should look similar to this.
>
> ```
> %build
> export CARGO_HOME=%{_builddir}/%{buildsubdir}/.cargo  # even just $PWD/.cargo is enough
> %cargo_build
> 
> %check
> export CARGO_HOME=%{_builddir}/%{buildsubdir}/.cargo  # even just $PWD/.cargo is enough
> %cargo_test
> 
> %install
> export CARGO_HOME=%{_builddir}/%{buildsubdir}/.cargo  # even just $PWD/.cargo is enough
> %cargo_install
> ```


> [!WARNING]
> The example `tree` output are what you should expect from projects that have a common top-level `Cargo.toml`. More configurations below are discussed
> such as subprojects or monorepo scenarios where a `Cargo.toml` is not at the top-most level directory of a project.

### No dependencies

We have updated the behaviour of this tool. If there are no dependencies then we will ship a vendored tarball from either
methods with no `vendor` directory inside. However, we will ship the `Cargo.lock` for both methods. The only
difference would be the **registry** method still having the cached registry located at `$CARGO_HOME/registry` where
`CARGO_HOME` is a directory pointing to the directory joined with `.cargo`.

### `cargotoml` behaviours

`cargotoml` is a flag/parameter that is available for **vendor** and **registry** methods. However, it's good to know that `cargotoml`
behaves differently between **vendor** and **registry** methods.

#### Vendor Method

For this method, `cargotoml` is used to pass over the `--sync` flag within `cargo
vendor` command. If there are no root manifest found but there is `cargotoml`, the first
`cargotoml` is assumed to be the "root manifest".

#### Registry Method

For this method, `cargotoml` acts like an extra set of "root" or top-level
manifests. It's not passed over to `--sync` since under the hood, the registry method
uses `cargo-fetch`. This design decision is intentional for monorepo scenarios where
you have many crates that are standalone, regardless if they are a dependency from
each other or not, like the s390-tools or python-tokenizers.

If there is a case that you will do that, do experiment with the setting of the
`no-root-manifest` flag. A tip would be setting `no-root-manifest` to true while having
a lot of `cargotoml` declared. See [Tips and Tricks](#tips-and-tricks) for more info.

## The `src` parameter

The `src` parameter can be in three types:

- a source directory
- a source tarball
- a Git URL

The first two supports globbing and finds the directory or file based on
how the list of matching patterns are sorted, in most cases, in alphanumeric
naming order.

The Git URL is unique as compared to the other two since it contains additional
flags that is passed internally to `roast_scm`. This is a section of the
service file that documents those flags.

```xml
   <parameter name="changesgenerate">
      <description>Whether to generate or update a changelog file or not. Default: false. To be passed to Roast SCM.</description>
      <allowedvalues>true</allowedvalues>
      <allowedvalues>false</allowedvalues>
   </parameter>
   <parameter name="changesauthor">
      <description>Author to include during the changelog generation. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="changesemail">
      <description>Email of author to include during the changelog generation. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="changesoutfile">
      <description> Whether to specify a path to the changes file. Otherwise, it is the current directory and the
      filename is the same filename prefix of the generated tarball e.g. `source.tar.xz` will have `source.changes`
      file. If file exists, append the newest changes to the top-most part of the text file. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="set-version">
      <description>Whether to hard code the version or not. Set it to hard code one, otherwise, it will use the generated version internally. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="set-name">
      <description>Whether to hard code the name or not. Set it to hard code one, otherwise, it will use the generated name internally. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="revision">
      <description>Revision or tag. It can also be a specific commit hash. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="versionrewriteregex">
      <description>Pass a regex with capture groups. Required by `versionrewritepattern` flag. Each capture group is labelled through increments of 1. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="versionrewritepattern">
      <description>Pass a pattern from the capture groups from `versionrewriteregex` flag. To be passed to Roast SCM.</description>
   </parameter>
```

You can see that [roast_scm.service](https://codeberg.org/Rusty-Geckos/roast/src/branch/main/roast_scm.service) have the flags of the same name. This is intentional
since we are passing those parameters' values to `roast_scm`.

> [!IMPORTANT]
> The `src` parameter is available and behaves the same either in **vendor** or **registry** methods.

## Updating dependencies

Updating crate dependencies require users to set `--update` to true. This is
a global update flag.

### Updating specific crates

If a user wishes to update specific crate dependencies,
this global `--update` flag will be overriden and set to false. The syntax
for updating specific crates are as follows:

- To pass a `--precise` flag to `cargo-update`, just add `@` and then a
valid version string e.g. `foo@1.0.0`.
- To pass a `--recursive` flag to `cargo-update`, just add `@` and then the
word `recursive` e.g. `foo@recursive`.
- To pass a specific manifest path that depends on the crate
dependency, just add `+` and then add the specific manifest path
e.g. `foo@1.0.0+foo/path/Cargo.toml`.

> [!IMPORTANT]
> Note that you cannot combine `--precise` and `--recursive` as stated in `cargo-help update`.

> [!IMPORTANT]
> The `update` parameter is available and behaves the same either in **vendor** or **registry** methods.

## Accepting risks of RUSTSEC advisories

Sometimes, software dependencies have vulnerabilities or security issues, and it's not that surprising. If you just want to YOLO because this version
should now be in openSUSE, you can "accept a risk" of a RUSTSEC ID by adding a new parameter `i-accept-the-risk`:

```xml
<services>
  <service mode="manual" name="download_files" />
  <service name="cargo_vendor" mode="manual">
     <param name="srctar">atuin-*.tar.gz</param>
     <param name="i-accept-the-risk">RUSTSEC-2022-0093</param>
     <param name="i-accept-the-risk">RUSTSEC-2021-0041</param>
  </service>
</services>
```

> [!IMPORTANT]
> If you are not sure what to do, let a security expert assess and audit it for you by just pushing the new update.


> [!IMPORTANT]
> The `i-accept-the-risk` parameter is available and behaves the same either in **vendor** or **registry** methods.

# Tips and Tricks

## Using the `cargotoml` parameter

The `cargotoml` parameter is flexible. However, since it behaves differently between methods, let's discuss them.

### With the vendor method

Use only `cargotoml` in situations where you need to also vendor from a subproject. This is useful for certain projects that have manifest files that are not located
at the top-most level directory of the project.

Since this flag is can be used repeatedly, it's good to know that the **first** `cargotoml` flag is considered to be the location of the main `Cargo.toml`. Other
`Cargo.toml` files are passed for ***syncing***.

When adding a `cargotoml` parameter, make sure *it is after the root folder*. For example, your project has a top-level directory named `s390-tools`, and then you should
omit `s390-tools` and proceed to the files or subdirectories next to it. So a manifest file located at `s390-tools/rust/utils/Cargo.toml` will have
the following `cargotoml` parameter value of `rust/utils/Cargo.toml`.

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
> </services>
> ```
>
> This means that the file located at `rust/utils/Cargo.toml` is our "main" `Cargo.toml`.
> But s390-tools have other subprojects in Rust, how do we also vendor them? Read [How to do multiple vendors](#how-to-do-multiple-vendors).

> [!IMPORTANT]
> If a project uses a workspace, you don't actually need to do this unless the workspace manifest is located inside of a subproject.

Once you are ready, run the following command locally:

```bash
osc service mr
```

Then add the generated tarball of vendored dependencies:

```bash
osc add vendor.tar.zst
```

> [!IMPORTANT]
> Some Rust software such as the infamous https://github.com/elliot40404/bonk do not have any dependencies so they may not generate a vendored tarball.
> The service will inform you if that's the case.

### With the registry method

`cargotoml` in the **registry** method is used to locate other "main" `Cargo.toml` files. This is useful for configurations where a project
has multiple crates that are actually subprojects themselves. This design decision was intentional since under the hood, the **registry** method
calls `cargo-fetch` and creates a tarball from the generated `$CARGO_HOME`. See [cargo book documentation of `CARGO_HOME`](https://doc.rust-lang.org/cargo/guide/cargo-home.html#cargo-home).

> [!NOTE]
> Read [How to do multiple vendors](#how-to-do-multiple-vendors) section
> for more information of how `cargotoml` behaves between.

## Respecting lockfiles

Respecting lockfiles is just a matter of setting `respect-lockfile` from `true` to `false`.

> [!WARNING]
> If a lockfile do needs updating, you're ultimately stuck at
> setting the `respect-lockfile` to `false` (which is the default) unless upstream
> uploads an updated lockfile. The vendoring process will abort in case it happens.
> Sadly, this is the behaviour of lockfiles. If there is an update of the version
> with **semver** compatibility e.g. `x.y.1` -> `x.y.2`, then it's likely
> the lockfile will attempt to be updated.
>
> You can think of `respect-lockfile` flag as a way to check if **there are
> any updates of the dependencies**. If set to true, it will try
> to respect the version of the dependencies in lockfile by passing the `--locked` flag BUT
> will try to update to the next compatible semver. And if there is, it will abort the operation.
>
> If we want to respect a lockfile, we have to avoid
> generating a new lockfile if it exists before we
> pass `--locked` flag to other operations.
>
> If a lockfile does not exist, we call `generate-lockfile`.
> Of course, it won't make sense to pass a `--locked` flag
> since we don't have any lockfile to respect to begin with.

# Versioned Dirs

The `--versioned-dirs` flag is used when you
- want to know the version quickly
- prefer this configuration

By default, it is set to true. So far, it has no impact on how we vendor.

It's also useful for things like applying patches to a specified crate since
we know that the extracted path results to `vendor/crate-name-version`.

> [!IMPORTANT]
> This is only available for the **vendor** method.
>
> `versioned-dirs` is not available for **registry** method. Fortunately,
> directory names of crates are already versioned. But to patch it, you have
> to obtain a copy. You can do this by having this knowledge that the
> `registry.tar.zst` contains a path `.cargo/registry/cache/crate-name-version.crate`.
>
> Although, the extension ends with `.crate`, it is actually equivalent to a `.tar.gz`
> or gz compressed tar. Hence, you can theoretically do the following in your specfile
>
> ```
> tar xf .cargo/registry/cache/crate-name-version.crate
> pushd crate-name-version
> # apply patch here
> patch -p1 < %{PATCH1}
> popd
> # Patch your Cargo.toml. Refer to https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section.
> patch -p1 < %{PATCH2}
> ```

# Filter

You can filter platform-specific crates and features
using `filter` option.  It's still **experimental** and it uses
[cargo-vendor-filterer](https://github.com/coreos/cargo-vendor-filterer)
under the hood.

> [!IMPORTANT]
> This is only available for the **vendor** method.

# How to do multiple vendors

## With the vendor method

It is possible to do multiple vendored tarballs by using the `--tag` parameter. This allows you to rename your vendored
in various contexts e.g. projects that are not part of one whole workspace. Example:

```xml
<services>
  <service name="cargo_vendor" mode="manual">
        <param name="src">s390-tools-2.29.0.tar.gz</param>
        <!-- omit root directory name -->
        <param name="cargotoml">rust/pv/Cargo.toml</param>
        <param name="i-accept-the-risk">RUSTSEC-2023-0044</param>
        <param name="tag">rust-pv</param>
        <param name="update">true</param>
  </service>
  <service name="cargo_vendor" mode="manual">
        <!-- omit root directory name -->
        <param name="src">s390-tools-2.29.0.tar.gz</param>
        <param name="cargotoml">rust/pvsecret/Cargo.toml</param>
        <param name="i-accept-the-risk">RUSTSEC-2023-0044</param>
        <param name="tag">pvsecret</param>
        <param name="update">true</param>
  </service>
  <service name="cargo_vendor" mode="manual">
        <!-- omit root directory name -->
        <param name="src">s390-tools-2.29.0.tar.gz</param>
        <param name="cargotoml">rust/utils/Cargo.toml</param>
        <param name="i-accept-the-risk">RUSTSEC-2023-0044</param>
        <param name="tag">utils</param>
        <param name="update">true</param>
  </service>
</services>
```

This will produce the following tarballs:

- `vendor-rust-pv.tar.zst`
- `vendor-pvsecret.tar.zst`
- `vendor-utils.tar.zst`

Thus, this allows you to have many vendored tarballs by using the `--tag` parameter.

> [!WARNING]
> As long as the manifest file contains dependencies or the manifest file is a workspace
> that either has workspace dependencies or member crates that have dependencies, it will produce a vendored tarball. Otherwise, it won't proceed to produce a tarball.
>

## With the registry method

> [!NOTE]
> This attempt started in this repository <https://github.com/openSUSE-Rust/obs-service-cargo-vendor-home-registry> but now,
> it's been merged here to avoid maintenance burden. As the old repository will retire, it still remains there for those that
> are curious about how we go from there to here.

As previously mentioned, we talked about the usage of tags for multi vendor scenarios. However, to avoid this issue of having
to write a long service file while also managing multiple vendors, we introduced the vendoring of `$CARGO_HOME`,
specifically, `$CARGO_HOME/registry`. The service file will look like this

```xml
<services>
  <service name="download_files" mode="manual" />
  <service name="cargo_vendor" mode="manual">
     <param name="src">s390-tools*.tar.gz</param>
     <param name="update">true</param>
     <param name="method">registry</param>
     <param name="no-root-manifest">true</param>
     <param name="cargotoml">rust/pvsecret/Cargo.toml</param>
     <param name="cargotoml">rust/pv/Cargo.toml</param>
     <param name="cargotoml">rust/utils/Cargo.toml</param>
  </service>
</services>
```

# Different configurations

As you might observe, you can have various configurations on how you want to use
both methods.

The [flux project](https://github.com/influxdata/flux) can have three different configurations below.

**Registry method variant 1**
```xml
<services>
  <service name="download_files" mode="manual" />
  <service name="cargo_vendor" mode="manual">
     <param name="src">flux*.tar.gz</param>
     <param name="update">true</param>
     <param name="method">registry</param>
     <param name="no-root-manifest">true</param>
     <param name="cargotoml">libflux/Cargo.toml</param>
  </service>
</services>
```

> [!NOTE]
> You have to decide when to set `no-root-manifest` to true or false.
> A big mistake here is you know that the project has a crate
> with a clear "root" manifest but not in the top-most level
> directory, so you accidentally set it to false. When we refer
> to `no-root-manifest`, we are actually referring to the fact
> that the "root" manifest is not at the top-most level of the directory.

**Registry method variant 2**
```xml
<services>
  <service name="download_files" mode="manual" />
  <service name="cargo_vendor" mode="manual">
     <param name="src">flux*.tar.gz</param>
     <param name="update">true</param>
     <param name="method">registry</param>
     <param name="custom-root">libflux</param>
  </service>
</services>
```

> [!NOTE]
> The second registry method variant is cleaner, as it avoids setting `no-root-manifest`
> and it's pretty clear that we are actually defining a **custom**
> **root** here.

**Vendor method**
```xml
<services>
  <service name="download_files" mode="manual" />
  <service name="cargo_vendor" mode="manual">
     <param name="src">flux*.tar.gz</param>
     <param name="update">true</param>
     <param name="custom-root">libflux</param>
  </service>
</services>
```
> [!NOTE]
> The **vendor method** is cleaner than the previous. As we said before, there is a clear
> location of where our "root" manifest is.

If we extract the contents of `registry.tar.zst` for both registry variants, you will get a tree like this

```
.
├── .cargo
│   └── registry
│
└── libflux
    └── Cargo.lock
```

As for **vendor method**, we get this instead.

```
.
└── libflux
    ├── .cargo
    ├── Cargo.lock
    └── vendor

```

It's good to know why this happens and why it's decided to be like this. For example, if one decides to use the
vendor method, they can just add this to their specfile on the build section.

```
%build
pushd libflux
%cargo_build
popd
```

The registry method looks like this.

```
%build
export CARGO_HOME=$PWD/.cargo
pushd libflux
%cargo_build
popd
```

One caveat with **vendor** method is it can only do *one* thing, so we ended up littered with many
vendored tarballs. While for **registry**, we can have one registry tarball and that's it.
You can see how advantageous it is when you look back at the s390-tools. The previous example can be converted to something like this
if using the **registry** method:

```xml
<services>
  <service name="cargo_vendor" mode="manual">
        <param name="src">s390-tools-2.29.0.tar.gz</param>
        <!-- omit root directory name -->
        <param name="method">registry</param>
        <param name="cargotoml">rust/utils/Cargo.toml</param>
        <param name="cargotoml">rust/pv/Cargo.toml</param>
        <param name="cargotoml">rust/pvsecret/Cargo.toml</param>
        <param name="i-accept-the-risk">RUSTSEC-2023-0044</param>
        <param name="tag">rust-pv</param>
        <param name="update">true</param>
  </service>
</services>
```

In conclusion, the logic is similar but the results of vendoring the home registry
results to lessened mental strain when trying to simplify the
build process in the specfile. Here is what it looks like.

> [!WARNING]
> This is just a theoretical scenario with s390-tools. This
> kind of specfile has not been tested whatsoever. It only
> serves as demonstration.

```
%prep
%autosetup -a1

%build
export CARGO_HOME=$PWD/.cargo
pushd rust/pv
%{cargo_build}
popd
pushd rust/pvsecret
%{cargo_build}
popd

%install
export CARGO_HOME=$PWD/.cargo
pushd rust/pv
%{cargo_install}
popd
pushd rust/pvsecret
%{cargo_install}
popd

%check
export CARGO_HOME=$PWD/.cargo
pushd rust/pv
%{cargo_test}
popd
pushd rust/pvsecret
%{cargo_test}
popd
```

> [!IMPORTANT]
> Overall, both methods are not perfect. But with the right combination of `--custom-root` and `--no-root-manifest`,
> it gets easier if you're able to find that combination.
> You can see more in the [./cargo/tests/behaviour.rs](./cargo/tests//behaviour.rs) for
> such combinations.

# Parameters

The following are the parameters you can use with this utility:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<service name="cargo_vendor">
   <summary>OBS Source Service to vendor all crates.io and dependencies for a Rust project</summary>
   <description><![CDATA[This service extracts a Rust application source,
  searches for a Rust application containing a Cargo.toml file,
  download all crates.io and dependecies,
  and creates a vendor.tar[.<tar compression>] to be committed allowing fully offline
  builds of Rust applications.]]></description>
   <parameter name="strategy">
      <description>Legacy argument, no longer used. Values: cargo_vendor. Default: cargo_vendor.</description>
   </parameter>
   <parameter name="changesgenerate">
      <description>Whether to generate or update a changelog file or not. Default: false. To be passed to Roast SCM.</description>
      <allowedvalues>true</allowedvalues>
      <allowedvalues>false</allowedvalues>
   </parameter>
   <parameter name="changesauthor">
      <description>Author to include during the changelog generation. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="changesemail">
      <description>Email of author to include during the changelog generation. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="changesoutfile">
      <description> Whether to specify a path to the changes file. Otherwise, it is the current directory and the
      filename is the same filename prefix of the generated tarball e.g. `source.tar.xz` will have `source.changes`
      file. If file exists, append the newest changes to the top-most part of the text file. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="set-version">
      <description>Whether to hard code the version or not. Set it to hard code one, otherwise, it will use the generated version internally. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="set-name">
      <description>Whether to hard code the name or not. Set it to hard code one, otherwise, it will use the generated name internally. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="revision">
      <description>Revision or tag. It can also be a specific commit hash. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="versionrewriteregex">
      <description>Pass a regex with capture groups. Required by `versionrewritepattern` flag. Each capture group is labelled through increments of 1. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="versionrewritepattern">
      <description>Pass a pattern from the capture groups from `versionrewriteregex` flag. To be passed to Roast SCM.</description>
   </parameter>
   <parameter name="method">
      <description>Whether to use vendor or the registry. Default: vendor</description>
      <allowedvalues>registry</allowedvalues>
      <allowedvalues>vendor</allowedvalues>
   </parameter>
   <parameter name="src">
      <description>Where to find sources. Source is either a directory or a source tarball or a URL to a remote git repository. Aliases: srctar, srcdir, target, url</description>
   </parameter>
   <parameter name="outdir">
      <description>Where to output vendor.tar* and cargo_config if method is vendor and registry.tar* if method is registry. If using with `osc service`, this option is automatically appended.</description>
   </parameter>
   <parameter name="custom-root">
      <description>Whether you want to manually set the root of the
      project. Useful with a combination with `--manifest-path` (aliased as `--cargotoml`) or
      `--no-root-manifest`.
      </description>
   </parameter>
   <parameter name="update">
      <description>Update dependencies or not. Default: true</description>
      <allowedvalues>false</allowedvalues>
      <allowedvalues>true</allowedvalues>
   </parameter>
   <parameter name="no-root-manifest">
      <description>Available only if `--method` is set to registry. If a
      project has no root manifest, this flag is useful for those situations
      to set the manifest path manually. Useful in combination with
      `--manifest-path` (aliased as `--cargotoml`) flag. Default: false
      </description>
      <allowedvalues>false</allowedvalues>
      <allowedvalues>true</allowedvalues>
   </parameter>
   <parameter name="tag">
      <description>Tag some files for multi-vendor and multi-cargo_config projects</description>
   </parameter>
   <parameter name="compression">
      <description>What compression algorithm to use. Set to `not` if you just want a normal tarball with no compression. Default: zst</description>
      <allowedvalues>zst</allowedvalues>
      <allowedvalues>gz</allowedvalues>
      <allowedvalues>xz</allowedvalues>
      <allowedvalues>bz2</allowedvalues>
      <allowedvalues>not</allowedvalues>
   </parameter>
   <parameter name="cargotoml">
      <description>Other cargo manifest files to sync with vendor or registry. Behaviour between methods changes. Consult the documentation.</description>
   </parameter>
   <parameter name="i-accept-the-risk">
      <description>A list of rustsec-id's to ignore. By setting this value, you acknowledge that this issue does not affect your package and you should be exempt from resolving it.</description>
   </parameter>
   <parameter name="filter">
      <description>Available only if `--method` is set to vendor. EXPERIMENTAL: Reduce vendor-tarball size by filtering out non-Linux dependencies. Default: false</description>
      <allowedvalues>false</allowedvalues>
      <allowedvalues>true</allowedvalues>
   </parameter>
   <parameter name="respect-lockfile">
      <description>Whether to respect Cargo.lock or lockfiles by passing the `--locked` flag. Default: false</description>
      <allowedvalues>false</allowedvalues>
      <allowedvalues>true</allowedvalues>
   </parameter>
   <parameter name="versioned-dirs">
      <description>Available only if `--method` is set to vendor. Whether to use the `--versioned-dirs` flag of cargo-vendor. Default: true</description>
      <allowedvalues>false</allowedvalues>
      <allowedvalues>true</allowedvalues>
   </parameter>
   <parameter name="update-crate">
      <description>Set of specific crates to update. If not empty, it will set the global update flag to false. You can specify a valid version string by adding a `@` after the crate name e.g. `foo@1.2.3`. You can also do recursive updates of a crate by appending `recursive` to `@` e.g. `foo@recursive`. However, recursive can't be used with precise. You can specify a manifest path to update a package with `+` e.g. `foo@1.0+foo/better/Cargo.toml`. See `cargo help update` for info about how to update specific crates.</description>
   </parameter>
</service>
```

# About lockfiles

OBS Cargo Vendor does a boring way to check for lockfiles:

1. If a manifest is not a workspace manifest, it's likely the lockfile
is in the directory of where the manifest is
2. If a manifest is part or a member of a workspace manifest, then it's
likely that the lockfile is on the path of where the workspace manifest
is.

So we just eagerly take all manifest paths from the parameters, and
just check if there are any lockfiles there. And then we slap their full
paths to be part of the vendored tarball. So a path that looks like
`rust/pv/Cargo.lock` may also be reflected in the vendored tarball. Thus,
if extracted, it will go to the desired path `rust/pv/Cargo.lock` from
the root folder of the project.

> [!IMPORTANT]
> If a source does not ship a lockfile, we attempt to regenerate it by
> running the command
> ```bash
> cargo generate-lockfile
> ```
> This ensures that there will be no errors during a `cargo update` or
> a build when update is set to false but there was no lockfile originally.
> Therefore, we check if there is a lockfile **twice**.

# Can we mix registry and vendor?

Yes! You can. There might be a case where you need to use the regular vendor sources
over registry i.e. respecting a working old lockfile by not setting update.

In the future, it would be nice to set each `cargotoml` to have a separate
`update` flag. For now, I see no benefit because, usually, projects that have
multiple crates always catch up with their dependencies of other member crates.

# List of possible scenarios when vendoring fails

- `cargo` issues. Sometimes deleting `~/.cargo` will solve your issues.
- Wrong permissions. You may not have a permission to access a file or folder.
- Incorrect usage of vendoring methods.
- There are updates of this project. Please call us out on that 🤣

# Limitations

There may be some corner/edge (whatever-you-call-it) cases that
will not work with **OBS Service Cargo**. Please open a bug report at
https://github.com/openSUSE-Rust/obs-service-cargo_vendor/issues. We will
try to investigate those in the best of our abilities. The goal of this
project is to help automate some tasks when packaging Rust software. We
won't assume we can automate where we can a locate a project's root manifest
file `Cargo.toml`.  Thus, at best, please indicate it with `cargotoml`
parameter. In the mean time, this will work, *hopefully*, in most projects
since most projects have a root manifest file.

## Reproducibility

This project does not and will not support reproducible builds as a feature. If
you submit a PR to enable those features, we may accept it but we will not
maintain or guarantee that it will continue to work in the future.

> [!NOTE]
> Although, reproducible or deterministic tarballs are fine to reduce checks,
> the idea of reproducibility for security is something the authors
> of this project don't believe in e.g. the Jia Tan incident can't be
> caught with reproducible builds. Although, the tainted sources can be identified using
> reproducible / deterministic generation of the tarballs, those tainted sources
> are **only known AFTER a successful malicious attempt**.

