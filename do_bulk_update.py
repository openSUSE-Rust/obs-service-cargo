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
            if tag.attrib['name'] == 'cargo_vendor':
                has_vendor = True
                for attr in tag:
                    if attr.attrib['name'] == 'update' and attr.text == 'true':
                        has_vendor_update = True
                    if attr.attrib['name'] == 'srctar':
                        srctar = attr.text
                    if attr.attrib['name'] == 'srcdir':
                        srcdir = attr.text
                    if attr.attrib['name'] == 'compression':
                        compression = attr.text
                if not has_vendor_update:
                    print("Forcing update to true in _service")
                    sub = ET.SubElement(tag, 'param')
                    sub.set('name', 'update')
                    sub.text = 'true'

        # Rewrite the _service to include vendor_update = true
        if not has_vendor_update:
            tree.write(service)

    return (has_vendor, srctar, srcdir, compression)

def attempt_update(pkgpath, message, yolo=False):
    print(f"Attempting update for {pkgpath}")

    (has_vendor, srctar, srcdir, compression) = does_have_cargo_vendor(pkgpath)

    print(f"has_vendor: {has_vendor}, srctar: {srctar}, srcdir: {srcdir}")

    if not has_vendor:
        print(f"ERROR ⚠️ : {pkgpath} is not setup for cargo vendor!")
        return

    # if srcdir and srctar is None:
    #     # We can use srcdir to have a guess what tar we need to use.
    #     content = os.listdir(pkgpath)
    #     maybe_src = [
    #         x for x in content
    #         if x.startswith(srcdir) and '.tar' in x and 'vendor' not in x and not x.endswith('.asc')
    #     ]
    #     if len(maybe_src) != 1:
    #         print(f"ERROR ⚠️ : confused! Not sure what tar to use in {pkgpath} {maybe_src}")
    #         print(f"This likely indicates a wacky package config that depends on services")
    #         return

    #     srctar = maybe_src[0]

    # assert srctar

    srctar = f"{pkgpath}/{srctar}"

    print(f"Running services in {pkgpath}")
    if not do_services(pkgpath):
        return

    # print(f"Running vendor in {pkgpath} ...")
    # vendor_tarfile = cargo_vendor_module.do_cargo_vendor(None, srctar, pkgpath, True, compression)
    #
    # if not vendor_tarfile:
    #     print(f"Failed to run cargo vendor 🥺 ")
    #     return

    out = subprocess.check_output(["osc", "status"], cwd=f"{pkgpath}", encoding='UTF-8', stderr=subprocess.STDOUT)
    print(out)
    changed = False
    for line in out.split('\n'):
        if line.startswith('M') and 'vendor' in line:
            changed = True

    if changed:
        if not yolo:
            print("You must manually run: ")
        print(f"osc vc {pkgpath}")
        if yolo:
            out = subprocess.check_output(["osc", "vc", "-m", message], cwd=f"{pkgpath}")

        print(f"osc ci {pkgpath}")
        if yolo:
            out = subprocess.check_output(["osc", "ci", "-m", message], cwd=f"{pkgpath}")

        print(f"osc sr {pkgpath}")
        if yolo:
            out = subprocess.check_output(["osc", "sr", "-m", message], cwd=f"{pkgpath}")
    else:
        print("No changes detected, skipping submit phase")

    print(f"Complete!")


if __name__ == '__main__':
    print("Started OBS cargo vendor bulk updater ...")

    basepath = "home:firstyear:branches"

    parser = argparse.ArgumentParser(
        description="update OBS gooderer",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument('--yolo', action='store_true')
    parser.add_argument('packages', nargs='+')
    parser.add_argument('message', nargs='?', default="Automatic update of vendored dependencies")
    args = parser.parse_args()

    print(args)

    pkgpaths = [checkout_or_update(pkgname, basepath) for pkgname in args.packages]

    for pkgpath in pkgpaths:
        print("---")
        attempt_update(pkgpath, args.message, args.yolo)

    for pkgpath in pkgpaths:
        print(f"echo {pkgpath}")
        print(f"osc results {pkgpath}")

    print("--- complete")

