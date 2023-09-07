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
     <param name="srctar">zellij-0.37.2.tar.gz</param>
     <param name="compression">zst</param>
     <param name="update">true</param>
  </service>
  <service name="cargo_audit" mode="manual" />
</services>
```

> [!WARNING]
> However, certain projects may not have a root manifest file, thus, each directory may be a separate subproject e.g. https://github.com/ibm-s390-linux/s390-tools and may need some thinking.
> If projects like these cannot have a root manifest but have different subprojects, you may need to define the relative path to the other manifest files from root.
> 
> ```xml
> <services>
>   <service name="cargo_vendor" mode="manual">
>      <param name="srcdir">s390-tools</param>
>      <param name="compression">zst</param>
>      <param name="cargotoml">s390-tools/rust/utils/Cargo.toml</param>
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

> [!NOTE]
> Some Rust software such as the infamous https://github.com/elliot40404/bonk do not have any dependencies so they may not generate a vendored tarball.
> The service will give you an output of information about it by checking the manifest file.

# Limitations

There may be some corner/edge (whatever-you-call-it) cases that will not work with **OBS Service Cargo**. Please open a bug report at https://github.com/openSUSE/obs-service-cargo_vendor/issues.
We will try to investigate those in the best of our abilities. In the mean time, this will work, *hopefully*, in most projects.