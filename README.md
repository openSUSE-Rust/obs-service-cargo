# OBS Source Service `obs-service-cargo`

[![Build OBS Service Cargo](https://github.com/openSUSE/obs-service-cargo_vendor/actions/workflows/build.yml/badge.svg)](https://github.com/openSUSE/obs-service-cargo_vendor/actions/workflows/build.yml)
[![build result](https://build.opensuse.org/projects/devel:languages:rust/packages/obs-service-cargo/badge.svg?type=percent)](https://build.opensuse.org/package/show/devel:languages:rust/obs-service-cargo)


This is a Rust written variant for https://github.com/openSUSE/obs-service-cargo_vendor and https://github.com/obs-service-cargo_audit.

> [!IMPORTANT]
> The original obs-service-cargo_audit is now deprecated as the vendoring process now
> includes audit.

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
</services>
```

## Versioned Dirs

The `--versioned-dirs` flag is used when you
- want to know the version quickly
- prefer this configuration

By default, it is set to true. So far, it has no impact on how we vendor.

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

## Using `cargotoml` parameter

Use only `cargotoml` in situations where you need to also vendor a subcrate. This is useful for certain projects with no root manifest like the warning below.

When adding a `cargotoml` parameter, make sure *it is after the root folder*. For example, your project has a root folder named `s390-tools`, and then you should
omit `s390-tools` and proceed to the files or subfolders next to it. So a manifest file located at `s390-tools/rust/utils/Cargo.toml` will have
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

> [!IMPORTANT]
> If a project uses a workspace, you don't actually need to do this unless the workspace manifest is part of a subproject.

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
> The service will give you an output of information about it by checking the manifest file.

# What is inside `vendor.tar.<zst,gz,xz>`?

The files inside the vendored tarball contains the following:
- a lockfile `Cargo.lock`. Sometimes it does not exist if the project directory is super different e.g. flux
- other lockfiles and their respective directories. See more [here](#about-lockfiles)
- a `.cargo/config`
- the crates that were fetched during the vendor process.

When extracted, it will have the following paths when extracted.

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

No need to copy a `cargo_config` or a lockfile to somewhere else or add it as part of the sources in the specfile. *They are all part of the vendored tarball now*.

> [!NOTE]
> If desired, you may use this knowledge for weird projects that have weird build configurations. 

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

## Respecting lockfiles

Respecting lockfiles is just a matter of setting `respect-lockfile` from `true` to `false`.

> [!WARNING]
> If a lockfile do needs updating, you're ultimately stuck at
> setting the `respect-lockfile` to `false` (which is the default) unless upstream uploads an updated lockfile.
> The vendoring process will abort in case it happens.
> Sadly, this is the behaviour of lockfiles. If there is an update of the version
> with **semver** compatibility e.g. `x.y.1` -> `x.y.2`, then it's likely
> the lockfile will attempt to be updated.
>
> You can think of `respect-lockfile` flag as a way to check if **there are
> any updates of the dependencies**. If set to false, it will try
> to respect the version of the dependencies in lockfile by passing the `--locked` flag BUT
> will try to update to the next compatible semver. And if there is, it will abort the operation.

# Filter

You can filter platform-specific crates and features
using `filter` option.  It's still **experimental** and it uses
[cargo-vendor-filterer](https://github.com/coreos/cargo-vendor-filterer)
under the hood.

# How to do multiple vendors

> [!NOTE]
> This is an old documentation but still applies for those
> that still use the regular cargo vendor, aka, the **vendor method**.
> A section using `cargo-fetch` and `$CARGO_HOME` is discussed further
> down below, aka, the **registry method**.

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
> that either has workspace dependencies or member crates that have
dependencies, it will produce a vendored tarball. Otherwise, it won't proceed
to produce a tarball.

# Cargo Vendor Home Registry

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
	 <param name="no_root_manifest">true</param>
	 <param name="cargotoml">rust/pvsecret/Cargo.toml</param>
	 <param name="cargotoml">rust/pv/Cargo.toml</param>
	 <param name="cargotoml">rust/utils/Cargo.toml</param>
  </service>
</services>
```

Another example would be libflux. It can have three different configurations and we include **vendor** method for comparisons.

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
You can see how advantageous it is when you look back at the s390-tools example at the
[beginning](#cargo-vendor-home-registry) of this section.

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

Filtering is not supported in this method. See more in the [./cargo_vendor.service](./cargo_vendor.service) file or the
[Parameters](#parameters) section below.

# Parameters

The following are the parameters you can use with this utility:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<service name="cargo_vendor">
   <summary>OBS Source Service to vendor all crates.io and dependencies for a Rust project</summary>
   <description><![CDATA[This service extracts a Rust application source,
  searches for a Rust application containing a Cargo.toml file,
  download all crates.io and dependencies,
  and creates a vendor.tar[.<tar compression>] to be committed allowing fully offline
  builds of Rust applications.]]></description>
   <parameter name="strategy">
      <description>Legacy argument, no longer used. Values: cargo_vendor. Default: cargo_vendor</description>
   </parameter>
   <parameter name="method">
      <description>Whether to use vendor or the registry. Default: vendor</description>
      <allowedvalue>registry</allowedvalue>
      <allowedvalue>vendor</allowedvalue>
   </parameter>
   <parameter name="src">
      <description>Where to find sources. Source is either a directory or a source tarball AND cannot be both. Aliases: srctar, srcdir</description>
   </parameter>
   <parameter name="outdir">
      <description>Where to output vendor.tar* and cargo_config if method is vendor and registry.tar* if method is registry. If using with `osc service`, this option is automatically appended.</description>
   </parameter>
   <parameter name="custom root>
      <description>Whether you want to manually set the root of the
      project. Useful with a combination with `--manifest-path` (aliased as `--cargotoml`) or
      `--no-root-manifest`. You have to replace the spaces with dashes (-) to make this work.
      </description>
   </parameter>
   <parameter name="update">
      <description>Update dependencies or not. Default: true</description>
      <allowedvalue>false</allowedvalue>
      <allowedvalue>true</allowedvalue>
   </parameter>
   <parameter name="no root manifest">
      <description>Available only if `--method` is set to registry. If a
      project has no root manifest, this flag is useful for those situations
      to set the manifest path manually. Useful in combination with
      `--manifest-path` (aliased as `--cargotoml`) flag. You have to replace the spaces with dashes (-) to make this work. Default: false
      </description>
      <allowedvalue>false</allowedvalue>
      <allowedvalue>true</allowedvalue>
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
   <parameter name="i accept the risk">
      <description>A list of rustsec-id's to ignore. By setting this value, you acknowledge that this issue does not affect your package and you should be exempt from resolving it. You have to replace the spaces with dashes (-) to make this work.</description>
   </parameter>
   <parameter name="filter">
      <description>Available only if `--method` is set to vendor. EXPERIMENTAL: Reduce vendor-tarball size by filtering out non-Linux dependencies. Default: false</description>
      <allowedvalue>false</allowedvalue>
      <allowedvalue>true</allowedvalue>
   </parameter>
   <parameter name="versioned dirs">
      <description>Available only if `--method` is set to vendor. Whether to use the `--versioned-dirs` flag of cargo-vendor. You have to replace the spaces with dashes (-) to make this work. Default: true</description>
      <allowedvalue>false</allowedvalue>
      <allowedvalue>true</allowedvalue>
   </parameter>
</service>
```

# `cargotoml` behaviours

`cargotoml` is a flag/parameter that needs to be discussed as well. `cargotoml`
behaviour between **vendor** and **registry** methods differ.

## Vendor Method

For this method, `cargotoml` is used to pass over the `--sync` flag. If there are
no root manifest found but there is `cargotoml`, the first `cargotoml` is assumed
to be the "root manifest".

## Registry Method

For this method, `cargotoml` acts like an extra set of "root" manifests. It's
not passed over to `--sync` since under the hood, the registry method uses
`cargo-fetch`. This design decision is intentional for monorepo scenarios
where you have many crates that are standalone, regardless if they are a
dependency from each other or not, like the s390-tools or python-tokenizers.

If there is a case that you will do that, do experiment with the setting of
the `no-root-manifest` flag. A tip would be setting `no-root-manifest` to true
while having a lot of `cargotoml` declared.

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
