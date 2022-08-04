#!/usr/bin/python3

import subprocess
import argparse
import os
import xml.etree.ElementTree as ET

# Import Crimes
import importlib.machinery
import importlib.util
loader = importlib.machinery.SourceFileLoader( 'cargo_vendor_module', './cargo_vendor' )
spec = importlib.util.spec_from_loader( 'cargo_vendor_module', loader )
cargo_vendor_module = importlib.util.module_from_spec( spec )
loader.exec_module( cargo_vendor_module )


def do_services(pkgpath):
    cmd = [
        "nsjail",
        "--really_quiet",
        "--config", "scan.cfg",
        "--cwd", f"{os.getcwd()}/{pkgpath}",
        "--bindmount", f"{os.getcwd()}:{os.getcwd()}",
        "/usr/bin/osc", "service", "ra"
    ]
    try:
        out = subprocess.check_output(cmd, encoding='UTF-8', stderr=subprocess.STDOUT)
        print(f"✅ -- services passed")
        return True
    except subprocess.CalledProcessError as e:
        print(f"🚨 -- services failed")
        print(" ".join(cmd))
        print(e.stdout)
        return False


def checkout_or_update(pkgname, basepath):
    pkgpath = f"{basepath}:{pkgname}"
    try:
        if os.path.exists(pkgpath):
            print(f"osc revert {pkgpath}")
            # Revert/cleanup if required.
            out = subprocess.check_output(["osc", "revert", "."], cwd=f"{pkgpath}")
            print(f"osc clean {pkgpath}")
            out = subprocess.check_output(["osc", "clean", "."], cwd=f"{pkgpath}")
            print(f"osc up {pkgpath}")
            out = subprocess.check_output(["osc", "up", f"{pkgpath}"])
        else:
            print(f"osc bco {pkgname}")
            out = subprocess.check_output(["osc", "bco", f"{pkgname}"])
    except subprocess.CalledProcessError as e:
        print(f"Failed to checkout or update {pkgname}")
        print(e.stdout)
        raise e
    print(f"done")
    return pkgpath


def does_have_cargo_vendor(pkgpath):
    service = f"{pkgpath}/_service"
    has_vendor = False
    has_vendor_update = False
    srctar = None
    srcdir = None
    compression = None

    if os.path.exists(service):
        tree = ET.parse(service)
        root_node = tree.getroot()
        for tag in root_node.findall('service'):
            if tag.attrib['name'] == 'cargo_audit':
                root_node.remove(tag)
            if tag.attrib['name'] == 'cargo_vendor':
                has_vendor = True
                for attr in tag:
                    # if attr.attrib['name'] == 'update' and attr.text == 'true':
                    #     has_vendor_update = True
                    if attr.attrib['name'] == 'srctar':
                        srctar = attr.text
                    if attr.attrib['name'] == 'srcdir':
                        srcdir = attr.text
                    if attr.attrib['name'] == 'compression':
                        compression = attr.text
                # if not has_vendor_update:
                #     print("Forcing update to true in _service")
                #     sub = ET.SubElement(tag, 'param')
                #     sub.set('name', 'update')
                #     sub.text = 'true'
                root_node.remove(tag)

        # Rewrite the _service to include vendor_update = true
        # and to temporarily remove audit.
        tree.write(service)

    return (has_vendor, srctar, srcdir, compression)


def attempt_update(pkgpath, message):
    print("---")
    print(f"Attempting update for {pkgpath}")

    (has_vendor, srctar, srcdir, compression) = does_have_cargo_vendor(pkgpath)

    print(f"has_vendor: {has_vendor}, srctar: {srctar}, srcdir: {srcdir}")

    if not has_vendor:
        print(f"ERROR ⚠️ : {pkgpath} is not setup for cargo vendor!")
        return False

    print(f"Running services in {pkgpath}")
    if not do_services(pkgpath):
        print(f"Services reported a failure, this should be checked ...")
        # return False

    if srcdir and srctar is None:
        # We can use srcdir to have a guess what tar we need to use.
        content = os.listdir(pkgpath)
        maybe_src = [
            x for x in content
            if x.startswith(srcdir) and '.tar' in x and 'vendor' not in x and not x.endswith('.asc')
        ]
        if len(maybe_src) != 1:
            print(f"ERROR ⚠️ : confused! Not sure what tar to use in {pkgpath} {maybe_src}")
            print(f"This likely indicates a wacky package config that depends on services")
            return False

        srctar = maybe_src[0]

    srctar = f"{pkgpath}/{srctar}"

    print(f"Running vendor against {srctar} ...  ")
    try:
        vendor_tarfile = cargo_vendor_module.do_cargo_vendor(None, srctar, outdir=pkgpath, compression=compression)
    except Exception as e:
        print("ERROR %s" % e)
        vendor_tarfile = None

    if not vendor_tarfile:
        print(f"Failed to run cargo vendor 🥺 ")
        return False

    out = subprocess.check_output(["osc", "status"], cwd=f"{pkgpath}", encoding='UTF-8', stderr=subprocess.STDOUT)
    print(out)

    revert = []
    changed = False
    for line in out.split('\n'):
        if line.startswith('M'):
            if 'vendor' in line:
                changed = True
            elif 'cargo_config' in line:
                # skipp, we don't want to revert this
                pass
            else:
                print("Reverting %s" % line.split()[1].strip())
                revert.append(line.split()[1].strip())

    for item in revert:
        subprocess.check_output(["osc", "revert", item], cwd=f"{pkgpath}")

    return changed


def attempt_submit(pkgpath, message, yolo=False):
    try:
        print("---")
        if not yolo:
            print("You must manually run: ")

        print(f"osc vc -m '{message}' {pkgpath}")
        if yolo:
            out = subprocess.check_output(["osc", "vc", "-m", message], cwd=f"{pkgpath}")

        print(f"osc ci -m '{message}' {pkgpath}")
        if yolo:
            out = subprocess.check_output(["osc", "ci", "-m", message], cwd=f"{pkgpath}")

        print(f"osc sr -m '{message}' {pkgpath}")
        if yolo:
            # out = subprocess.check_output(["osc", "sr", "-m", message], cwd=f"{pkgpath}")
            pass

        return f"osc sr -m '{message}' {pkgpath}"

    except Exception as e:
        print("ERROR %s" % e)
        return None


if __name__ == '__main__':
    print("Started OBS cargo vendor bulk updater ...")

    basepath = "home:firstyear:branches"

    parser = argparse.ArgumentParser(
        description="update OBS gooderer",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument('--yolo', action='store_true')
    parser.add_argument('--message', nargs='?', default="Automatic update of vendored dependencies")
    parser.add_argument('packages', nargs='+')
    args = parser.parse_args()

    print(args)

    pkgpaths = [checkout_or_update(pkgname, basepath) for pkgname in args.packages]

    submit_req = [attempt_update(pkgpath, args.message) for pkgpath in pkgpaths]
    submited_req = [attempt_submit(pkgpath, args.message, args.yolo) for pkgpath in pkgpaths]

    print("--- complete")

    for sr in sorted(submited_req):
        print(sr)

